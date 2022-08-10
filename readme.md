# Description

this repo is for educational purposes only.
it contains homework projects for rust course, provided by otus.ru

## transmitter.rs
mocks some source temperature data broadcasting via udp socket

## main.rs
runs thermometer that receives temperature data from udp socket

# Testing: 

-run main first to open socket connection: 
cargo run --bin main

-run transmitter:
cargo run --bin transmitter