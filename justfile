[default]
dev:
    wasm-pack build --target web
    bun ./src/web/index.html
