# Polyglot Conway's Game of Life

A simple learning playground for implementing [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

The beauty of using the Game of Life as a learning playground is that it starts out extremely simple, but can become as complex and fully featured as we'd like. We can get into algorithms, graphics, testing, benchmarking, coding practices, or whatever else we can imagine. Or we could just implement it in a script with a hundred lines of code!

This project will be polyglottal so each top level folder should be the project root for a different language/tech. The top-level folder `patterns` is reserved for plaintext starting state patterns that can be read in to the game program.

## To Do

- [ ] better stats (sparkline for pop or something? side panel instead of bottom to keep pane more square)
  - [ ] mem usage?
- [ ] perf
  - [ ] clean app, state, etc. structs
  - [ ] benchmarks
  - [ ] unroll delta loops
  - [ ] remove branches where possible
  - [ ] best practices around sets (clear vs new, contains etc.)
  - [ ] repr pt struct impl eq or lists or something?
- [ ] CLI
  - [ ] pattern
  - [ ] tick rate
  - [ ] zoom level
- [ ] UI simplify to single side panel?
- [ ] add non-tui graphics version
  - [ ] tui/graphics cli option
- [ ] Haskell version
