# cyberspyde dota 2

A team coordination tool for a 5-stack. Each player shares **their own** game state with the rest of the team in real time. Everyone gets an in-game overlay showing:

- **Who is under attack, right now.** It flags burst damage (12%+ HP lost within 2.5 s) and shows your distance to that teammate and whether your TP is ready.
- **Teammate status**: HP and mana, dead or alive with respawn time and buyback status, ultimate and TP cooldowns, key items (BKB, Blink, Glimmer and so on, with cooldowns), and ward stock.
- **A radar** showing where everyone is, with a pulsing ring on anyone taking damage.
- **Callouts on hotkeys**: help, group up, retreat, careful, on my way, Roshan.
- **A build helper for your hero**: next item and gold still needed, skill-order reminders, game-plan tips. It comes from the old CYBERSPYDE build database, plus builds your pro friend writes **once** instead of explaining them every game.
- **A team board** web page (for a second monitor or a phone) where anyone can send tips to one player or the whole team.

## Fair play

The tool uses only Valve's official **Game State Integration (GSI)**. In player mode, Dota sends each player only their own hero, items and abilities. Sharing that with your own team is the same as reading it out on voice chat, just faster and without the shouting. It never reads game memory or injects anything, and it shows **nothing about enemies**. Spectator/observer feeds are explicitly ignored (`gsi.rs`).

## How it works

```
Dota 2 ──GSI (HTTP POST, localhost, ≤50 ms)──▶ cyberspyde-agent ──WebSocket──▶ cyberspyde-relay ──▶ 4 teammates' agents
                                                    │                                    └──▶ team board (browser)
                                                    └─ overlay (transparent, click-through, always on top)
```

| Crate | Binary | What it does |
|---|---|---|
| `crates/protocol` | – | Shared JSON message types, hero/item names |
| `crates/relay` | `cyberspyde-relay` | Team room: fans each player's state out to everyone else; serves the team board |
| `crates/agent` | `cyberspyde-agent` | Runs on each player's PC: GSI receiver, relay client, overlay, build helper |
| `crates/sim` | `cyberspyde-sim` | Latency and load test with N fake players |
| `legacy-helper/` | – | The old CYBERSPYDE Dota helper (React + Netlify + Supabase), imported from [github.com/cyberspyde/dota](https://github.com/cyberspyde/dota) at commit `9c92436`. Its `build_data_final.json` is compiled into the agent |

## Speed

Every step is event-driven: nothing is batched and nothing runs on a fixed tick.

1. Dota posts within 50 ms of a change (`throttle 0.05`, `buffer 0`).
2. The agent forwards the newest state immediately. If updates pile up while a send is in flight, only the latest goes.
3. The relay serializes each update once and pushes it to every member's queue. Sockets use `TCP_NODELAY`.
4. The receiving overlay repaints as soon as the frame lands.

Measured on one PC (`cyberspyde-sim`, release build):

| Load | Relay overhead p50 / p99 |
|---|---|
| 5 players × 20 updates/s | 0.22 ms / 0.60 ms |
| 10 players × 50 updates/s | 0.31 ms / 0.46 ms |

So the delay is almost all **network distance**: one-way time from player A to the relay, plus from the relay to player B. The overlay header shows your live round-trip time to the relay. To keep it low when you're in different countries, **host the relay where it's roughly equidistant**. A small VPS in a central city is better than one player's home connection. Dota's own servers are in the same region, so this is usually 20–80 ms, well under the time it takes to die.

## Building

Rust with the GNU toolchain, plus MinGW-w64 (WinLibs) for the linker. No Visual Studio needed.

1. Install rustup with the `x86_64-pc-windows-gnu` host.
2. Unpack WinLibs MinGW-w64 (UCRT, POSIX threads, x86_64) to `%USERPROFILE%\mingw64`.
3. Build:

```powershell
.\build.ps1 -Test
```

Binaries go to `dist\`. Only `cyberspyde-agent.exe` goes to your friends; nothing else needs installing on their PCs.

## Running

### Host (you): the relay

```powershell
.\dist\cyberspyde-relay.exe --bind 0.0.0.0:4747 --key choose-a-long-team-key
```

Open TCP port 4747 on that machine (and forward it on the router, if it's at home). The team board is at `http://<host>:4747/#key=<team-key>`.

### Every player: the agent

1. Run `cyberspyde-agent.exe` once. It creates `agent.toml` next to itself.
2. Edit `agent.toml`: set `name`, `relay = "ws://<host>:4747/ws"` and `key`.
3. Run it again. It finds Dota through Steam and writes `game/dota/cfg/gamestate_integration/gamestate_integration_cyberspyde.cfg`.
4. In Steam, add `-gamestateintegration` to Dota 2's launch options, then restart Dota.
5. In Dota, set Display Mode to **Borderless window**. The overlay can't draw over exclusive fullscreen.

Try it without Dota: `cyberspyde-agent.exe --demo` (four fake teammates; one keeps getting ganked).

### Hotkeys (change them in `agent.toml`)

| Key | Action |
|---|---|
| Numpad 1 | Need help |
| Numpad 2 | Group up |
| Numpad 3 | Retreat |
| Numpad 4 | Careful, enemies missing |
| Numpad 5 | On my way |
| Numpad 6 | Roshan |
| Numpad 0 | Move mode: drag the overlay, press again to save |
| Numpad . | Hide or show the overlay |

## Builds from your pro friend

Put a `builds.override.json` next to the agent; see `examples/builds.override.example.json`. Entries there win over the legacy database. They can include a `skills` order (`Q W E R`, `T` for a talent); the overlay highlights the pick for your current level. Share the file with the team once, and nobody needs to ask "what do I build?" mid-game again.

The legacy database covers 62 heroes, alphabetically from Abaddon to Monkey King. Heroes after that need override entries.

## Security notes

- The team key is checked by the relay. Traffic is plain `ws://`: game stats aren't secret, but don't reuse a real password as the key. For `wss://`, put the relay behind a TLS reverse proxy such as Caddy.
- The GSI receiver listens only on `127.0.0.1` and requires the random token written into Dota's cfg.

## Not yet built

- Counter-pick suggestions: player-mode GSI doesn't include the enemy draft. Next step: type enemy heroes on the team board and pull matchup data from OpenDota.
- Item icons: the overlay uses short text labels today.
