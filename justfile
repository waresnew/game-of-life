[default]
dev:
    wasm-pack build --release --target web
    NO_COLOR=1 bun ./web/index.html

profile:
    wasm-pack build --dev --target web
    NO_COLOR=1 bun ./web/index.html

bench:
    cargo bench

bin:
    RUST_BACKTRACE=1 cargo run --release
    
build:
    wasm-pack build --release --target web
    bun build --compile --target=browser --minify ./web/index.html --outdir=dist
