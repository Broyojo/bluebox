use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let path = "program.txt";

    let source = match read_file(path) {
        Ok(s) => s,
        Err(m) => {
            eprintln!("Error ({path}): {m}");
            std::process::exit(1)
        }
    };
    let tokens = tokenize(&source);
    println!("{:?}", tokens);

    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    let text = read_to_string(path)?;
    Ok(text)
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

#[derive(Debug, PartialEq)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    O = 6, // TODO: This is probably not 6, the register ordering is quite weird
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
        Err(msg) => Err(format!("Invalid literal {x} ({msg})")),
    }
}

#[derive(Debug, PartialEq)]
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
}

impl Instruction {
    fn from(s: Vec<&str>) -> Result<Self, String> {
        use Instruction::*;
        // TODO: Add restrictions for A and B registers
        match s[0] {
            "STORE" => Ok(Store(Register::from(s[1])?, parse_num(s[2])?)),
            "LOAD" => Ok(Load(Register::from(s[1])?, parse_num(s[2])?)),
            "MOV" => Ok(Mov(Register::from(s[1])?, Register::from(s[2])?)),
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
            x => Err(format!("Unknown Opcode '{x}'")),
        }
    }
}

#[derive(Debug, PartialEq)]
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
