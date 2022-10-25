fn main() {
    let path = "testing.txt";

    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(m) => {
            eprintln!("Error ({path}): {m}");
            std::process::exit(1)
        }
    };

    let tokens = tokenize(&source);

    println!("{:?}", tokens);

    println!("{}", tokens[0].encode());
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

#[derive(Debug, PartialEq, Clone, Copy)]
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
        Err(msg) => Err(format!("Invalid number literal {x} ({msg})")),
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

    fn encode(&self) -> String {
        use Instruction::*;

        match self {
            Store(reg, addr) => format!("00001{:03b}\n{:08b}", *reg as u8, *addr),
            _ => todo!(),
        }

        // match self {
        //     Store(reg, addr) => format!("{:b}\n{:08b}", 0b0001, *reg as u8, *addr),
        //     Load(reg, addr) => format!("{:b}{:08b}\n{:08b}", 0b00010, *reg as u8, *addr),
        //     Mov(reg1, reg2) => {
        //         format!("{:08b}{:08b}\n{:08b}", 0b10100, *reg1 as u8, *reg2 as u8)
        //     }
        //     Add => format!("{:08b}", 0b00011000),
        //     Sub => vec![0b00100],
        //     Rotla => vec![0b00101000],
        //     Rotra => vec![0b00110001],
        //     Rotlb => vec![],
        //     Rotrb => vec![],
        //     Inc => vec![],
        //     Dec => vec![],
        //     Xor => vec![],
        //     And => vec![],
        //     Not => vec![],
        //     Or => vec![],
        //     Push(reg) => vec![0b01101, *reg as u8],
        //     Pop(reg) => vec![0b01110, *reg as u8],
        //     Halt => vec![0b01111],
        //     In(reg, addr) => vec![0b10000, *reg as u8, *addr],
        //     Out(reg, addr) => vec![0b10110, *reg as u8, *addr],
        //     Jumpif(cond, addr) => {
        //         use Condition::*;
        //         match cond {
        //             Positive | Zero | Carry | Negative | Even | Odd | Always => {
        //                 todo!()
        //             }
        //             NotZero | NotCarry | NotOverflow => {
        //                 let code = *cond as u8 - 8;
        //                 println!("{code:b}");
        //                 todo!()
        //             }
        //         }
        //     }
        //     Assign(_, _) => vec![],
        // }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Condition {
    Positive = 0b10110000,
    Zero = 0b10010101,
    Carry = 0b10010100,
    Negative = 0b10010011,
    Even = 0b10010010,
    Odd = 0b10010001,
    Always = 0b10010000,
    NotZero = 0b11000000,
    NotCarry = 0b11000001,
    NotOverflow = 0b11000010,
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
