# Game of Life

An implementation and web demo of the [Hashlife](https://en.wikipedia.org/wiki/Hashlife) algorithm, which is used to simulate extremely large patterns of the Game of Life for an extremely large number of future generations.

As a background, the [Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) is a cellular automaton with the following rules:

- Alive cells with more than 3 alive neighbours die.
- Alive cells with less than 2 alive neighbours die.
- Dead cells with 3 alive neighbours become alive.

To support very sparse patterns, the world is stored in a quadtree with a hash table for deduplication of subtrees. Since most Game of Life seeds tend to exhibit repeating patterns, like gliders, oscillators, or empty space, a lot of repeated computation can be avoided with memoization, which allows the recursive algorithm to quickly advance the grid by an exponential number of generations each time.

Note: To simplify rendering on the JavaScript side, the grid is constrained by (-1e14, -1e14) and (1e14, 1e14). Any cells that exit that region will die.

Pattern presets were taken from [LifeWiki](https://conwaylife.com/patterns/).

## Building

The demo website is fully client-side and can be visited here: https://waresnew.github.io/game-of-life/

Several preset patterns are provided in the [.rle](https://conwaylife.com/wiki/Run_Length_Encoded) format. You can upload your own via file.

You can build it locally by running `just build` and opening `./dist/index.html` in a web browser.

### Dependencies 
- [just](https://github.com/casey/just)
- [wasm-pack](https://github.com/wasm-bindgen/wasm-pack)
- [bun](https://github.com/oven-sh/bun)

