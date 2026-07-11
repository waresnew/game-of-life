[default]
dev:
    wasm-pack build --release --target bundler
    cd web && bunx vite

profile:
    wasm-pack build --profiling --target bundler
    cd web && bunx vite

build:
    wasm-pack build --release --target bundler
    cd web && bunx vite build --minify

bench-web:
    wasm-pack build --release --target bundler
    cd web && \
    bunx vitest bench --run --config=vitest.browser.config.ts

