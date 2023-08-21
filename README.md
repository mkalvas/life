# Polyglot Conway's Game of Life

A simple learning playground for implementing [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

The beauty of using the Game of Life as a learning playground is that it starts out extremely simple, but can become as complex and fully featured as we'd like. We can get into algorithms, graphics, testing, benchmarking, coding practices, or whatever else we can imagine. Or we could just implement it in a script with a hundred lines of code!

This project will be polyglottal so each top level folder should be the project root for a different language/tech. The top-level folder `patterns` is reserved for plaintext starting state patterns that can be read in to the game program.

## To Do

### Rust

- [x] game
  - [x] reset game keybind
  - [x] pre-made pattern selector (use [popup](https://github.com/ratatui-org/ratatui/blob/main/examples/popup.rs)?)
- [x] tests
  - [x] basic
  - [ ] detailed?
- [x] cli
  - [x] pattern
  - [x] tick rate
  - [x] zoom level
- [x] ui
  - [x] tui
  - [x] simplify to single side panel layout? Kinda
  - [x] sparkline for population (cool slice-deque!)
  - [ ] add non-tui graphics version
    - [ ] tui/graphics cli option
- [x] perf
  - [x] clean app
  - [x] clean tui
  - [x] clean state
  - [x] benchmarks
  - [x] unroll delta loops
  - [x] remove branches where possible
  - [x] best practices around sets (clear vs new, contains etc.)
  - [ ] mem profiling (see <https://github.com/koute/bytehound>)

### JS / TS

- [x] game
  - [x] oop style
  - [ ] unbounded grid
- [x] tests
- [ ] cli
  - [ ] pattern
- [ ] ui
  - [ ] nicer output?

### Haskell

- [ ] game
- [ ] tests
- [ ] cli
- [ ] ui

### React + WASM

- [ ] game
- [ ] tests
- [ ] cli
- [ ] ui
