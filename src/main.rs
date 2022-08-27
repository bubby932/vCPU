#![allow(non_snake_case)]

use exec::Executor;
use std::fs;

mod exec;
mod tokenizer;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref ROM : &'static str = include_str!(r"..\BOOT.vraw");
}

fn main() {
    let asm = tokenizer::parse_asm(&ROM);
    let mut executor = Executor::new(asm);
    executor.run();
}

