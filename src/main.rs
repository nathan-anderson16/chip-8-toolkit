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
        RawInstruction::Jmp | RawInstruction::Call | RawInstruction::Jo => {
            if args.is_empty() {
                token_panic(ins, "missing argument 'addr'");
            }
            if args.len() > 1 {
                token_panic(args[0], "unexpected argument");
            }

            let final_instruction = match raw_ins {
                RawInstruction::Jmp => Instruction::Jump,
                RawInstruction::Call => Instruction::SubroutineCall,
                RawInstruction::Jo => Instruction::JumpOffset,
                _ => panic!("should never happen"),
            };

            match args[0].token {
                Token::Val(nnn) => Some(final_instruction(validate_u12(nnn))),
                Token::Reg(_) => token_panic(args[0], "expected value, found register"),
                Token::Ins(_) => token_panic(args[0], "expected value, found instruction"),
            }
        }
        RawInstruction::Ske | RawInstruction::Skn => {
            validate_two(&args, ins);

            let final_instruction_1 = match raw_ins {
                RawInstruction::Ske => Instruction::SkipConditional3,
                RawInstruction::Skn => Instruction::SkipConditional4,
                _ => panic!("should never happen"),
            };

            let final_instruction_2 = match raw_ins {
                RawInstruction::Ske => Instruction::SkipConditional1,
                RawInstruction::Skn => Instruction::SkipConditional2,
                _ => panic!("should never happen"),
            };

            let vx = validate_token_vx(args[0]);

            match args[1].token {
                Token::Ins(_) => {
                    token_panic(
                        args[1],
                        "only general-purpose registers (VX) are allowed in this operation",
                    );
                }
                Token::Reg(reg) => {
                    let vy = validate_vx(args[1], reg);
                    Some(final_instruction_1(vx, vy))
                }
                Token::Val(nn) => {
                    let nn = validate_u8(nn);
                    Some(final_instruction_2(vx, nn))
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
                        Token::Val(nn) => Some(Instruction::SetRegister(vx, validate_u8(nn))),
                        Token::Ins(_) => {
                            token_panic(src, "expected value or register, found instruction")
                        }
                    },
                    // Valid options: u12
                    RawRegister::I => {
                        let nnn = validate_token_nnn(src);
                        Some(Instruction::SetIndexRegister(nnn))
                    }
                    // Valid options: vx
                    RawRegister::Delay => {
                        let vx = validate_token_vx(src);
                        Some(Instruction::SetDelayTimer(vx))
                    }
                    // Valid options: vx
                    RawRegister::Sound => {
                        let vx = validate_token_vx(src);
                        Some(Instruction::SetSoundTimer(vx))
                    }
                },
                _ => token_panic(dst, "invalid dest for 'mov'"),
            }
        }
        RawInstruction::Add => {
            let (dst, src) = validate_two(&args, ins);

            match dst.token {
                Token::Reg(dst_reg) => match dst_reg {
                    // Valid options: vx, u8
                    RawRegister::Reg(vx) => match src.token {
                        Token::Reg(src_reg) => {
                            let vy = validate_vx(src, src_reg);
                            Some(Instruction::RegAdd(vx, vy))
                        }
                        Token::Val(nn) => Some(Instruction::Add(vx, validate_u8(nn))),
                        Token::Ins(_) => {
                            token_panic(src, "expected register or value, found instruction")
                        }
                    },
                    // Valid options: vx
                    RawRegister::I => {
                        let vx = validate_token_vx(src);
                        Some(Instruction::AddToIndex(vx))
                    }
                    _ => token_panic(
                        dst,
                        "only general-purpose registers (VX) or I are allowed in this operation",
                    ),
                },
                Token::Ins(_) => token_panic(src, "expected register, found instruction"),
                Token::Val(_) => token_panic(src, "expected register, found value"),
            }
        }
        RawInstruction::Or
        | RawInstruction::And
        | RawInstruction::Xor
        | RawInstruction::Sub1
        | RawInstruction::Sub2
        | RawInstruction::Shr
        | RawInstruction::Shl => {
            let (dst, src) = validate_two(&args, ins);

            let final_instruction = match raw_ins {
                RawInstruction::Or => Instruction::BinaryOr,
                RawInstruction::And => Instruction::BinaryAnd,
                RawInstruction::Xor => Instruction::BinaryXor,
                RawInstruction::Sub1 => Instruction::Subtract1,
                RawInstruction::Sub2 => Instruction::Subtract2,
                RawInstruction::Shr => Instruction::ShiftRight,
                RawInstruction::Shl => Instruction::ShiftLeft,
                _ => panic!("should never happen"),
            };

            let vx = validate_token_vx(dst);
            let vy = validate_token_vx(src);

            Some(final_instruction(vx, vy))
        }
        RawInstruction::Rand => {
            let (dst, src) = validate_two(&args, ins);

            let vx = validate_token_vx(dst);
            let nn = validate_token_nn(src);

            Some(Instruction::Random(vx, nn))
        }
        RawInstruction::Draw => {
            if args.len() < 3 {
                token_panic(ins, "not enough arguments for instruction 'draw'");
            }
            if args.len() > 3 {
                token_panic(args[3], "unexpected argument");
            }

            let vx = validate_token_vx(args[0]);
            let vy = validate_token_vx(args[1]);
            let n = validate_token_n(args[2]);

            Some(Instruction::Draw(vx, vy, n))
        }
        RawInstruction::Skk | RawInstruction::Sknk | RawInstruction::Key | RawInstruction::Bcd => {
            todo!()
        }
        RawInstruction::Store | RawInstruction::Load => todo!(),
    }
}

