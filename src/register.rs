use std::fmt::Display;

/// Registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    V0 = 0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    /// Also used as the flag register.
    VF,
}

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        match value {
            Register::V0 => 0,
            Register::V1 => 1,
            Register::V2 => 2,
            Register::V3 => 3,
            Register::V4 => 4,
            Register::V5 => 5,
            Register::V6 => 6,
            Register::V7 => 7,
            Register::V8 => 8,
            Register::V9 => 9,
            Register::VA => 0xA,
            Register::VB => 0xB,
            Register::VC => 0xC,
            Register::VD => 0xD,
            Register::VE => 0xE,
            Register::VF => 0xF,
        }
    }
}

impl From<Register> for u16 {
    fn from(value: Register) -> Self {
        Self::from(Into::<u8>::into(value))
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::V0,
            1 => Self::V1,
            2 => Self::V2,
            3 => Self::V3,
            4 => Self::V4,
            5 => Self::V5,
            6 => Self::V6,
            7 => Self::V7,
            8 => Self::V8,
            9 => Self::V9,
            0xA => Self::VA,
            0xB => Self::VB,
            0xC => Self::VC,
            0xD => Self::VD,
            0xE => Self::VE,
            0xF => Self::VF,
            _ => panic!("value {value} could not be converted to register"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V0 => f.write_str("V0"),
            Self::V1 => f.write_str("V1"),
            Self::V2 => f.write_str("V2"),
            Self::V3 => f.write_str("V3"),
            Self::V4 => f.write_str("V4"),
            Self::V5 => f.write_str("V5"),
            Self::V6 => f.write_str("V6"),
            Self::V7 => f.write_str("V7"),
            Self::V8 => f.write_str("V8"),
            Self::V9 => f.write_str("V9"),
            Self::VA => f.write_str("VA"),
            Self::VB => f.write_str("VB"),
            Self::VC => f.write_str("VC"),
            Self::VD => f.write_str("VD"),
            Self::VE => f.write_str("VE"),
            Self::VF => f.write_str("VF"),
        }
    }
}
