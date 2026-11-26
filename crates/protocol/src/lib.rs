//! Wire protocol shared by the relay, the agent and the simulator.
//!
//! Every frame is one JSON object tagged by `t`. JSON keeps the browser
//! dashboard trivial; a full player state is ~1-2 KB, so at Dota's GSI rate
//! (at most ~20 updates/s per player) bandwidth is never the bottleneck --
//! network round-trip time is.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub const PROTOCOL_VERSION: u32 = 1;

/// Milliseconds since the Unix epoch.
pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Item {
    /// Internal name without the `item_` prefix, e.g. `black_king_bar`.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<u32>,
    pub cooldown: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Ability {
    pub name: String,
    pub level: u32,
    pub cooldown: u32,
    pub ultimate: bool,
    pub can_cast: bool,
    pub passive: bool,
}

/// One enemy hero, as read by the in-game bridge.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct EnemyHero {
    /// Internal hero name without `npc_dota_hero_`.
    pub hero: String,
    pub level: u32,
    pub alive: bool,
    pub health: u32,
    pub max_health: u32,
    pub x: f32,
    pub y: f32,
    /// Fight-changing items on cooldown or ready (BKB, Blink, Glimmer…).
    pub items: Vec<Item>,
    /// Ultimate seconds remaining; 0 = ready, -1 = not yet learned.
    pub ult_cooldown: i32,
    pub magic_immune: bool,
    pub smoked: bool,
    pub stunned: bool,
}

/// An enemy ward we can see through the helper.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct EnemyWard {
    /// "observer" or "sentry".
    pub kind: String,
    pub x: f32,
    pub y: f32,
    /// Seconds since it was placed (the helper's clock).
    pub age: u32,
}

/// Roshan and the neutral objective, as the helper sees them.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct RoshanInfo {
    pub alive: bool,
    pub health: u32,
    pub max_health: u32,
    pub x: f32,
    pub y: f32,
    /// Seconds since last death (0 = never died this game).
    pub last_death: u32,
}

/// A rune on the map.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Rune {
    /// "double" | "aegis" | "blessing" | "haste" | "regeneration" | "vision".
    pub kind: String,
    pub x: f32,
    pub y: f32,
    /// Seconds until it spawns (0 = on the ground right now).
    pub available_in: u32,
}

/// The courier.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Courier {
    pub x: f32,
    pub y: f32,
    /// "idle" | "moving" | "stashed" | "dead".
    pub state: String,
}

/// Everything the in-game bridge can read about the enemy team and the map.
/// Sent by one player per game (the one whose bridge is running) and shared
/// with the whole team through the relay, so everyone gets the same picture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct EnemyInfo {
    /// Game clock seconds when this snapshot was taken.
    pub clock: i32,
    pub heroes: Vec<EnemyHero>,
    pub wards: Vec<EnemyWard>,
    pub roshan: RoshanInfo,
    pub runes: Vec<Rune>,
    pub courier: Courier,
    /// Wall-clock ms since epoch when the helper captured this.
    pub captured_ms: u64,
}

/// What one player's own Game State Integration reports about themself.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct PlayerState {
    /// Internal hero name without `npc_dota_hero_`, empty when not in a match.
    pub hero: String,
    pub level: u32,
    pub alive: bool,
    pub respawn_seconds: u32,
    pub health: u32,
    pub max_health: u32,
    pub mana: u32,
    pub max_mana: u32,
    pub buyback_cost: u32,
    pub buyback_cooldown: u32,
    pub gold: u32,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub last_hits: u32,
    pub denies: u32,
    pub gpm: u32,
    pub xpm: u32,
    pub has_pos: bool,
    pub x: f32,
    pub y: f32,
    pub items: Vec<Item>,
    pub backpack: Vec<Item>,
    pub neutral: Option<Item>,
    pub tp: Option<Item>,
    pub abilities: Vec<Ability>,
    pub smoked: bool,
    pub stunned: bool,
    pub silenced: bool,
    pub hexed: bool,
    pub magic_immune: bool,
    pub team: String,
    pub clock: i32,
    pub game_state: String,
    pub daytime: bool,
    /// Dota's match id; every teammate reports the same one, which is how the
    /// relay groups a game for the post-game report.
    pub match_id: String,
    /// "radiant" / "dire" once the game is decided, else empty.
    pub win_team: String,
    /// Unreliable gold is what a death costs you.
    pub gold_reliable: u32,
    pub gold_unreliable: u32,
    pub kill_streak: u32,
    pub stash: Vec<Item>,
}

