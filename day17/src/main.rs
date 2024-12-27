use std::io;

#[derive(Debug)]
struct Emulator {
    program: Vec<usize>,
    sp: usize,
    register_a: usize,
    register_b: usize,
    register_c: usize,
    output_tape: Vec<usize>,
}

impl Emulator {
    fn new(program: Vec<usize>, register_a: usize, register_b: usize, register_c: usize) -> Self {
        Self {
            program,
            register_a,
            register_b,
            register_c,
            sp: 0,
            output_tape: vec![],
        }
    }

    fn combo_operand(&self, position: usize) -> usize {
        let combo_operand = self.program[position];
        if combo_operand == 0 || combo_operand == 1 || combo_operand == 2 || combo_operand == 3 {
            return combo_operand;
        } else if combo_operand == 4 {
            return self.register_a;
        } else if combo_operand == 5 {
            return self.register_b;
        } else if combo_operand == 6 {
            return self.register_c;
        } else {
            panic!("valid programs will not use combo_operand {combo_operand}");
        }
    }

    fn fetch_decode_execute(&mut self) -> bool {
        let opcode = self.program[self.sp];

        //ADV
        if opcode == 0 {
            let combo_operand = self.combo_operand(self.sp + 1);
            self.register_a = self.register_a >> combo_operand;
            self.sp = self.sp + 2;
        }
        //BXL
        else if opcode == 1 {
            let literal_operand = self.program[self.sp + 1];
            self.register_b = self.register_b ^ literal_operand;
            self.sp = self.sp + 2;
        }
        // BST
        else if opcode == 2 {
            let combo_operand = self.combo_operand(self.sp + 1) % 8;
            self.register_b = combo_operand;
            self.sp = self.sp + 2;
        }
        // JNZ
        else if opcode == 3 {
            if self.register_a == 0 {
                self.sp = self.sp + 2;
            } else {
                let literal_operand = self.program[self.sp + 1];
                self.sp = literal_operand;
            }
        }
        //BXC
        else if opcode == 4 {
            self.register_b = self.register_b ^ self.register_c;
            self.sp = self.sp + 2;
        }
        //OUT
        else if opcode == 5 {
            let combo_operand = self.combo_operand(self.sp + 1) % 8;
            self.output_tape.push(combo_operand);
            self.sp = self.sp + 2;
        }
        //BDV
        else if opcode == 6 {
            let combo_operand = self.combo_operand(self.sp + 1);
            self.register_b = self.register_a >> combo_operand;
            self.sp = self.sp + 2;
        }
        //CDV
        else if opcode == 7 {
            let combo_operand = self.combo_operand(self.sp + 1);
            self.register_c = self.register_a >> combo_operand;
            self.sp = self.sp + 2;
        }
        true
    }

    fn execute(&mut self) -> bool {
        if self.sp >= self.program.len() {
            return false;
        }
        self.fetch_decode_execute()
    }
}

fn part_2() -> io::Result<()> {
    let content = "Register A: 34615120
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0";
    let (_registers, program) = content.split_once("\n\n").unwrap();
    let program: Vec<usize> = program
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|char| char.parse::<usize>().unwrap())
        .collect();
    fn dfs(a_s: &mut Vec<usize>, program: &Vec<usize>) -> Option<u64> {
        let current_position = program.len() - 1 - a_s.len();
        let mut current_a = 0;
        for (index, a) in a_s.iter().enumerate().rev() {
            current_a += a * 8_usize.pow((program.len() - index) as u32 - 1);
        }
        if a_s.len() == program.len() {
            return Some(current_a as u64);
        }
        for i in 0..8 {
            let mut the_a = current_a >> (current_position) * 3;
            the_a += i;
            let mut b = the_a % 8;
            b = b ^ 5;
            let c = the_a >> b;
            b = b ^ 6;
            b = b ^ c;
            if b % 8 == program[current_position] {
                a_s.push(i);
                if let Some(result) = dfs(a_s, &program) {
                    return Some(result as u64);
                }
                a_s.pop();
            }
        }
        None
    }
    let res = dfs(&mut vec![], &program);
    println!("day 17 part 2 result is : {res:?}");
    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = "Register A: 34615120
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0";
    let (registers, program) = content.split_once("\n\n").unwrap();
    let registers: Vec<usize> = registers
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap().parse::<usize>().unwrap())
        .collect();
    let program: Vec<usize> = program
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|char| char.parse::<usize>().unwrap())
        .collect();

    let mut emulator = Emulator::new(program.clone(), registers[0], registers[1], registers[2]);
    while emulator.execute() {}
    println!("day 17 part 1 result is : {:?}", emulator.output_tape);
    Ok(())
}

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    part_1()?;
    part_2()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}
