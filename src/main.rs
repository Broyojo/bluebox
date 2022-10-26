use std::fs;

fn main() {
    let path = "program.txt";

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(m) => {
            eprintln!("Error ({path}): {m}");
            std::process::exit(1)
        }
    };

    let tokens = tokenize(&source);

    let encoded = encode(&tokens);

    tokens
        .iter()
        .zip(encoded.iter())
        .for_each(|(t, e)| println!("{t:?} {e}"));
}

fn tokenize(source: &str) -> Vec<Instruction> {
    let mut tokens = vec![];
    for (i, line) in source.lines().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let instr = match Instruction::from(parts) {
            Ok(i) => i,
            Err(msg) => {
                eprintln!("Error (line {}): {}", i + 1, msg);
                std::process::exit(1)
            }
        };

        tokens.push(instr);
    }
    tokens
}

fn encode(tokens: &[Instruction]) -> Vec<String> {
    tokens.iter().map(|t| t.encode()).collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    O = 69, // works for now, but not a final solution. just making an exception for MOV
}

impl Register {
    fn from(s: &str) -> Result<Self, String> {
        use Register::*;
        match s {
            "A" => Ok(A),
            "B" => Ok(B),
            "C" => Ok(C),
            "D" => Ok(D),
            "E" => Ok(E),
            "F" => Ok(F),
            "O" => Ok(O),
            _ => Err(format!("Unknown Register '{}'", s)),
        }
    }
}

type Address = u8;
type PortAddress = u8;
type Value = u8;

fn parse_num(x: &str) -> Result<u8, String> {
    match x.parse() {
        Ok(x) => Ok(x),
        Err(msg) => Err(format!("Invalid number literal {x} ({msg})")),
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Store(Register, Address),
    Load(Register, Address),
    Mov(Register, Register),
    Add,
    Sub,
    Rotla,
    Rotra,
    Rotlb,
    Rotrb,
    Inc,
    Dec,
    Xor,
    And,
    Not,
    Or,
    Push(Register),
    Pop(Register),
    Halt,
    In(Register, PortAddress),
    Out(Register, PortAddress),
    Jumpif(Condition, PortAddress),
    Assign(Register, Value),
    IO, // TODO: Figure out what this is!!!
}

impl Instruction {
    fn from(s: Vec<&str>) -> Result<Self, String> {
        use Instruction::*;
        // TODO: Add restrictions for A and B registers
        match s[0] {
            "STORE" => Ok(Store(Register::from(s[1])?, parse_num(s[2])?)),
            "LOAD" => Ok(Load(Register::from(s[1])?, parse_num(s[2])?)),
            "MOV" => Ok(Mov(Register::from(s[1])?, Register::from(s[2])?)), // MOV A B : A -> B
            "ADD" => Ok(Add),
            "SUB" => Ok(Sub),
            "ROTLA" => Ok(Rotla),
            "ROTRA" => Ok(Rotra),
            "ROTLB" => Ok(Rotlb),
            "ROTRB" => Ok(Rotrb),
            "INC" => Ok(Inc),
            "DEC" => Ok(Dec),
            "XOR" => Ok(Xor),
            "AND" => Ok(And),
            "NOT" => Ok(Not),
            "OR" => Ok(Or),
            "PUSH" => Ok(Push(Register::from(s[1])?)),
            "POP" => Ok(Pop(Register::from(s[1])?)),
            "HALT" => Ok(Halt),
            "IN" => Ok(In(Register::from(s[1])?, parse_num(s[2])?)),
            "OUT" => Ok(Out(Register::from(s[1])?, parse_num(s[2])?)),
            "JUMPIF" => Ok(Jumpif(Condition::from(s[1])?, parse_num(s[2])?)),
            "ASSIGN" => Ok(Assign(Register::from(s[1])?, parse_num(s[2])?)),
            "IO" => todo!(),
            x => Err(format!("Unknown Opcode '{x}'")),
        }
    }

    fn encode(self) -> String {
        use Instruction::*;

        fn enc_reg_data(op: &str, reg: Register, addr: Address) -> String {
            format!("{op}{:03b} {addr:08b}", reg as u8)
        }

        fn enc_reg_reg(op: &str, reg1: Register, reg2: Register) -> String {
            format!("{op}{:03b} {:08b}", reg1 as u8, reg2 as u8)
        }

        fn enc_reg(op: &str, reg: Register) -> String {
            format!("{op}{:03b}", reg as u8)
        }

        match self {
            Store(reg, addr) => enc_reg_data("00001", reg, addr),
            Load(reg, addr) => enc_reg_data("00010", reg, addr),
            Mov(reg1, reg2) => {
                // TODO: investiage this weird corner case
                if reg1 == Register::O {
                    format!("10100000 {:08b}", reg2 as u8)
                } else {
                    enc_reg_reg("10100", reg1, reg2)
                }
            }
            Add => "00011000".into(),
            Sub => "00100000".into(),
            Rotla => "00101000".into(),
            Rotra => "00110000".into(),
            Rotlb => "00101001".into(),
            Rotrb => "00110001".into(),
            Inc => "00111000".into(),
            Dec => "01000000".into(),
            Xor => "01001000".into(),
            And => "01010000".into(),
            Not => "01011000".into(),
            Or => "01100000".into(),
            Push(reg) => enc_reg("01101", reg),
            Pop(reg) => enc_reg("01110", reg),
            Halt => "01111000".into(),
            In(reg, addr) => enc_reg_data("10000", reg, addr),
            Out(reg, addr) => enc_reg_data("10110", reg, addr),
            Jumpif(cond, addr) => {
                use Condition::*;
                format!(
                    "{} {addr:08b}",
                    match cond {
                        Positive => "10110000",
                        Zero => "10010101",
                        Carry => "10010100",
                        Negative => "10010011",
                        Even => "10010010",
                        Odd => "10010001",
                        Always => "10010000",
                        NotZero => "11000000",
                        NotCarry => "11000001",
                        NotOverflow => "11000010",
                    }
                )
            }
            Assign(reg, val) => enc_reg_data("10101", reg, val),
            IO => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Condition {
    Positive,
    Zero,
    Carry,
    Negative,
    Even,
    Odd,
    Always,
    NotZero,
    NotCarry,
    NotOverflow,
}

impl Condition {
    fn from(s: &str) -> Result<Self, String> {
        use Condition::*;
        match s {
            "POSITIVE" => Ok(Positive),
            "ZERO" => Ok(Zero),
            "CARRY" => Ok(Carry),
            "NEGATIVE" => Ok(Negative),
            "EVEN" => Ok(Even),
            "ODD" => Ok(Odd),
            "ALWAYS" => Ok(Always),
            "NOTZERO" => Ok(NotZero),
            "NOTCARRY" => Ok(NotCarry),
            "NOTOVERFLOW" => Ok(NotOverflow),
            _ => Err(format!("Unknown Condition '{s}'")),
        }
    }
}