/// Agent build version, shown on the site and used for update checks.
pub const AGENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const GAME_IN_PROGRESS: &str = "DOTA_GAMERULES_STATE_GAME_IN_PROGRESS";
pub const GAME_PRE_GAME: &str = "DOTA_GAMERULES_STATE_PRE_GAME";
pub const GAME_POST_GAME: &str = "DOTA_GAMERULES_STATE_POST_GAME";

impl PlayerState {
    pub fn hp_frac(&self) -> f32 {
        if self.max_health == 0 {
            0.0
        } else {
            self.health as f32 / self.max_health as f32
        }
    }
    pub fn mana_frac(&self) -> f32 {
        if self.max_mana == 0 {
            0.0
        } else {
            self.mana as f32 / self.max_mana as f32
        }
    }
    pub fn in_match(&self) -> bool {
        !self.hero.is_empty()
    }
    pub fn ultimate(&self) -> Option<&Ability> {
        self.abilities.iter().find(|a| a.ultimate)
    }
    pub fn has_item(&self, name: &str) -> bool {
        self.items.iter().chain(self.backpack.iter()).any(|i| i.name == name)
    }
    pub fn item(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|i| i.name == name)
    }
    /// Seconds until TP is ready; 0 = ready, None = no TP scroll.
    pub fn tp_cooldown(&self) -> Option<u32> {
        self.tp.as_ref().map(|t| t.cooldown)
    }
    pub fn ward_count(&self) -> (u32, u32) {
        let mut obs = 0;
        let mut sen = 0;
        for i in self.items.iter().chain(self.backpack.iter()) {
            match i.name.as_str() {
                "ward_observer" => obs += i.charges.unwrap_or(1),
                "ward_sentry" => sen += i.charges.unwrap_or(1),
                "ward_dispenser" => {
                    // Combined stack: GSI reports observer charges only.
                    obs += i.charges.unwrap_or(1);
                }
                _ => {}
            }
        }
        (obs, sen)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Player,
    Viewer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CalloutKind {
    /// "I'm being attacked, come help."
    Help,
    /// "Group up on me."
    Group,
    /// "Back off / retreat."
    Retreat,
    /// "Enemies missing, be careful."
    Careful,
    /// "Coming to you."
    OnMyWay,
    /// "Let's take Roshan."
    Roshan,
}

impl CalloutKind {
    pub fn label(self) -> &'static str {
        match self {
            CalloutKind::Help => "NEEDS HELP",
            CalloutKind::Group => "GROUP UP",
            CalloutKind::Retreat => "RETREAT",
            CalloutKind::Careful => "CAREFUL",
            CalloutKind::OnMyWay => "ON MY WAY",
            CalloutKind::Roshan => "ROSHAN",
        }
    }
}

/// Optional coaching aids, chosen per player by the team host. Only these
/// neutral feature switches are sent, and only to the player they belong to;
/// the host's reasons for choosing them never leave the host.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    /// Warn when you are far from every teammate mid-game.
    IsolationWarning,
    /// Big "team is fighting / retreating" banner, and a warning when you
    /// move the other way.
    TeamModeBanner,
    /// During fights, hide everything but alerts and instructions.
    FightFocus,
    /// Tell you plainly when the team is fighting without you.
    JoinFightNudge,
    /// Reminders to buy and place observer wards.
    WardReminders,
    /// Points out short, winnable fight windows and when a fight drags on.
    EngageAdvisor,
    /// "Low HP, back off" when you are hurt in a fight.
    LowHpRetreat,
    /// Late game: warns when you spend below buyback gold.
    BuybackGuard,
    /// Sent by a newer relay; ignored by this agent.
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Coaching {
    pub features: Vec<Feature>,
    /// Alert text size multiplier; 0 means 1.0.
    pub alert_scale: f32,
}

