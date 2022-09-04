use std::collections::HashMap;

use crate::tokenizer::{Assembly, self};
use crate::vfs::VFS;

// Memory size in bytes
const MEM_SIZE : usize = 512;

const DUMP_VFS : bool = false;

// Minimum accessable memory address, will cause segmentation fault if read below this.
// Consider this reserved for system use.
const RESERVED_MIN_MEM_ADDR : usize = 0x0;

// Unsafe behaviour - continue after fault (e.g Segmentation Fault etc)
// Only use when absolutely certain of behaviour.
const CONTINUE_AFTER_FAULT : bool = false;

pub struct Executor {
    index : usize,
    tokens : Vec<&'static str>,
    label_table: HashMap<String, usize>,
    vfs : VFS,
    rax : u8,
    rbx : u8,
    rcx : u8,
    rdx : u8,
    mem : [u8; MEM_SIZE]
}

impl Executor {
    pub fn new(vasm : Assembly, vfs : VFS) -> Self {
        Self {
            index: 0,
            tokens: vasm.tokens,
            label_table: vasm.label_table,
            vfs,
            rax : 0,
            rbx : 0,
            rcx : 0,
            rdx : 0,
            mem : [0 ; MEM_SIZE]
        }
    }
    pub fn run(&mut self) {
        let len = self.tokens.len();
        loop {
            let code = self.tokens[self.index].clone();
            
            match code {
                "label" => {
                    self.index += 1;
                },
                "dmp" => if DUMP_VFS {
                    println!(
                        "!!! DUMPED !!!\n  Registers:\n    RAX: {}\n      *RAX: {}\n    RBX: {}\n      *RBX: {}\n    RCX: {}\n      *RCX: {}\n    RDX: {}\n      *RDX: {}\n  Memory:\n    {}\nVFS:\n    {:#?}",
                    self.rax,
                    self.mem[self.rax as usize],
                    self.rbx,
                    self.mem[self.rbx as usize],
                    self.rcx,
                    self.mem[self.rcx as usize],
                    self.rdx,
                    self.mem[self.rdx as usize],
                    self.mem.into_iter()
                        .enumerate()
                        .map(|(index, byte)| { format!("{}: {}", index, byte) })
                        .collect::<Vec<String>>()
                        .join("\n    "),
                    self.vfs.dmp());
                } else {
                    println!(
                        "!!! DUMPED !!!\n  Registers:\n    RAX: {}\n      *RAX: {}\n    RBX: {}\n      *RBX: {}\n    RCX: {}\n      *RCX: {}\n    RDX: {}\n      *RDX: {}\n  Memory:\n    {}",
                    self.rax,
                    self.mem[self.rax as usize],
                    self.rbx,
                    self.mem[self.rbx as usize],
                    self.rcx,
                    self.mem[self.rcx as usize],
                    self.rdx,
                    self.mem[self.rdx as usize],
                    self.mem.into_iter()
                        .enumerate()
                        .map(|(index, byte)| { format!("{}: {}", index, byte) })
                        .collect::<Vec<String>>()
                        .join("\n    "));

                },
                "panic" => panic!("Panic requested by instruction set at instr #{}", self.index),
                "fault" => fault(format!("Fault requested by instr #{}.", self.index)),
                "memset" => {
                    self.index += 1;
                    let addr_token = self.tokens[self.index].clone();
                    let address = addr_token.parse::<usize>().expect("Invalid memory address in memset instruction. No memory write occurred.");
        
                    self.index += 1;
                    let unparsed = self.tokens[self.index];
                    let value = unparsed.parse::<u8>().expect("Invalid value in memset operation. No memory write occurred.");
        
                    if address < RESERVED_MIN_MEM_ADDR || address >= MEM_SIZE {
                        fault(format!("Segmentation fault - Accessed memory out of bounds. Address: {}. Instr #{}", address, self.index));
                        continue;
                    }
        
                    self.mem[address] = value;
                },
                "mov" => {
                    self.index += 1;
                    let dest = self.tokens[self.index];
                    self.index += 1;
                    let src = self.tokens[self.index];
                    let value = match src {
                        "rax" => self.mem[self.rax as usize],
                        "rbx" => self.mem[self.rbx as usize],
                        "rcx" => self.mem[self.rcx as usize],
                        "rdx" => self.mem[self.rdx as usize],
                        _ => {
                            if let Ok(addr) = src.parse::<usize>() {
                                self.mem[addr]
                            } else {
                                panic!("Invalid token `{}` after mov instruction at instr #{}", src, self.index);
                            }
                        }
                    };
        
                    match dest {
                        "rax" => {
                            self.rax = value;
                        }
                        "rbx" => {
                            self.rbx = value;
                        }
                        "rcx" => {
                            self.rcx = value;
                        }
                        "rdx" => {
                            self.rdx = value;
                        }
                        _ => {
                            if let Ok(addr) = dest.parse::<usize>() {
                                if addr >= MEM_SIZE || addr < RESERVED_MIN_MEM_ADDR {
                                    fault(format!("Segmentation fault - Accessed memory out of bounds. Address: {}. Instr #{}", addr, self.index));
                                    continue;
                                }
        
                                self.mem[addr] = value;
                            } else {
                                panic!("Invalid token `{}` after mov instruction at instr #{}", dest, self.index);
                            }
                        }
                    }
                }
                "add" => {
                    self.index += 1;
                    let src1 = self.tokens[self.index];
                    self.index += 1;
                    let src2 = self.tokens[self.index];
                    self.index += 1;
                    let dest = self.tokens[self.index];
        
                    let val1 = match src1 {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src1, self.index)
                    };
                    let val2 = match src2 {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src2, self.index)
                    };
                    
                    match dest {
                        "rax" => {
                            self.rax = val1 + val2;
                        },
                        "rbx" => {
                            self.rbx = val1 + val2;
                        },
                        "rcx" => {
                            self.rcx = val1 + val2;
                        },
                        "rdx" => {
                            self.rdx = val1 + val2;
                        },
                        _ => panic!("Unrecognized register `{}` at instr #{}", dest, self.index)
                    };
                },
                "sub" => {
                    self.index += 1;
                    let src1 = self.tokens[self.index];
                    self.index += 1;
                    let src2 = self.tokens[self.index];
                    self.index += 1;
                    let dest = self.tokens[self.index];
        
                    let val1 = match src1 {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src1, self.index)
                    };
                    let val2 = match src2 {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src2, self.index)
                    };
                    