fn validate_token_n(token_info: &TokenInfo) -> u8 {
    match token_info.token {
        Token::Val(n) => validate_u4(n),
        Token::Ins(_) => token_panic(token_info, "expected value, found instruction"),
        Token::Reg(_) => token_panic(token_info, "expected value, found register"),
    }
}

fn validate_token_nn(token_info: &TokenInfo) -> u8 {
    match token_info.token {
        Token::Val(nn) => validate_u8(nn),
        Token::Ins(_) => token_panic(token_info, "expected value, found instruction"),
        Token::Reg(_) => token_panic(token_info, "expected value, found register"),
    }
}

fn validate_token_nnn(token_info: &TokenInfo) -> u16 {
    match token_info.token {
        Token::Val(nnn) => validate_u12(nnn),
        Token::Ins(_) => token_panic(token_info, "expected value, found instruction"),
        Token::Reg(_) => token_panic(token_info, "expected value, found register"),
    }
}

fn validate_token_vx(token_info: &TokenInfo) -> Register {
    match token_info.token {
        Token::Reg(reg) => validate_vx(token_info, reg),
        Token::Ins(_) => token_panic(token_info, "expected register, found instruction"),
        Token::Val(_) => token_panic(token_info, "expected register, found value"),
    }
}

fn validate_vx(token_info: &TokenInfo, reg: RawRegister) -> Register {
    match reg {
        RawRegister::Reg(vx) => vx,
        _ => token_panic(
            token_info,
            "only general-purpose registers (VX) are allowed here",
        ),
    }
}

/// Validate that args contains two arguments, and return those arguments.
fn validate_two<'a>(args: &[&'a TokenInfo], ins: &TokenInfo) -> (&'a TokenInfo, &'a TokenInfo) {
    if args.len() < 2 {
        token_panic(ins, "not enough arguments");
    }
    if args.len() > 2 {
        token_panic(args[2], "unexpected argument");
    }

    (args[0], args[1])
}

fn validate_u4(val: usize) -> u8 {
    validate_addr(val, 0x0F, 4).expect("failed to parse address")
}

fn validate_u8(val: usize) -> u8 {
    validate_addr(val, 0xFF, 8).expect("failed to parse address")
}

fn validate_u12(val: usize) -> u16 {
    validate_addr(val, 0x0FFF, 12).expect("failed to parse address")
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
