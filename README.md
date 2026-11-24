# cyberspyde dota 2

A team coordination tool for a 5-stack. Each player shares **their own** game state with the rest of the team in real time. Everyone gets an in-game overlay showing:

- **Who is under attack, right now.** It flags burst damage (12%+ HP lost within 2.5 s) and shows your distance to that teammate and whether your TP is ready.
- **Teammate status**: HP and mana, dead or alive with respawn time and buyback status, ultimate and TP cooldowns, key items (BKB, Blink, Glimmer and so on, with cooldowns), and ward stock.
- **A radar** showing where everyone is, with a pulsing ring on anyone taking damage.
- **Callouts on hotkeys**: help, group up, retreat, careful, on my way, Roshan.
- **A build helper for your hero**: next item and gold still needed, skill-order reminders, game-plan tips. It comes from the old CYBERSPYDE build database, plus builds your pro friend writes **once** instead of explaining them every game.
- **A team summary line**: how many ultimates and TPs are ready right now, who is dead, observer wards on the team.
- **An in-game menu** (Numpad *): name, key, server, five themes (Glass, Midnight, Ember, Neon, Frost), size, detailed or compact rows, what to show, and every hotkey.
- **The team site** at [dota.cyberspyde.uz](https://dota.cyberspyde.uz): download, live view of everyone, match history, and a **post-game report** for every game.

![Overlay](docs/screenshots/overlay.png)

A real capture of `cyberspyde-agent --demo` (placeholder names). Each player has two lines: status dot (green ok, amber low, red under attack), name, hero and level, `ULT` (green ready, grey with seconds left), `TP` (green ready, grey cooldown, red none); then HP with a thin mana line, observer wards carried (`W2`) and up to three fight-changing items. A dead player shows the respawn time and whether buyback is possible.

## Post-game report

Dota already shows damage, KDA and net worth, so the report measures what it can't: how each player behaved **as part of the team**. The relay records one sample per player per game second (positions, HP, gold, cooldowns, callouts) and analyses the game when it ends:

| Measure | How |
|---|---|
| Team fights joined / skipped | Fights are found from burst damage near allies and deaths; present = within 3000 units of the fight |
| Skipped a fight with TP ready | Alive, more than 4500 away, TP off cooldown at the start of the fight |
| Help calls answered | After a HELP or GROUP UP callout, did you close half the distance within 30 s, and how fast |
| Died alone | Nearest living ally more than 3000 away at the moment of death |
| Died with items unused | BKB, Glimmer, Force Staff and the like off cooldown when you died |
| Died after a RETREAT call | Within 20 s of a teammate's retreat callout |
| Sat on gold | 2,000+ unspent gold for two minutes or more |
| Carried wards around | Observers in the inventory for three minutes or more before placing them |
| Time far from the team | Share of time (after 10:00) more than 4000 from every ally |

Each player gets a **usefulness score** out of 100 (fights joined, calls answered, deaths alone, skipped fights, unspent gold, wards; KDA barely counts), a grade from S to D, a nickname for what stood out ("Lone Wolf", "First Responder", "Dragon on a Gold Pile"), their worst and best moments with game time, and a chart of how spread out the team was. A **local AI coach** (Ollama on the team server, Qwen 3.6) then writes a short, honest review per player from those numbers. It only sees the computed report, never the host's private notes.

## Personal coaching (private)

The host can switch on coaching aids per player in a private `team.toml` (see `examples/team.example.toml`). It lives only on the host PC and is git-ignored. When a player connects, the relay sends them only their own switches, for example `["team_mode_banner", "fight_focus"]`, plus an alert text size. Nobody receives anyone else's profile, and the host's `notes` never leave the file. Players just see clearer prompts.

| Switch | What the player sees |
|---|---|
| `team_mode_banner` | A banner across the top: TEAM FIGHTING / TEAM RETREATING / GROUP UP. It turns red ("TURN AROUND - GO IN" or "STOP - RUN BACK") when the player moves the opposite way from the team |
| `fight_focus` | During a fight, the build line, radar and notes disappear |
| `join_fight_nudge` | "Fight 5.2k away - TP in" |
| `isolation_warning` | "You're alone - X 6.1k" (mid-game, outside base, after 8 s) |
| `engage_advisor` | "Fight window: 3 close, 3 ults - go fast", and "Fight dragging - disengage" |
| `low_hp_retreat` | "LOW HP - back off" while hurt in a fight |
| `ward_reminders` | "No observers on team - buy" / "Place your observer" |
| `buyback_guard` | Late game: "Below buyback gold (800/1200)" |

How the team's state is read: allies taking burst damage with another ally within 2500 = a team fight; a retreat callout or 2+ dead = retreating; a group-up callout = grouping. Movement is measured over the last 1.5 s. Everything is computed on each player's own PC from the shared state, so it adds no delay.

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
| `crates/relay` | `cyberspyde-relay` | Team server: fans each player's state out to everyone else; records matches, analyses them, serves the site |
| `crates/agent` | `cyberspyde-agent` | The one program players run: GSI receiver, relay client, overlay, menu, build helper |
| `crates/sim` | `cyberspyde-sim` | Developer tool, not shipped: latency test with N fake players, and `--script`, a 40-minute scripted match for testing the report |
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

`dist\` gets two programs: `cyberspyde-agent.exe` is the only one players need (the site offers it for download), and `cyberspyde-relay.exe` runs on the team server. `cargo build -p sim` builds the test tool when you need it.

## Running

### The relay

The team relay runs on an always-on server at **`dota.cyberspyde.uz`**, behind a TLS reverse proxy (Caddy). Agents connect to `wss://dota.cyberspyde.uz/ws` by default, so nobody's home internet has to stay up for the others. The team board is at `https://dota.cyberspyde.uz/#key=<team-key>`.

To run your own relay instead:

```powershell
.\dist\cyberspyde-relay.exe --bind 0.0.0.0:4747 --key-file team-key.txt --team team.toml --data-dir data --public-dir public --ai-url http://127.0.0.1:11434
```

- `--key-file` keeps the key out of the command line; `--key <key>` and the `RELAY_KEY` variable also work.
- `--data-dir`: match history (one report and one raw recording per game). `cyberspyde-relay analyze <match.rec.json>` re-runs the analysis offline.
- `--public-dir`: `cyberspyde-agent.exe` and `version.txt` for the download button (agents see "update available" when the version there is newer), plus `assets/` images for the landing page.
- `--ai-url` / `--ai-model` / `--ai-lang`: Ollama for the AI coach (off without `--ai-url`; default model `qwen3.6-agent`, English).
- `--team` is optional; without it nobody gets coaching aids. The file is re-read on every connect, so edits apply when a player reconnects.
- Either open TCP port 4747 (players then enter the bare IP, which becomes `ws://<ip>:4747/ws`), or put the relay on `127.0.0.1:4747` behind a reverse proxy with a domain, which becomes `wss://<domain>/ws`.

### Every player: the agent

1. Download `cyberspyde-agent.exe` from [dota.cyberspyde.uz](https://dota.cyberspyde.uz) and run it. The first time, a setup window asks for your name (as the team knows you) and the team key, and lets you pick a theme, size, layout and side. The server is already filled in.

   Settings are saved in `agent.toml` next to the program. Change them any time in game with **Numpad \***. There is no console window; logs go to `agent.log` (or start with `--console`).
2. The agent finds Dota through Steam and writes `game/dota/cfg/gamestate_integration/gamestate_integration_cyberspyde.cfg`.
3. In Steam, add `-gamestateintegration` to Dota 2's launch options, then restart Dota.
4. In Dota, set Display Mode to **Borderless window**. The overlay can't draw over exclusive fullscreen.

Try it without Dota: `cyberspyde-agent.exe --demo` (your four teammates with fake game data; one keeps getting ganked).

### Hotkeys (change them in the menu)

| Key | Action |
|---|---|
| Numpad * | Open / close the menu. While it's open, drag the overlay anywhere; drag the menu by its title. Both places are remembered |
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

- The team key is checked by the relay. Traffic to the team server is encrypted (`wss://`, certificate verified). A self-hosted relay on a bare IP uses plain `ws://`, so don't reuse a real password as the key.
- The site's landing page and download are public; the live view, match history, reports and player stats need the team key. The site keeps the key in the browser and sends it in a request header, never in a URL query.
- The GSI receiver listens only on `127.0.0.1` and requires the random token written into Dota's cfg.

## Not yet built

- Counter-pick suggestions: player-mode GSI doesn't include the enemy draft. Next step: type enemy heroes on the team board and pull matchup data from OpenDota.
- Item icons: the overlay uses short text labels today.
