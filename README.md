# Game of Life

An implementation and web demo of the Hashlife algorithm, which is used to simulate extremely large patterns of the Game of Life for an extremely large number of future generations.

As a background, the Game of Life is a cellular automaton with the following rules:

- Alive cells with more than 3 alive neighbours die.
- Alive cells with less than 2 alive neighbours die.
- Dead cells with 3 alive neighbours become alive.

The thresholds for becoming dead/alive can be configured in this program, but these are the most well-known.

To support very sparse patterns, the world is stored in a quadtree with a hash table for deduplication of subtrees. Since most Game of Life seeds tend to exhibit repeating patterns across time and space, like gliders, oscillators, or empty space, a lot of repeated computation can be avoided with memoization, which allows the recursive algorithm to quickly advance the grid by an exponential number of generations each time.

Note: For simplification, the grid is constrained by (+-2^47, +-2^47). Any cells that exit that region will die.

## Building

The demo website is fully client-side and can be visited here: https://waresnew.github.io/game-of-life/

You can build it locally by running `just build` and opening `./dist/index.html` in a web browser.

### Dependencies 
- just
- wasm-pack
- bun

