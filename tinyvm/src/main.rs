// The operation list composed of Instructions.
type Program<'a> = &'a [u8];

#[derive(Debug)]
enum Instruction {
    PUSH,
    POP,
    ADD,
    SUB,
    INCR,
    DECR,
    MUL,
    DIV,
    JUMP,
    RETURN,
}
impl Instruction {
    fn from_code(value: &u8) -> Option<Self> {
        match value {
            0x1 => Some(Instruction::PUSH),
            0x2 => Some(Instruction::POP),
            0x3 => Some(Instruction::ADD),
            0x4 => Some(Instruction::SUB),
            0x5 => Some(Instruction::INCR),
            0x6 => Some(Instruction::DECR),
            0x7 => Some(Instruction::MUL),
            0x8 => Some(Instruction::DIV),
            0x9 => Some(Instruction::JUMP),
            0xa => Some(Instruction::RETURN),
            _ => None,
        }
    }
}

struct Stack(Vec<u64>);

impl Stack {
    fn push(&mut self, v: u64) {
        self.0.push(v);
    }

    fn pop(&mut self) -> u64 {
        self.0.pop().unwrap()
    }

    fn last(&mut self) -> &mut u64 {
        self.0.last_mut().unwrap()
    }
}

fn interpret<'a>(program: Program<'a>) {
    use Instruction::*;

    let mut stack: Stack = Stack(Vec::new());
    let mut pointer: usize = 0;

    while let Some(instruction) = Instruction::from_code(program.get(pointer).unwrap()) {
        pointer += 1;

        match instruction {
            // pushes a value to the top of the stack.
            PUSH => {
                stack.push(*program.get(pointer).unwrap() as u64);
                pointer += 1;
            }

            // removes a value from the top of the stack.
            POP => {
                stack.pop();
            }

            ADD => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a + b)
            }

            SUB => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b - a)
            }

            MUL => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a * b)
            }
            DIV => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b / a)
            }

            INCR => *stack.last() += 1,
            DECR => *stack.last() -= 1,

            JUMP => pointer = *program.get(pointer).unwrap() as usize,

            RETURN => {
                let value = stack.pop() as usize;
                println!("> RETURN = {:?}", value);
                break;
            }
        }
    }
}

fn main() {
    println!("Start");
    let program: Vec<u8> = vec![0x1, 0x2, 0x1, 0x3, 0x3, 0xa];
    interpret(&program[..]);
    println!("End");
}
