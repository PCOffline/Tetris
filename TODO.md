# Tetris — TODO

## Phase 1: Core Mechanics
- [x] Board grid & camera setup
- [x] Spawn a piece
- [x] Gravity (piece falls over time)
- [x] Collision detection (floor + landed blocks)
- [x] Lock piece on collision
- [x] Left/right movement (`A`/`D` or `←`/`→`)
- [x] Soft drop (`S` or `↓`)
- [x] Hard drop (`Space` — instant drop to bottom)
- [x] Piece rotation clockwise (`W` or `↑`)
- [ ] Piece rotation counter-clockwise (needs keybind, e.g. `Z` or `Q`)
- [ ] Wall kick (adjust position when rotation is blocked by walls/blocks)
- [x] Collision detection for lateral movement (prevent moving into walls/blocks)

## Phase 2: Piece System
- [x] Define all 7 tetrominoes (I, O, T, S, Z, J, L) with shapes and rotations
- [x] Random piece spawning (spawn a random piece when the previous one locks)
- [ ] Piece preview (show the next 1–3 pieces)
- [ ] Piece bag (7-bag randomizer: shuffle all 7 pieces, deal them out, repeat)
- [ ] Hold piece (swap current piece with held piece, once per drop; needs keybind, e.g. `C` or `Shift`)

## Phase 3: Line Clearing & Scoring
- [x] Detect full rows
- [x] Remove full rows from board
- [x] Line clear animation (fade out + despawn after timer)
- [ ] Shift rows down after clearing (update both Board data and block Positions)
- [ ] Score system (points for singles, doubles, triples, tetrises)
- [ ] Level system (level increases every N lines cleared)
- [ ] Speed scaling (gravity interval decreases as level increases)
- [ ] Combo tracking (consecutive line clears)

## Phase 4: Game State
- [x] Game over detection (piece spawns overlapping existing blocks → `GameState::Ended`)
- [ ] Game over screen (display message, final score)
- [ ] Start / title screen
- [ ] Pause menu (`Escape` or `P` to toggle `GameState::Paused`)
- [ ] Restart functionality (reset board, score, and level)

## Phase 5: Aesthetics & Polish
- [x] Unique color per tetromino type
- [x] Board border / frame (dark rectangle behind the grid)
- [x] Lock delay (brief pause before a piece locks, resets on lateral movement)
- [ ] Background (solid color or gradient behind the board frame)
- [ ] Grid lines (subtle lines to show cell boundaries)
- [ ] Ghost piece (translucent preview of where the piece will land)
- [ ] Piece spawn animation (fade in)
- [ ] Screen shake or flash on Tetris (4-line clear)
- [ ] Particle effects on line clear

## Phase 6: UI & HUD
- [ ] Score display
- [ ] Level display
- [ ] Lines cleared counter
- [ ] Next piece preview box
- [ ] Hold piece display box
- [ ] High score (persist to file)

## Phase 7: Audio
- [ ] Background music
- [ ] Sound effect: piece move
- [ ] Sound effect: piece rotate
- [ ] Sound effect: piece lock / land
- [ ] Sound effect: line clear
- [ ] Sound effect: Tetris (4-line clear, distinct from regular clear)
- [ ] Sound effect: game over

## Phase 8: Nice-to-Haves
- [ ] T-spin detection & bonus scoring
- [ ] Adjustable controls / key remapping
- [ ] DAS (Delayed Auto Shift) for smooth left/right holding
- [ ] Configurable settings (starting level, grid size)
- [ ] Back-to-back bonus (consecutive Tetrises / T-spins)

## Phase 9: Refactors
- [x] Spawn the next piece from `OnEnter(PlayState::Spawning)` instead of a message
- [x] Remove `GameStarted`, `spawn_initial_piece` and `ClearingFinished`
- [x] Gate systems per `PlayState` phase
- [x] Only leave `Clearing` once every clearing timer has finished
- [x] Don't set `PlayState::Falling` after a game over
- [x] Change state through `NextState` everywhere
- [x] Rename `GameState::Started` to `Playing`
- [x] Move pause out of `GameState` into its own sub-state (`IsPaused`)
- [x] Gate the pause-sensitive systems on `IsPaused::Unpaused`
- [x] Remove cleared blocks from the `Board`
- [x] Delete `SpawnSet`
- [x] Register the states in `main.rs` instead of `PiecePlugin`
- [x] Tear down ghost pieces on `OnExit(PlayState::Falling)` instead of on `PieceLocked`
- [x] Spawn the next piece even if the game is paused while entering `Spawning`
- [ ] Log when a block's `Position` and the `Board` disagree
- [x] Fix pieces merging into locked blocks on lock
- [x] Make row shifting independent of query iteration order
- [x] Validate moves and rotations against `ActivePieceState` instead of `Position`
- [x] Fix the ordering of `sync_active_piece_positions` relative to movement and lock
- [ ] Separate the lock timer from `ActivePieceState`
- [ ] One owner for the state change after a piece locks
- [ ] Gate `toggle_pause_on_keyboard_input` on `Playing`
- [x] Fix the `retain` in `spawn_piece`
- [ ] Represent "no active piece" properly instead of the dummy `ActivePieceState::default()`
- [ ] Choose the initial `GameState` from the debug config and wire up `auto_start` and `Menu`
- [ ] Stop reading `DebugConfig` at plugin build time
- [ ] Split `piece.rs` into spawn, active piece, lock and clearing modules

## Phase 10: Concepts to Learn
- [x] Gating systems with `run_if(in_state(..))`
- [x] `OnEnter` for one-shot setup such as spawning
- [x] A sub-state only exists while its source state matches
- [x] One enum holds one fact; independent facts belong in separate sub-states
- [x] System ordering only applies within one schedule
- [ ] `OnEnter` / `OnExit` only fire once per state change
- [ ] Pending state requests still resolve while paused
- [ ] Reading the state of a sub-state that doesn't exist
- [ ] Pausing `Time<Virtual>`
- [ ] `NextState::set` is a request applied in `StateTransition`, hence one frame in `Spawning`
- [ ] `Commands` are deferred
- [ ] `NextState` is last-write-wins
- [ ] Messages vs states
- [ ] Systems with conflicting access and no ordering run in arbitrary order
- [ ] Change detection and `ResMut`
- [x] Query iteration order isn't guaranteed
- [x] One writer per fact: `Board`, `Position` and `ActivePieceState` can drift apart
- [ ] Reading config at plugin build time makes plugin order matter