impl Coaching {
    pub fn has(&self, f: Feature) -> bool {
        self.features.contains(&f)
    }
    pub fn scale(&self) -> f32 {
        if self.alert_scale > 0.0 {
            self.alert_scale.clamp(0.8, 1.6)
        } else {
            1.0
        }
    }
}

/// One player's post-game card: a few plain facts, the worst moment and the
/// coach's line for them. Everything else is in the full report on the site.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct PostGameCard {
    pub match_id: String,
    pub won: Option<bool>,
    /// Game length, seconds.
    pub duration: i32,
    /// "6 team fights: 4 won, 1 lost."
    pub team_line: String,
    /// (tone, text): tone is "good", "bad" or "neutral"; worst first.
    pub facts: Vec<(String, String)>,
    /// "18:20 Died alone, holding 2,400 gold"
    pub worst: Option<String>,
    pub best: Option<String>,
    /// The AI coach's line for this player, once written.
    pub coach: Option<String>,
    pub lesson: Option<String>,
    /// True while the coach is still writing (the card updates itself).
    pub coach_pending: bool,
    pub replay_facts: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MemberInfo {
    pub id: u32,
    pub name: String,
    pub role: Role,
    pub state: Option<PlayerState>,
}

/// Client -> relay.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum ClientMsg {
    Hello {
        key: String,
        name: String,
        role: Role,
        version: u32,
    },
    State {
        state: PlayerState,
        /// Enemy intel from the in-game bridge (empty when the bridge is off).
        #[serde(default)]
        enemy: Option<EnemyInfo>,
    },
    Callout {
        kind: CalloutKind,
        x: f32,
        y: f32,
    },
    /// Free-text tip, optionally addressed to one member id.
    Note {
        to: Option<u32>,
        text: String,
    },
    Ping {
        id: u64,
    },
}

/// Relay -> client.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum ServerMsg {
    Welcome {
        you: u32,
        server_ms: u64,
        members: Vec<MemberInfo>,
        /// This player's own coaching switches (empty when the host set none).
        #[serde(default)]
        coaching: Coaching,
        /// Newest agent on the download page, e.g. "0.2.0" (empty if unknown).
        #[serde(default)]
        latest_agent: String,
    },
    Joined {
        member: MemberInfo,
    },
    Left {
        id: u32,
    },
    State {
        id: u32,
        server_ms: u64,
        state: PlayerState,
        /// Enemy intel shared by the player whose in-game bridge is running.
        #[serde(default)]
        enemy: Option<EnemyInfo>,
    },
    Callout {
        id: u32,
        server_ms: u64,
        kind: CalloutKind,
        x: f32,
        y: f32,
    },
    Note {
        id: u32,
        server_ms: u64,
        to: Option<u32>,
        text: String,
    },
    Pong {
        id: u64,
        server_ms: u64,
    },
    Error {
        message: String,
    },
    /// Your own short report after a game. Sent when the report is ready and
    /// again when the coach's lesson or the replay facts arrive.
    PostGame {
        card: PostGameCard,
    },
}

/// `npc_dota_hero_x` -> `x`.
pub fn hero_short(name: &str) -> &str {
    name.strip_prefix("npc_dota_hero_").unwrap_or(name)
}

