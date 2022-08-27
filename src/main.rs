#![allow(non_snake_case)]

use exec::Executor;
use vfs::VFS;

mod exec;
mod tokenizer;
mod vfs;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref ROM : &'static str = include_str!(r"..\BOOT.vraw");
}

fn main() {
    let mut vfs = VFS::create_empty();
    let f = vfs.create_file(ROM.as_bytes().to_vec(), "BOOT.vraw".to_owned(), true);
    vfs.write_file(f).expect("Failed to write bootloader into VFS.");

    let assembly = tokenizer::parse_asm(&ROM);

    let mut exec = Executor::new(assembly);
    exec.run();
}