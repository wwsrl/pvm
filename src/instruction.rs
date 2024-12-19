const OPERATION_MASK: u16 = 0x000F;
const LOAD_TYPE_MASK: u16 = 0x0030;
const LOAD_LENGTH_MASK: u16 = 0x00C0;
const SOURCE_MASK: u16 = 0x0F00;
const DESTINATION_MASK: u16 = 0xF000;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InstructionType {
    Nop,
    Invalid,
    Operation,
    Load,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Operation {
    Sum,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Nand,
    Nor,
    Xor,
    Bsl,
    Bsr,
    Rlr,
    PopStack,
    PushStack,
    SwInterrupt,
    Nop,
    Invalid,
}

impl From<u8> for Operation {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Sum,
            1 => Self::Subtract,
            2 => Self::Multiply,
            3 => Self::Divide,
            4 => Self::And,
            5 => Self::Or,
            6 => Self::Nand,
            7 => Self::Nor,
            8 => Self::Xor,
            9 => Self::Bsl,
            10 => Self::Bsr,
            11 => Self::Rlr,
            12 => Self::PopStack,
            13 => Self::PushStack,
            14 => Self::SwInterrupt,
            15 => Self::Nop,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Register {
    A,
    X,
    Y,
    B,
    W,
    Z,
    C,
    D,
    Pc,
    Sb,
    Sp,
    Fl,
    Reserved,
    Reserved1,
    Reserved2,
    None,
    Invalid,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::A,
            1 => Self::X,
            2 => Self::Y,
            3 => Self::B,
            4 => Self::W,
            5 => Self::Z,
            6 => Self::C,
            7 => Self::D,
            8 => Self::Pc,
            9 => Self::Sb,
            10 => Self::Sp,
            11 => Self::Fl,
            12 => Self::Reserved,
            13 => Self::Reserved1,
            14 => Self::Reserved2,
            15 => Self::None,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LoadType {
    Register,
    Ram,
    Immediate,
    None,
    Invalid,
}

impl From<u8> for LoadType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Register,
            1 => Self::Ram,
            2 => Self::Immediate,
            3 => Self::None,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LoadSize {
    Byte,
    Word,
    DWord,
    QWord,
    Invalid,
}

impl From<u8> for LoadSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Byte,
            1 => Self::Word,
            2 => Self::DWord,
            3 => Self::QWord,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Instruction {
    operation: Operation,
    load_type: LoadType,
    load_size: LoadSize,
    source: Register,
    destination: Register,
    family: InstructionType,
}

impl Instruction {
    pub fn parse_instruction(instruction: u16) -> Self {
        if instruction == 0xFFFF {
            return Self {
                operation: Operation::Nop,
                load_type: LoadType::None,
                load_size: LoadSize::QWord,
                source: Register::None,
                destination: Register::None,
                family: InstructionType::Nop,
            };
        } else if instruction == 0 {
            return Self {
                operation: Operation::Invalid,
                load_type: LoadType::Invalid,
                load_size: LoadSize::Invalid,
                source: Register::Invalid,
                destination: Register::Invalid,
                family: InstructionType::Invalid,
            };
        }

        let parts = Self::split_bits(instruction);

        let operation = Operation::from(parts.0);
        let load_type = LoadType::from(parts.1);
        let load_size = LoadSize::from(parts.2);
        let source = Register::from(parts.3);
        let destination = Register::from(parts.4);

        let family = if (operation != Operation::Nop && load_type != LoadType::None)
            || (operation == Operation::Nop && load_type == LoadType::None)
            || (source == destination)
            || (load_type != LoadType::Immediate && load_size != LoadSize::QWord)
        {
            InstructionType::Invalid
        } else if load_type != LoadType::None {
            InstructionType::Load
        } else {
            InstructionType::Operation
        };

        Self {
            operation,
            load_type,
            load_size,
            source,
            destination,
            family,
        }
    }

    pub fn split_bits(instruction: u16) -> (u8, u8, u8, u8, u8) {
        let operation = (instruction & OPERATION_MASK) as u8;
        let load_type = ((instruction & LOAD_TYPE_MASK) >> 4) as u8;
        let load_size = ((instruction & LOAD_LENGTH_MASK) >> 6) as u8;
        let source = ((instruction & SOURCE_MASK) >> 8) as u8;
        let destination = ((instruction & DESTINATION_MASK) >> 12) as u8;

        (operation, load_type, load_size, source, destination)
    }

    pub fn operation(&self) -> Operation {
        self.operation
    }

    pub fn load_type(&self) -> LoadType {
        self.load_type
    }

    pub fn load_size(&self) -> LoadSize {
        self.load_size
    }

    pub fn source(&self) -> Register {
        self.source
    }

    pub fn destination(&self) -> Register {
        self.destination
    }

    pub fn family(&self) -> InstructionType {
        self.family.clone()
    }
}
