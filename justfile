set windows-powershell := true

# Build optimized binary for a day
@build day:
    cargo build --bin day{{day}} --release

# Cargo run optimized build and pipe data to stdin
@run day:
    cat ./data/day{{day}}.txt | cargo run --release --bin day{{day}}

# Test solutions for a day
@test day:
    cargo test --bin day{{day}}

# Create new day's rust file and input file
@new:
    ./gen_d