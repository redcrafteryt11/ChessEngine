#![allow(dead_code, unused_imports, unused_mut)]
mod board;
mod engine;

fn main() {
    engine::uci::run();
}