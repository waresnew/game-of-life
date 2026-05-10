[default]
dev:
    wasm-pack build --release --target web
    bun ./web/index.html

profile:
    wasm-pack build --dev --target web
    bun ./web/index.html
