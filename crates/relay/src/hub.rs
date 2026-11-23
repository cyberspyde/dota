use axum::extract::ws::Utf8Bytes;
use protocol::{MemberInfo, PlayerState, Role, ServerMsg};
use std::{collections::HashMap, sync::Mutex};
use tokio::sync::{mpsc, oneshot};
use tracing::warn;

struct Member {
    name: String,
    role: Role,
    state: Option<PlayerState>,
    tx: mpsc::Sender<Utf8Bytes>,
    kick: Option<oneshot::Sender<()>>,
}

struct Inner {
    next_id: u32,
    members: HashMap<u32, Member>,
}

/// The team room. A plain mutex is enough: every critical section is a few
/// map operations and non-blocking `try_send`s, never an await.
pub struct Hub {
    key: String,
    inner: Mutex<Inner>,
}

impl Hub {
    pub fn new(key: String) -> Self {
        Hub { key, inner: Mutex::new(Inner { next_id: 1, members: HashMap::new() }) }
    }

    pub fn check_key(&self, key: &str) -> bool {
        // Length check plus full-length comparison, so timing reveals little.
        let a = self.key.as_bytes();
        let b = key.as_bytes();
        if a.len() != b.len() {
            return false;
        }
        a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
    }

    /// Adds a member and returns its id plus a snapshot of everyone else.
    /// A player reconnecting under the same name replaces the stale entry.
    pub fn join(
        &self,
        name: String,
        role: Role,
        tx: mpsc::Sender<Utf8Bytes>,
        kick: oneshot::Sender<()>,
    ) -> (u32, Vec<MemberInfo>) {
        let mut g = self.inner.lock().unwrap();
        let mut replaced = Vec::new();
        if role == Role::Player {
            let stale: Vec<u32> = g
                .members
                .iter()
                .filter(|(_, m)| m.role == Role::Player && m.name == name)
                .map(|(id, _)| *id)
                .collect();
            for old in stale {
                if let Some(mut m) = g.members.remove(&old) {
                    if let Some(k) = m.kick.take() {
                        let _ = k.send(());
                    }
                    replaced.push(old);
                }
            }
        }
        let id = g.next_id;
        g.next_id += 1;
        let snapshot = g
            .members
            .iter()
            .map(|(id, m)| MemberInfo { id: *id, name: m.name.clone(), role: m.role, state: m.state.clone() })
            .collect();
        g.members.insert(id, Member { name: name.clone(), role, state: None, tx, kick: Some(kick) });

        for old in replaced {
            Self::broadcast_locked(&g, &enc(&ServerMsg::Left { id: old }), Some(id));
        }
        let joined = enc(&ServerMsg::Joined { member: MemberInfo { id, name, role, state: None } });
        Self::broadcast_locked(&g, &joined, Some(id));
        (id, snapshot)
    }

    pub fn leave(&self, id: u32) {
        let mut g = self.inner.lock().unwrap();
        if g.members.remove(&id).is_some() {
            Self::broadcast_locked(&g, &enc(&ServerMsg::Left { id }), None);
        }
    }

    pub fn set_state(&self, id: u32, state: PlayerState) {
        let mut g = self.inner.lock().unwrap();
        if let Some(m) = g.members.get_mut(&id) {
            m.state = Some(state);
        }
    }

    pub fn broadcast(&self, frame: &Utf8Bytes, except: Option<u32>) {
        let g = self.inner.lock().unwrap();
        // Ignore frames from a connection that has already been replaced.
        if let Some(sender) = except {
            if !g.members.contains_key(&sender) {
                return;
            }
        }
        Self::broadcast_locked(&g, frame, except);
    }

    fn broadcast_locked(g: &Inner, frame: &Utf8Bytes, except: Option<u32>) {
        for (id, m) in &g.members {
            if Some(*id) == except {
                continue;
            }
            // Utf8Bytes clones are reference-counted, not copies.
            if let Err(mpsc::error::TrySendError::Full(_)) = m.tx.try_send(frame.clone()) {
                // State is latest-wins; a member this far behind will catch up
                // on the next update instead of us blocking everyone else.
                warn!("#{id} {} is not keeping up; dropped a frame", m.name);
            }
        }
    }
}

fn enc(msg: &ServerMsg) -> Utf8Bytes {
    serde_json::to_string(msg).expect("serialize").into()
}
