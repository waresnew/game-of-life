[default]
dev:
    wasm-pack build --release --target bundler
    cd web && bunx vite

profile:
    wasm-pack build --dev --target bundler
    cd web && bunx vite

bench:
    cargo bench

bin:
    RUST_BACKTRACE=1 cargo run --release
    
build:
    wasm-pack build --release --target bundler
    cd web && bunx vite build --minify
