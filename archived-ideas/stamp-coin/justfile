default: run

build: audit
    cargo build --release --target x86_64-unknown-linux-musl

dist binary: build
    mkdir -p ./dist

    cp -v ./target/x86_64-unknown-linux-musl/release/{{binary}} ./dist/

format:
    cargo fmt -- --check

audit:
    cargo audit

licensing:
    cargo deny check

run:
    cargo run --release --target x86_64-unknown-linux-musl

musl:
    cargo build --release --target x86_64-unknown-linux-musl

clean:
    rm -rf ./dist/
    cargo clean
