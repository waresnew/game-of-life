[default]
dev:
    wasm-pack build --release --target web
    NO_COLOR=1 bun ./web/index.html

profile:
    wasm-pack build --dev --target web
    NO_COLOR=1 bun ./web/index.html
