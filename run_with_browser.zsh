#!/bin/zsh

# cargo run
cargo build
(
	sleep 1
	google-chrome http://127.0.0.1:8080/app/top
)&
./target/debug/saint-sorting
