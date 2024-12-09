test:
    cargo test

all:
    cargo run --release -- -1

run day:
    cargo run {{day}}

fast day:
    cargo run --release {{day}}