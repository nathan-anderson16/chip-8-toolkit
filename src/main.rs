use std::{env, fs::File, io::Read};

use c8util::{instructions::Instruction, register::Register};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <path> <output>", args[0]);
        return;
    }

    let mut buf = String::new();
    let mut f = File::open(&args[1]).expect("failed to open file");
    f.read_to_string(&mut buf).expect("failed to read file");

    let tokens = run_lexer(&buf);
    println!("{tokens:?}");
    let output = run_parser(&tokens);

    f = File::create(&args[2]).expect("failed to open output file");
    // f.write_all(output.as_bytes())
    //     .expect("failed to write result to file");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RawInstruction {
    Clear,
    Ret,
    Jmp,
    Call,
    Ske,
    Skn,
    Mov,
    Add,
    Or,
    And,
    Xor,
    Sub1,
    Sub2,
    Shr,
    Shl,
    Jo,
    Rand,
    Draw,
    Skk,
    Sknk,
    Key,
    Bcd,
    Store,
    Load,
}

impl TryFrom<&str> for RawInstruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "clear" => Ok(Self::Clear),
            "ret" => Ok(Self::Ret),
            "jmp" => Ok(Self::Jmp),
            "call" => Ok(Self::Call),
            "ske" => Ok(Self::Ske),
            "skn" => Ok(Self::Skn),
            "mov" => Ok(Self::Mov),
            "add" => Ok(Self::Add),
            "or" => Ok(Self::Or),
            "and" => Ok(Self::And),
            "xor" => Ok(Self::Xor),
            "sub1" => Ok(Self::Sub1),
            "sub2" => Ok(Self::Sub2),
            "shr" => Ok(Self::Shr),
            "shl" => Ok(Self::Shl),
            "jo" => Ok(Self::Jo),
            "rand" => Ok(Self::Rand),
            "draw" => Ok(Self::Draw),
            "skk" => Ok(Self::Skk),
            "sknk" => Ok(Self::Sknk),
            "key" => Ok(Self::Key),
            "bcd" => Ok(Self::Bcd),
            "store" => Ok(Self::Store),
            "load" => Ok(Self::Load),
            _ => Err(format!("unknown instruction: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RawRegister {
    Reg(Register),
    I,
    Delay,
    Sound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Ins(RawInstruction),
    Reg(RawRegister),
    Val(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenInfo {
    token: Token,
    line: usize,
    original_line: String,
    original_text: String,
}

fn token_panic(token_info: &TokenInfo, message: &str) -> ! {
    panic!(
        "error at line {}: `{}`: `{}`: {message}",
        token_info.line, token_info.original_line, token_info.original_text
    );
}

fn parse_register(reg: &str) -> Result<RawRegister, String> {
    match reg.to_lowercase().as_str() {
        "$i" => Ok(RawRegister::I),
        "$s" => Ok(RawRegister::Sound),
        "$d" => Ok(RawRegister::Delay),
        "$v0" => Ok(RawRegister::Reg(Register::V0)),
        "$v1" => Ok(RawRegister::Reg(Register::V1)),
        "$v2" => Ok(RawRegister::Reg(Register::V2)),
        "$v3" => Ok(RawRegister::Reg(Register::V3)),
        "$v4" => Ok(RawRegister::Reg(Register::V4)),
        "$v5" => Ok(RawRegister::Reg(Register::V5)),
        "$v6" => Ok(RawRegister::Reg(Register::V6)),
        "$v7" => Ok(RawRegister::Reg(Register::V7)),
        "$v8" => Ok(RawRegister::Reg(Register::V8)),
        "$v9" => Ok(RawRegister::Reg(Register::V9)),
        "$va" => Ok(RawRegister::Reg(Register::VA)),
        "$vb" => Ok(RawRegister::Reg(Register::VB)),
        "$vc" => Ok(RawRegister::Reg(Register::VC)),
        "$vd" => Ok(RawRegister::Reg(Register::VD)),
        "$ve" => Ok(RawRegister::Reg(Register::VE)),
        "$vf" => Ok(RawRegister::Reg(Register::VF)),
        _ => Err(format!("{reg} is not a register")),
    }
}

/// Try to convert the given string to a number.
/// Supports hex (0x123), binary (0b111), and base 10 (123).
fn str_to_num(addr: &str) -> Result<usize, String> {
    if addr.contains("0x") {
        match usize::from_str_radix(&addr[2..], 16) {
            Ok(val) => Ok(val),
            Err(e) => Err(format!("could not parse hex value {addr}: {e}")),
        }
    } else if addr.contains("0b") {
        match usize::from_str_radix(&addr[2..], 2) {
            Ok(val) => Ok(val),
            Err(e) => Err(format!("could not parse binary value {addr}: {e}")),
        }
    } else {
        match addr.parse::<usize>() {
            Ok(val) => Ok(val),
            Err(e) => Err(format!("could not parse base 10 value {addr}: {e}")),
        }
    }
}

fn run_lexer(buf: &str) -> Vec<Vec<TokenInfo>> {
    buf.split('\n')
        .filter_map(|mut line| {
            // Remove comments
            if let Some(idx) = line.find(';') {
                line = &line[..idx];
            }

            // Split line on whitespace to separate tokens
            let split = line.replace(',', " ");
            let split: Vec<&str> = split.split_whitespace().collect();
            if split.is_empty() {
                None
            } else {
                let v = split
                    .iter()
                    .enumerate()
                    .map(|(i, &s)| {
                        if i == 0 {
                            TokenInfo {
                                token: Token::Ins(
                                    RawInstruction::try_from(s)
                                        .map_err(|e| {
                                            panic!(
                                                "error at line {i}: `{line}`: `{s}`: failed to parse instruction: {e}",
                                            );
                                        })
                                        .unwrap(),
                                ),
                                line: i,
                                original_line: line.to_string(),
                                original_text: s.to_string(),
                            }
                        } else {
                            match s.chars().next() {
                                Some('$') => TokenInfo {
                                    token: Token::Reg(
                                        parse_register(s)
                                            .map_err(|e| {
                                                panic!(
                                                    "error at line {i}: `{line}`: `{s}`: failed to parse register: {e}",
                                                );
                                            })
                                            .unwrap(),
                                    ),
                                    line: i,
                                    original_line: line.to_string(),
                                    original_text: s.to_string(),
                                },
                                Some(_) => TokenInfo {
                                    token: Token::Val(
                                        str_to_num(s)
                                            .map_err(|e| {
                                                panic!(
                                                    "error at line {i}: `{line}`: `{s}`: failed to parse number: {e}",
                                                );
                                            })
                                            .unwrap(),
                                    ),
                                    line: i,
                                    original_line: line.to_string(),
                                    original_text: s.to_string(),
                                },
                                _ => panic!(
                                    "error at line {i}: `{line}`: `{s}`: unrecognized token",
                                )
,
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                Some(v)
            }
        })
        .collect()
}

fn run_parser(tokens: &[Vec<TokenInfo>]) -> Vec<Instruction> {
    tokens.iter().filter_map(|line| parse_line(line)).collect()
}

#[allow(clippy::too_many_lines)]
fn parse_line(line: &[TokenInfo]) -> Option<Instruction> {
    if line.is_empty() {
        return None;
    }
    let ins = line.first().unwrap();

    let args: Vec<&TokenInfo> = line.iter().skip(1).collect();

    let raw_ins = match ins.token {
        Token::Ins(i) => i,
        Token::Reg(_) => token_panic(ins, "expected instruction, found register"),
        Token::Val(_) => token_panic(ins, "expected instruction, found value"),
    };

    match raw_ins {
        RawInstruction::Clear => {
            if !args.is_empty() {
                token_panic(args[0], "instruction 'clear' takes no arguments");
            }
            Some(Instruction::Clear)
        }
        RawInstruction::Ret => {
            if !args.is_empty() {
                token_panic(args[0], "instruction 'ret' takes no arguments");
            }
            Some(Instruction::SubroutineReturn)
        }
        RawInstruction::Jmp | RawInstruction::Call => {
            if args.is_empty() {
                token_panic(ins, "missing argument 'addr'");
            }
            if args.len() > 1 {
                token_panic(args[2], "unexpected argument");
            }
            match args[1].token {
                Token::Val(val) => match raw_ins {
                    RawInstruction::Jmp => Some(Instruction::Jump(
                        validate_addr(val, 0x0FFF, 12).expect("failed to parse address"),
                    )),
                    RawInstruction::Call => Some(Instruction::SubroutineCall(
                        validate_addr(val, 0x0FFF, 12).expect("failed to parse address"),
                    )),
                    _ => panic!("should never happen"),
                },
                Token::Reg(_) => token_panic(args[1], "expected value, found register"),

                Token::Ins(_) => token_panic(args[1], "expected value, found instruction"),
            }
        }
        RawInstruction::Ske | RawInstruction::Skn => {
            if args.len() < 2 {
                token_panic(args[0], "not enough arguments");
            }
            if args.len() > 2 {
                token_panic(args[2], "unexpected argument");
            }
            match args[0].token {
                Token::Ins(_) => token_panic(args[0], "expected register, found instruction"),
                Token::Val(_) => token_panic(args[0], "expected register, found value"),
                Token::Reg(reg) => {
                    let RawRegister::Reg(reg) = reg else {
                        token_panic(args[0], "invalid register for 'ske' operation")
                    };
                    match args[1].token {
                        Token::Ins(_) => {
                            token_panic(
                                args[1],
                                "only general-purpose registers (VX) are allowed in this operation",
                            );
                        }
                        Token::Reg(reg2) => {
                            let RawRegister::Reg(reg2) = reg2 else {
                                token_panic(args[0], "invalid register for 'ske' operation")
                            };
                            match raw_ins {
                                RawInstruction::Ske => {
                                    Some(Instruction::SkipConditional3(reg, reg2))
                                }
                                RawInstruction::Skn => {
                                    Some(Instruction::SkipConditional4(reg, reg2))
                                }
                                _ => panic!("should never happen"),
                            }
                        }
                        Token::Val(v) => match raw_ins {
                            RawInstruction::Ske => Some(Instruction::SkipConditional1(
                                reg,
                                validate_addr(v, 0xFF, 8).expect("failed to parse address"),
                            )),
                            RawInstruction::Skn => Some(Instruction::SkipConditional2(
                                reg,
                                validate_addr(v, 0xFF, 8).expect("failed to parse address"),
                            )),
                            _ => panic!("should never happen"),
                        },
                    }
                }
            }
        }
        RawInstruction::Mov => {
            if args.is_empty() {
                token_panic(ins, "'mov' instruction missing argument: dst");
            }
            if args.len() == 1 {
                token_panic(ins, "'mov' instruction missing argument: src");
            }
            if args.len() > 2 {
                token_panic(args[2], "unexpected argument");
            }
            let dst = args[0];
            let src = args[1];

            match dst.token {
                // Moving into a register
                Token::Reg(dst_reg) => match dst_reg {
                    // Valid sources: vx, u8, d
                    RawRegister::Reg(vx) => match dst.token {
                        Token::Reg(src_reg) => match src_reg {
                            RawRegister::Reg(vy) => Some(Instruction::RegSet(vx, vy)),
                            RawRegister::Delay => Some(Instruction::GetDelayTimer(vx)),
                            RawRegister::Sound => token_panic(src, "invalid source register: $s"),
                            RawRegister::I => token_panic(src, "invalid source register: $i"),
                        },
                        Token::Val(val) => Some(Instruction::SetRegister(
                            vx,
                            validate_addr(val, 0xFF, 8).expect("failed to parse address"),
                        )),
                        Token::Ins(_) => {
                            token_panic(src, "expected value or register, found instruction")
                        }
                    },
                    // Valid options: u12
                    RawRegister::I => match src.token {
                        Token::Val(nnn) => Some(Instruction::SetIndexRegister(
                            validate_addr(nnn, 0x0FFF, 12).expect("failed to parse address"),
                        )),
                        Token::Ins(_) => token_panic(src, "expected value, found instruction"),
                        Token::Reg(_) => token_panic(src, "expected value, found register"),
                    },
                    // Valid options: vx
                    RawRegister::Delay => match src.token {
                        Token::Reg(reg) => match reg {
                            RawRegister::Reg(vx) => Some(Instruction::SetDelayTimer(vx)),
                            RawRegister::I | RawRegister::Delay | RawRegister::Sound => {
                                token_panic(src, "invalid register for dst of $d")
                            }
                        },
                        Token::Ins(_) => token_panic(src, "expected register, found instruction"),
                        Token::Val(_) => token_panic(src, "expected register, found value"),
                    },
                    // Valid options: vx
                    RawRegister::Sound => match src.token {
                        Token::Reg(reg) => match reg {
                            RawRegister::Reg(vx) => Some(Instruction::SetSoundTimer(vx)),
                            RawRegister::I | RawRegister::Delay | RawRegister::Sound => {
                                token_panic(src, "invalid register for dst of $s")
                            }
                        },
                        Token::Ins(_) => token_panic(src, "expected register, found instruction"),
                        Token::Val(_) => token_panic(src, "expected register, found value"),
                    },
                },
                _ => token_panic(dst, "invalid dest for 'mov'"),
            }
        }
        RawInstruction::Add => todo!(),
        RawInstruction::Or => todo!(),
        RawInstruction::And => todo!(),
        RawInstruction::Xor => todo!(),
        RawInstruction::Sub1 => todo!(),
        RawInstruction::Sub2 => todo!(),
        RawInstruction::Shr => todo!(),
        RawInstruction::Shl => todo!(),
        RawInstruction::Jo => todo!(),
        RawInstruction::Rand => todo!(),
        RawInstruction::Draw => todo!(),
        RawInstruction::Skk => todo!(),
        RawInstruction::Sknk => todo!(),
        RawInstruction::Key => todo!(),
        RawInstruction::Bcd => todo!(),
        RawInstruction::Store => todo!(),
        RawInstruction::Load => todo!(),
    }
}

fn validate_addr<T>(val: usize, mask: T, n_bits: usize) -> Result<T, String>
where
    T: Copy + Into<usize>,
{
    if val & mask.into() == val {
        Ok(mask)
    } else {
        Err(format!("value must be less than {n_bits} bits"))
    }
}
