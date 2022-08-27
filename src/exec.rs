use std::collections::HashMap;

use crate::tokenizer::Assembly;

// Memory size in bytes
const MEM_SIZE : usize = 2;

// Minimum accessable memory address, will cause segmentation fault if read below this.
// Consider this reserved for system use.
const RESERVED_MIN_MEM_ADDR : usize = 0;

// Unsafe behaviour - continue after fault (e.g Segmentation Fault etc)
// Only use when absolutely certain of behaviour.
const CONTINUE_AFTER_FAULT : bool = false;

pub struct Executor {
    index : usize,
    tokens : Vec<&'static str>,
    label_table: HashMap<String, usize>
}

impl Executor {
    pub fn new(asm : Assembly) -> Self {
        Self {
            index: 0,
            tokens: asm.tokens,
            label_table: asm.label_table
        }
    }
    pub fn run(&mut self) {
        let mut rax : u8 = 0; // Registers
        let mut rbx : u8 = 0;
        let mut rcx : u8 = 0;
        let mut rdx : u8 = 0;

        let mut mem : [u8; MEM_SIZE] = [0; MEM_SIZE]; // Memory initialized to all 0s.
        let len = self.tokens.len();
        loop {
            let code = self.tokens[self.index].clone();
            
            match code {
                "label" => {
                    self.index += 1;
                },
                "dmp" => println!(
                    "!!! DUMPED !!!\n  Registers:\n    RAX: {}\n    RBX: {}\n    RCX: {}\n    RDX: {}\n  Memory:\n    {}",
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
        
                    mem[address] = value;
                },
                "mov" => {
                    self.index += 1;
                    let dest = self.tokens[self.index];
                    self.index += 1;
                    let src = self.tokens[self.index];
                    let value = match src {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => {
                            if let Ok(addr) = src.parse::<usize>() {
                                mem[addr]
                            } else {
                                panic!("Invalid token `{}` after mov instruction at instr #{}", src, self.index);
                            }
                        }
                    };
        
                    match dest {
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
                            if let Ok(addr) = dest.parse::<usize>() {
                                if addr >= MEM_SIZE || addr < RESERVED_MIN_MEM_ADDR {
                                    fault(format!("Segmentation fault - Accessed memory out of bounds. Address: {}. Instr #{}", addr, self.index));
                                    continue;
                                }
        
                                mem[addr] = value;
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
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src1, self.index)
                    };
                    let val2 = match src2 {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src2, self.index)
                    };
                    
                    match dest {
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
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src1, self.index)
                    };
                    let val2 = match src2 {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src2, self.index)
                    };
                    
                    match dest {
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
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src1, self.index)
                    };
                    let val2 = match src2 {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src2, self.index)
                    };
                    
                    match dest {
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
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src1, self.index)
                    };
                    let val2 = match src2 {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", src2, self.index)
                    };
                    
                    match dest {
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
                        _ => panic!("Unrecognized register `{}` at instr #{}", dest, self.index)
                    };
                },
                "goto" => {
                    self.index += 1;
                    if let Some(position) = self.label_table.get(self.tokens[self.index]) {
                        self.index = position.clone();
                        continue;
                    } else {
                        panic!("Invalid label {} at instr #{}.", self.tokens[self.index], self.index);
                    }
                },
                "cgt" => {
                    self.index += 1;
                    let reg = self.tokens[self.index];
                    self.index += 1;

                    let condition = match reg {
                        "rax" => rax > 0,
                        "rbx" => rbx > 0,
                        "rcx" => rcx > 0,
                        "rdx" => rdx > 0,
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
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };
                    self.index += 1;
                    let rhs = match self.tokens[self.index] {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };

                    let value = if rhs > lhs { 1 } else { 0 };

                    match dest {
                        "rax" => { rax = value },
                        "rbx" => { rbx = value },
                        "rcx" => { rcx = value },
                        "rdx" => { rdx = value },
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    }
                },
                "lt" => {
                    self.index += 1;
                    let dest = self.tokens[self.index];
                    self.index += 1;
                    let lhs = match self.tokens[self.index] {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };
                    self.index += 1;
                    let rhs = match self.tokens[self.index] {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };

                    let value = if rhs < lhs { 1 } else { 0 };

                    match dest {
                        "rax" => { rax = value },
                        "rbx" => { rbx = value },
                        "rcx" => { rcx = value },
                        "rdx" => { rdx = value },
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    }
                },
                "eq" => {
                    self.index += 1;
                    let dest = self.tokens[self.index];
                    self.index += 1;
                    let lhs = match self.tokens[self.index] {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };
                    self.index += 1;
                    let rhs = match self.tokens[self.index] {
                        "rax" => rax,
                        "rbx" => rbx,
                        "rcx" => rcx,
                        "rdx" => rdx,
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
                    };

                    let value = if rhs == lhs { 1 } else { 0 };

                    match dest {
                        "rax" => { rax = value },
                        "rbx" => { rbx = value },
                        "rcx" => { rcx = value },
                        "rdx" => { rdx = value },
                        _ => panic!("Unrecognized register `{}` at instr #{}", self.tokens[self.index], self.index)
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