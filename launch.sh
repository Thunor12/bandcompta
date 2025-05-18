trap "exit" INT TERM ERR
trap "kill 0" EXIT

cargo build --manifest-path backend/Cargo.toml

cargo run --manifest-path backend/Cargo.toml &

npm --prefix frontend/ run dev &

wait