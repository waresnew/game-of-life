[default]
dev:
    wasm-pack build --release --target bundler
    cd web && bunx vite

profile:
    wasm-pack build --dev --target bundler
    cd web && bunx vite

bin example:
    RUST_BACKTRACE=1 cargo run --release --example {{example}}
    
build:
    wasm-pack build --release --target bundler
    cd web && bunx vite build --minify

bench-web:
    wasm-pack build --release --target bundler
    cd web && \
    bunx vitest bench --run --config=vitest.browser.config.ts