                    match dest {
                        "rax" => {
                            self.rax = val1 - val2;
                        },
                        "rbx" => {
                            self.rbx = val1 - val2;
                        },
                        "rcx" => {
                            self.rcx = val1 - val2;
                        },
                        "rdx" => {
                            self.rdx = val1 - val2;
                        },
                        _ => panic!("Unrecognized register `{}` at instr #{}", dest, self.index)
                    };
                },
                "mul" => {
                    self.index += 1;
                    let src1 = self.tokens[self.index];
                    self.index += 1;
                    let src2 = self.tokens[self.index];
                    self.index += 1;
                    let dest = self.tokens[self.index];
        
                    let val1 = match src1 {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src1, self.index)
                    };
                    let val2 = match src2 {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src2, self.index)
                    };
                    
                    match dest {
                        "rax" => {
                            self.rax = val1 * val2;
                        },
                        "rbx" => {
                            self.rbx = val1 * val2;
                        },
                        "rcx" => {
                            self.rcx = val1 * val2;
                        },
                        "rdx" => {
                            self.rdx = val1 * val2;
                        },
                        _ => panic!("Unrecognized register `{}` at instr #{}", dest, self.index)
                    };
                },
                "div" => {
                    self.index += 1;
                    let src1 = self.tokens[self.index];
                    self.index += 1;
                    let src2 = self.tokens[self.index];
                    self.index += 1;
                    let dest = self.tokens[self.index];
        
                    let val1 = match src1 {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src1, self.index)
                    };
                    let val2 = match src2 {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src2, self.index)
                    };
                    
                    match dest {
                        "rax" => {
                            self.rax = val1 / val2;
                        },
                        "rbx" => {
                            self.rbx = val1 / val2;
                        },
                        "rcx" => {
                            self.rcx = val1 / val2;
                        },
                        "rdx" => {
                            self.rdx = val1 / val2;
                        },
                        _ => panic!("Unrecognized register `{}` at instr #{}", dest, self.index)
                    };
                },
                "goto" => {
                    self.index += 1;
                    let position = match self.tokens[self.index] {
                        "rax" => self.rax as usize,
                        "rbx" => self.rbx as usize,
                        "rcx" => self.rcx as usize,
                        "rdx" => self.rdx as usize,
                        _ => {
                            if let Some(position) = self.label_table.get(self.tokens[self.index]) {
                                *position
                            } else {
                                panic!("Invalid label {} at instr #{}.", self.tokens[self.index], self.index);
                            }
                        }
                    };

                    self.index = position;
                    continue;
                },
                "cgt" => {
                    self.index += 1;
                    let reg = self.tokens[self.index];
                    self.index += 1;

                    let condition = match reg {
                        "rax" => self.rax > 0,
                        "rbx" => self.rbx > 0,
                        "rcx" => self.rcx > 0,
                        "rdx" => self.rdx > 0,
                        _ => panic!("Unrecognized register {} at instr #{}", reg, self.index)
                    };

                    if condition {
                        if let Some(position) = self.label_table.get(self.tokens[self.index]) {
                            self.index = position.clone();
                            continue;
                        } else {
                            panic!("Invalid label {} at instr #{}.", self.tokens[self.index], self.index);
                        }
                    }
                },
                "grt" => {
                    self.index += 1;
                    let dest = self.tokens[self.index];
                    self.index += 1;
                    let lhs = match self.tokens[self.index] {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };
                    self.index += 1;
                    let rhs = match self.tokens[self.index] {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };

                    let value = if rhs > lhs { 1 } else { 0 };

                    match dest {
                        "rax" => { self.rax = value },
                        "rbx" => { self.rbx = value },
                        "rcx" => { self.rcx = value },
                        "rdx" => { self.rdx = value },
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    }
                },
                "lt" => {
                    self.index += 1;
                    let dest = self.tokens[self.index];
                    self.index += 1;
                    let lhs = match self.tokens[self.index] {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };
                    self.index += 1;
                    let rhs = match self.tokens[self.index] {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };

                    let value = if rhs < lhs { 1 } else { 0 };

                    match dest {
                        "rax" => { self.rax = value },
                        "rbx" => { self.rbx = value },
                        "rcx" => { self.rcx = value },
                        "rdx" => { self.rdx = value },
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    }
                },
                "eq" => {
                    self.index += 1;
                    let dest = self.tokens[self.index];
                    self.index += 1;
                    let lhs = match self.tokens[self.index] {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        "*rax" => self.mem[self.rax as usize],
                        "*rbx" => self.mem[self.rbx as usize],
                        "*rcx" => self.mem[self.rcx as usize],
                        "*rdx" => self.mem[self.rdx as usize],
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };
                    self.index += 1;
                    let rhs = match self.tokens[self.index] {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        "*rax" => self.mem[self.rax as usize],
                        "*rbx" => self.mem[self.rbx as usize],
                        "*rcx" => self.mem[self.rcx as usize],
                        "*rdx" => self.mem[self.rdx as usize],
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };

                    let value = if rhs == lhs { 1 } else { 0 };

                    match dest {
                        "rax" => { self.rax = value },
                        "rbx" => { self.rbx = value },
                        "rcx" => { self.rcx = value },
                        "rdx" => { self.rdx = value },
                        "*rax" => { self.mem[self.rax as usize] = value },
                        "*rbx" => { self.mem[self.rbx as usize] = value },
                        "*rcx" => { self.mem[self.rcx as usize] = value },
                        "*rdx" => { self.mem[self.rdx as usize] = value },
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    }
                },
                "outstr" => {
                    self.index += 1;
                    let byte = match self.tokens[self.index] {
                        "rax" => self.rax,
                        "rbx" => self.rbx,
                        "rcx" => self.rcx,
                        "rdx" => self.rdx,
                        "*rax" => self.mem[self.rax as usize],
                        "*rbx" => self.mem[self.rbx as usize],
                        "*rcx" => self.mem[self.rcx as usize],
                        "*rdx" => self.mem[self.rdx as usize],
                        _ => {
                            if let Ok(val) = self.tokens[self.index].parse::<u8>() {
                                val
                            } else {
                                panic!("Invalid out byte.");
                            }
                        }
                    };

                    print!("{}", String::from_utf8_lossy(&[byte]));
                },
                "outbyte" => {
                    self.index += 1;
                    let byte = match self.tokens[self.index] {
                        "rax" => self.mem[self.rax as usize],
                        "rbx" => self.mem[self.rbx as usize],
                        "rcx" => self.mem[self.rcx as usize],
                        "rdx" => self.mem[self.rdx as usize],
                        _ => {
                            if let Ok(val) = self.tokens[self.index].parse::<u8>() {
                                val
                            } else {
                                panic!("Invalid out byte.");
                            }
                        }
                    };

                    print!("{}", &byte);
                },
                "vfsr" => {
                    self.index += 1;
                    let start_ptr = match self.tokens[self.index] {
                        "rax" => self.rax as usize,
                        "rbx" => self.rbx as usize,
                        "rcx" => self.rcx as usize,
                        "rdx" => self.rdx as usize,
                        _ => self.tokens[self.index].parse::<usize>().expect("Failed to parse pointer literal.")
                    };

                    // MEMORY ADDRESS IS A POINTER TO A POINTER NOT A DIRECT POINTER
                    // THIS IS BECAUSE REGISTERS CAN ONLY HOLD A u8 NOT A usize

                    self.index += 1;
                    let identifier = self.tokens[self.index].parse::<u8>().expect("Failed to parse file identifier.");
                    let file = self.vfs.read_file(identifier).expect("Failed to read file.");

                    if file.contents.len() + start_ptr >= MEM_SIZE {
                        fault("SEGMENTATION FAULT - FAILED TO READ FILE INTO INVALID MEMORY".to_owned());
                    }
                    
                    for (ptr, value) in file.contents.iter().enumerate() {
                        self.mem[start_ptr + ptr] = *value;
                    }
                },
                "//" => {
                    self.index += 1;
                    loop {
                        self.index += 1;
                        if self.index >= self.tokens.len() {
                            panic!("EOF after comment without closing.");
                        }

                        if self.tokens[self.index] == "//" {
                            break;
                        }
                    }
                },
                "vrlx" => {
                    self.index += 1;
                    let fid = self.tokens[self.index].parse::<u8>().expect("Failed to parse fid in vrlx op.");

                    let step_read = self.vfs.read_file(fid).expect("Failed to read vraw in vrlx op.");

                    let l : &'static str = Box::leak(String::from_utf8_lossy(&step_read.contents).to_string().into_boxed_str());

                    let step_load = tokenizer::parse_asm(l);

                    // CONTEXT SWITCH //

                    let tmp_index = self.index.clone();
                    let tmp_tokens = self.tokens.clone();
                    let tmp_lt = self.label_table.clone();

                    self.index = 0;
                    self.tokens = step_load.tokens;
                    self.label_table = step_load.label_table;
                    self.run();

                    self.index = tmp_index;
                    self.tokens = tmp_tokens;
                    self.label_table = tmp_lt;
                    
                    // EXIT FORK CONTEXT
                },
                "inv" => {
                    self.index += 1;

                    match self.tokens[self.index] {
                        "rax" => if self.rax > 0 { self.rax = 0; } else { self.rax = 1; },
                        "rbx" => if self.rbx > 0 { self.rbx = 0; } else { self.rbx = 1; },
                        "rcx" => if self.rcx > 0 { self.rcx = 0; } else { self.rcx = 1; },
                        "rdx" => if self.rdx > 0 { self.rdx = 0; } else { self.rdx = 1; },
                        _ => panic!("Unrecognized token after invert instruction.")
                    }
                },
                _ => fault(format!("Unrecognized instruction `{}` at instr #{}", code, self.index))
            }

            self.index += 1;
            if self.index >= len { break; }
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