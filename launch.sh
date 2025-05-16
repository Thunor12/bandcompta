trap "exit" INT TERM ERR
trap "kill 0" EXIT

cargo run --manifest-path backend/Cargo.toml &

npm --prefix frontend/ run dev &

wait