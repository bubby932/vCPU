#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
lazy_static! {
    #[derive(Debug)]
    static ref READ_ONLY_MEMORY : &'static str = include_str!(r"../BOOT");
}

// Memory size in bytes
const MEM_SIZE : usize = 8;

// Minimum accessable memory address, will cause segmentation fault if read below this.
// Consider this reserved for system use.
const RESERVED_MIN_MEM_ADDR : usize = 0;

// Unsafe behaviour - continue after fault (e.g Segmentation Fault etc)
// Only use when absolutely certain of behaviour.
const CONTINUE_AFTER_FAULT : bool = false;

fn main() {
    let mut instructions = READ_ONLY_MEMORY
        .split_whitespace()
        .enumerate()
        .into_iter();

    let mut rax : u8 = 0; // Registers
    let mut rbx : u8 = 0;
    let mut rcx : u8 = 0;
    let mut rdx : u8 = 0;

    let mut mem : [u8; MEM_SIZE] = [0; MEM_SIZE]; // Memory initialized to all 0s.

    while let Some((index, code)) = instructions.next() {
        match code {
            "dmp" => println!(
                "!!! DUMPED !!!\n  ROM: {}\n  Registers:\n    RAX: {}\n    RBX: {}\n    RCX: {}\n    RDX: {}\n  Memory:\n    {}", 
                READ_ONLY_MEMORY.to_owned(),
                rax,
                rbx,
                rcx,
                rdx,
                mem.into_iter()
                    .enumerate()
                    .map(|(index, byte)| { format!("{}: {}", index, byte) })
                    .collect::<Vec<String>>()
                    .join("\n    ")
            ),
            "panic" => panic!("Panic requested by instruction set at instr #{}", index),
            "fault" => fault(format!("Fault requested by instr #{}.", index)),
            "memset" => {
                let addr_token = instructions.next().expect("EOF after memset instruction.");
                let address = addr_token.1.parse::<usize>().expect("Invalid memory address in memset instruction. No memory write occurred.");

                let unparsed = instructions.next().expect("EOF after memset address. No memory write occurred.");
                let value = unparsed.1.parse::<u8>().expect("Invalid value in memset operation. No memory write occurred.");

                if address < RESERVED_MIN_MEM_ADDR || address >= MEM_SIZE {
                    fault(format!("Segmentation fault - Accessed memory out of bounds. Address: {}. Instr #{}", address, addr_token.0));
                    continue;
                }

                mem[address] = value;
            },
            "mov" => {
                let dest = instructions.next().expect("EOF after mov instruction.");
                let src = instructions.next().expect("EOF after mov instruction.");
                let value = match src.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => {
                        if let Ok(addr) = src.1.parse::<usize>() {
                            mem[addr]
                        } else {
                            panic!("Invalid token `{}` after mov instruction at instr #{}", src.1, src.0);
                        }
                    }
                };

                match dest.1 {
                    "rax" => {
                        rax = value;
                    }
                    "rbx" => {
                        rbx = value;
                    }
                    "rcx" => {
                        rcx = value;
                    }
                    "rdx" => {
                        rdx = value;
                    }
                    _ => {
                        if let Ok(addr) = dest.1.parse::<usize>() {
                            if addr >= MEM_SIZE || addr < RESERVED_MIN_MEM_ADDR {
                                fault(format!("Segmentation fault - Accessed memory out of bounds. Address: {}. Instr #{}", addr, index));
                                continue;
                            }

                            mem[addr] = value;
                        } else {
                            panic!("Invalid token `{}` after mov instruction at instr #{}", dest.1, dest.0);
                        }
                    }
                }
            }
            "add" => {
                let src1 = instructions.next().expect("EOF after add instr.");
                let src2 = instructions.next().expect("EOF after add instr.");
                let dest = instructions.next().expect("EOF after add instr.");

                let val1 = match src1.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => panic!("Unrecognized register `{}` at instr #{}", src1.1, src1.0)
                };
                let val2 = match src2.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => panic!("Unrecognized register `{}` at instr #{}", src2.1, src2.0)
                };
                
                match dest.1 {
                    "rax" => {
                        rax = val1 + val2;
                    },
                    "rbx" => {
                        rbx = val1 + val2;
                    },
                    "rcx" => {
                        rcx = val1 + val2;
                    },
                    "rdx" => {
                        rdx = val1 + val2;
                    },
                    _ => panic!("Unrecognized register `{}` at instr #{}", dest.1, dest.0)
                };
            },
            "sub" => {
                let src1 = instructions.next().expect("EOF after sub instr.");
                let src2 = instructions.next().expect("EOF after sub instr.");
                let dest = instructions.next().expect("EOF after sub instr.");

                let val1 = match src1.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => panic!("Unrecognized register `{}` at instr #{}", src1.1, src1.0)
                };
                let val2 = match src2.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => panic!("Unrecognized register `{}` at instr #{}", src2.1, src2.0)
                };
                
                match dest.1 {
                    "rax" => {
                        rax = val1 - val2;
                    },
                    "rbx" => {
                        rbx = val1 - val2;
                    },
                    "rcx" => {
                        rcx = val1 - val2;
                    },
                    "rdx" => {
                        rdx = val1 - val2;
                    },
                    _ => panic!("Unrecognized register `{}` at instr #{}", dest.1, dest.0)
                };
            },
            "mul" => {
                let src1 = instructions.next().expect("EOF after mul instr.");
                let src2 = instructions.next().expect("EOF after mul instr.");
                let dest = instructions.next().expect("EOF after mul instr.");

                let val1 = match src1.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => panic!("Unrecognized register `{}` at instr #{}", src1.1, src1.0)
                };
                let val2 = match src2.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => panic!("Unrecognized register `{}` at instr #{}", src2.1, src2.0)
                };
                
                match dest.1 {
                    "rax" => {
                        rax = val1 * val2;
                    },
                    "rbx" => {
                        rbx = val1 * val2;
                    },
                    "rcx" => {
                        rcx = val1 * val2;
                    },
                    "rdx" => {
                        rdx = val1 * val2;
                    },
                    _ => panic!("Unrecognized register `{}` at instr #{}", dest.1, dest.0)
                };
            },
            "div" => {
                let src1 = instructions.next().expect("EOF after div instr.");
                let src2 = instructions.next().expect("EOF after div instr.");
                let dest = instructions.next().expect("EOF after div instr.");

                let val1 = match src1.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => panic!("Unrecognized register `{}` at instr #{}", src1.1, src1.0)
                };
                let val2 = match src2.1 {
                    "rax" => rax,
                    "rbx" => rbx,
                    "rcx" => rcx,
                    "rdx" => rdx,
                    _ => panic!("Unrecognized register `{}` at instr #{}", src2.1, src2.0)
                };
                
                match dest.1 {
                    "rax" => {
                        rax = val1 / val2;
                    },
                    "rbx" => {
                        rbx = val1 / val2;
                    },
                    "rcx" => {
                        rcx = val1 / val2;
                    },
                    "rdx" => {
                        rdx = val1 / val2;
                    },
                    _ => panic!("Unrecognized register `{}` at instr #{}", dest.1, dest.0)
                };
            },
            _ => fault(format!("Unrecognized instruction `{}` at instr #{}", code, index))
        }
    }
}

fn fault(cause : String) {
    if CONTINUE_AFTER_FAULT {
        println!("!!! FAULTED !!!\n  Cause: {}", cause)
    } else {
        panic!("!!! FAULTED !!!\n  Cause: {}", cause);
    }
}
