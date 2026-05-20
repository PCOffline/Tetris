# Tetris — TODO

## Phase 1: Core Mechanics
- [x] Board grid & camera setup
- [x] Spawn a piece (O-piece hardcoded)
- [x] Gravity (piece falls over time)
- [x] Collision detection (floor + landed blocks)
- [x] Lock piece on collision
- [x] Left/right movement (keyboard input)
- [x] Soft drop (hold down to fall faster)
- [x] Hard drop (instant drop to bottom when pressing space)
- [ ] Piece rotation (clockwise & counter-clockwise)
- [ ] Wall kick (adjust position when rotation is blocked by walls/blocks)
- [x] Collision detection for lateral movement (prevent moving into walls/blocks)

## Phase 2: Piece System
- [x] Define all 7 tetrominoes (I, O, T, S, Z, J, L) with their shapes and pivot points
- [x] Random piece spawning (spawn a random piece when the previous one locks)
- [ ] Piece preview (show the next piece)
- [ ] Piece bag (7-bag randomizer: shuffle all 7 pieces, deal them out, repeat)
- [ ] Hold piece (swap current piece with held piece, once per drop)

## Phase 3: Line Clearing & Scoring
- [ ] Detect full rows
- [ ] Clear full rows (remove blocks, shift everything above down)
- [ ] Score system (points for singles, doubles, triples, tetrises)
- [ ] Level system (level increases every N lines cleared)
- [ ] Speed scaling (gravity gets faster as level increases)
- [ ] Combo tracking (consecutive line clears)

## Phase 4: Game State
- [ ] Game over detection (new piece spawns overlapping existing blocks)
- [ ] Game over screen
- [ ] Start / title screen
- [ ] Pause menu
- [ ] Restart functionality

## Phase 5: Aesthetics & Polish
- [ ] Unique color per tetromino type
- [ ] Board border / frame
- [ ] Background (solid color or gradient)
- [ ] Grid lines (subtle lines to show the cell boundaries)
- [ ] Ghost piece (translucent preview of where the piece will land)
- [ ] Lock delay (brief pause before a piece locks, allowing last-second moves)
- [ ] Line clear animation (flash, dissolve, or slide)
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