/// Human-readable hero name from the internal short name.
pub fn hero_display(short: &str) -> String {
    let special = match short {
        "antimage" => Some("Anti-Mage"),
        "nevermore" => Some("Shadow Fiend"),
        "zuus" => Some("Zeus"),
        "obsidian_destroyer" => Some("Outworld Destroyer"),
        "wisp" => Some("Io"),
        "magnataur" => Some("Magnus"),
        "furion" => Some("Nature's Prophet"),
        "skeleton_king" => Some("Wraith King"),
        "rattletrap" => Some("Clockwerk"),
        "shredder" => Some("Timbersaw"),
        "treant" => Some("Treant Protector"),
        "windrunner" => Some("Windranger"),
        "vengefulspirit" => Some("Vengeful Spirit"),
        "queenofpain" => Some("Queen of Pain"),
        "doom_bringer" => Some("Doom"),
        "centaur" => Some("Centaur Warrunner"),
        "life_stealer" => Some("Lifestealer"),
        "abyssal_underlord" => Some("Underlord"),
        "necrolyte" => Some("Necrophos"),
        "keeper_of_the_light" => Some("Keeper of the Light"),
        "spirit_breaker" => Some("Spirit Breaker"),
        "monkey_king" => Some("Monkey King"),
        _ => None,
    };
    if let Some(s) = special {
        return s.to_string();
    }
    short
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Short label for an item so a row of six fits in the overlay.
pub fn item_short(name: &str) -> String {
    let s = match name {
        "black_king_bar" => "BKB",
        "blink" => "Blink",
        "overwhelming_blink" => "OBlink",
        "swift_blink" => "SBlink",
        "arcane_blink" => "ABlink",
        "ultimate_scepter" => "Aghs",
        "aghanims_shard" => "Shard",
        "glimmer_cape" => "Glimmer",
        "force_staff" => "Force",
        "hurricane_pike" => "Pike",
        "sheepstick" => "Hex",
        "cyclone" => "Eul",
        "wind_waker" => "Waker",
        "lotus_orb" => "Lotus",
        "pipe" => "Pipe",
        "mekansm" => "Mek",
        "guardian_greaves" => "Greaves",
        "magic_wand" => "Wand",
        "magic_stick" => "Stick",
        "power_treads" => "Treads",
        "phase_boots" => "Phase",
        "arcane_boots" => "Arcane",
        "tranquil_boots" => "Tranq",
        "travel_boots" | "travel_boots_2" => "BoT",
        "boots" => "Boots",
        "manta" => "Manta",
        "refresher" => "Refresher",
        "aeon_disk" => "Aeon",
        "sphere" => "Linken",
        "silver_edge" => "SilverE",
        "invis_sword" => "Shadow",
        "monkey_king_bar" => "MKB",
        "greater_crit" => "Daedalus",
        "bfury" => "BFury",
        "desolator" => "Deso",
        "skadi" => "Skadi",
        "assault" => "AC",
        "shivas_guard" => "Shiva",
        "heart" => "Heart",
        "satanic" => "Satanic",
        "butterfly" => "Fly",
        "abyssal_blade" => "Abyssal",
        "basher" => "Basher",
        "orchid" => "Orchid",
        "bloodthorn" => "Bloodthorn",
        "nullifier" => "Nullifier",
        "radiance" => "Radiance",
        "echo_sabre" => "Echo",
        "harpoon" => "Harpoon",
        "spirit_vessel" => "Vessel",
        "urn_of_shadows" => "Urn",
        "solar_crest" => "Crest",
        "ward_observer" => "Obs",
        "ward_sentry" => "Sentry",
        "ward_dispenser" => "Wards",
        "dust" => "Dust",
        "smoke_of_deceit" => "Smoke",
        "tpscroll" => "TP",
        "tango" => "Tango",
        "flask" => "Salve",
        "clarity" => "Clarity",
        "bottle" => "Bottle",
        "gem" => "Gem",
        "rapier" => "Rapier",
        "cheese" => "Cheese",
        "aegis" => "AEGIS",
        "blade_mail" => "BMail",
        "vanguard" => "Vanguard",
        "crimson_guard" => "Crimson",
        "heavens_halberd" => "Halberd",
        "eternal_shroud" => "Shroud",
        "holy_locket" => "Locket",
        "octarine_core" => "Octarine",
        "aether_lens" => "Lens",
        "dragon_lance" => "Lance",
        "mask_of_madness" => "MoM",
        "armlet" => "Armlet",
        "maelstrom" => "Maelstrom",
        "mjollnir" => "Mjoll",
        "gungir" => "Gleipnir",
        "bloodstone" => "Bloodstone",
        "helm_of_the_overlord" => "Overlord",
        "helm_of_the_dominator" => "Dominator",
        "orb_of_corrosion" => "Corrosion",
        "yasha" => "Yasha",
        "sange_and_yasha" => "S&Y",
        "kaya_and_sange" => "K&S",
        "yasha_and_kaya" => "Y&K",
        "disperser" => "Disperser",
        "ethereal_blade" => "Ethereal",
        "dagon" | "dagon_2" | "dagon_3" | "dagon_4" | "dagon_5" => "Dagon",
        _ => "",
    };
    if !s.is_empty() {
        return s.to_string();
    }
    let disp = hero_display(name);
    disp.chars().take(9).collect()
}

/// Items worth highlighting on teammates because they change fights.
pub fn is_key_item(name: &str) -> bool {
    matches!(
        name,
        "black_king_bar"
            | "blink"
            | "overwhelming_blink"
            | "swift_blink"
            | "arcane_blink"
            | "glimmer_cape"
            | "force_staff"
            | "hurricane_pike"
            | "sheepstick"
            | "cyclone"
            | "wind_waker"
            | "lotus_orb"
            | "pipe"
            | "mekansm"
            | "guardian_greaves"
            | "refresher"
            | "aeon_disk"
            | "sphere"
            | "silver_edge"
            | "invis_sword"
            | "abyssal_blade"
            | "bloodthorn"
            | "orchid"
            | "nullifier"
            | "smoke_of_deceit"
            | "gem"
            | "aegis"
            | "cheese"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_messages() {
        let msg = ClientMsg::Callout { kind: CalloutKind::Help, x: 1.0, y: -2.0 };
        let s = serde_json::to_string(&msg).unwrap();
        assert!(s.contains("\"t\":\"callout\""));
        assert_eq!(serde_json::from_str::<ClientMsg>(&s).unwrap(), msg);

        let st = PlayerState { hero: "axe".into(), level: 7, ..Default::default() };
        let m = ServerMsg::State { id: 3, server_ms: 9, state: st, enemy: None };
        let s = serde_json::to_string(&m).unwrap();
        assert_eq!(serde_json::from_str::<ServerMsg>(&s).unwrap(), m);
    }

    #[test]
    fn lenient_state_parse() {
        // Missing fields default instead of failing: old and new agents interoperate.
        let st: PlayerState = serde_json::from_str(r#"{"hero":"lina","future_field":1}"#).unwrap();
        assert_eq!(st.hero, "lina");
        assert_eq!(st.level, 0);
    }

    #[test]
    fn coaching_is_forward_compatible() {
        let c: Coaching = serde_json::from_str(r#"{"features":["fight_focus","from_the_future"]}"#).unwrap();
        assert_eq!(c.features, vec![Feature::FightFocus, Feature::Unknown]);
        assert_eq!(c.scale(), 1.0);
        // An old relay's Welcome without coaching still parses.
        let w: ServerMsg = serde_json::from_str(r#"{"t":"welcome","you":1,"server_ms":0,"members":[]}"#).unwrap();
        assert!(matches!(w, ServerMsg::Welcome { coaching, .. } if coaching.features.is_empty()));
    }

    #[test]
    fn names() {
        assert_eq!(hero_short("npc_dota_hero_nevermore"), "nevermore");
        assert_eq!(hero_display("nevermore"), "Shadow Fiend");
        assert_eq!(hero_display("crystal_maiden"), "Crystal Maiden");
        assert_eq!(item_short("black_king_bar"), "BKB");
    }
}
