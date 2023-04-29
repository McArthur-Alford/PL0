use std::collections::HashMap;

mod parser;
mod token;

#[derive(Debug, Clone, Copy)]
enum OPCODE {
    ADD,
    SUB,
    MUL,
    POP,
    JMP,
    LDC(i32),
    STF, // Store Frame
    LDF, // Load Frame
    BRT, // Branch True
    BRF, // Branch False
    WRT, // Write
    HALT
}

struct StackMachine {
    Stack: Vec<i32>,
    Variables: HashMap<i32, i32>,
    Program: Vec<OPCODE>,
    PC: usize
}

#[derive(Clone)]
struct StepResult {
    Location: usize,
    OpCode: OPCODE,
    Result: Result<Option<i32>, String>
}

impl StackMachine {
    fn new() -> Self {
        Self {
            Stack: Vec::new(),
            Variables: HashMap::new(),
            Program: Vec::new(),
            PC: 0
        }
    }

    fn load(&mut self, program: Vec<OPCODE>) {
        self.Program = program;
        self.Stack.clear();
        self.PC = 0;
    }

    fn step(&mut self) -> Result<Option<i32>, String> {
        match self.Program.get(self.PC).ok_or("Program Counter out of bounds")? {
            OPCODE::ADD => {
                let a = self.Stack.pop().ok_or("Stack Empty")?;
                let b = self.Stack.pop().ok_or("Stack Empty")?;
                self.Stack.push(a + b);
                self.PC += 1;
                Ok(None)
            },
            OPCODE::SUB => {
                let a = self.Stack.pop().ok_or("Stack Empty")?;
                let b = self.Stack.pop().ok_or("Stack Empty")?;
                self.Stack.push(a - b);
                self.PC += 1;
                Ok(None)
            },
            OPCODE::MUL => {
                let a = self.Stack.pop().ok_or("Stack Empty")?;
                let b = self.Stack.pop().ok_or("Stack Empty")?;
                self.Stack.push(a * b);
                self.PC += 1;
                Ok(None)
            },
            OPCODE::POP => {
                let out = self.Stack.pop().ok_or("Stack Empty")?;
                self.PC += 1;
                Ok(Some(out))
            },
            OPCODE::JMP => {
                let offset = self.Stack.pop().ok_or("Stack Empty")?;
                let newPC = (self.PC as i32 + offset);
                if newPC >= self.Program.len() as i32 || newPC < 0{
                    return Err("Program Counter out of bounds".to_string());
                }
                self.PC = newPC as usize;
                Ok(None)
            },
            OPCODE::LDC(x) => {
                self.Stack.push(*x);
                self.PC += 1;
                Ok(None)
            },
            OPCODE::STF => {
                let key = self.Stack.pop().ok_or("Stack Empty")?;
                let value = self.Stack.pop().ok_or("Stack Empty")?;
                self.Variables.insert(key, value);
                self.PC += 1;
                Ok(None)
            },
            OPCODE::LDF => {
                let key = self.Stack.pop().ok_or("Stack Empty")?;
                let value = self.Variables.get(&key).ok_or("Variable not found")?;
                self.Stack.push(*value);
                self.PC += 1;
                Ok(None)
            },
            OPCODE::BRT => {
                let offset = self.Stack.pop().ok_or("Stack Empty")?;
                let condition = self.Stack.pop().ok_or("Stack Empty")?;
                if condition != 0 {
                    let newPC = (self.PC as i32 + offset);
                    if newPC >= self.Program.len() as i32 || newPC < 0{
                        return Err("Program Counter out of bounds".to_string());
                    }
                    self.PC = newPC as usize;
                } else {
                    self.PC += 1;
                }
                Ok(None)
            },
            OPCODE::BRF => {
                let offset = self.Stack.pop().ok_or("Stack Empty")?;
                let condition = self.Stack.pop().ok_or("Stack Empty")?;
                if condition == 0 {
                    let newPC = (self.PC as i32 + offset);
                    if newPC >= self.Program.len() as i32 || newPC < 0{
                        return Err("Program Counter out of bounds".to_string());
                    }
                    self.PC = newPC as usize;
                } else {
                    self.PC += 1;
                }
                Ok(None)
            },
            OPCODE::WRT => {
                let value = self.Stack.pop().ok_or("Stack Empty")?;
                print!("{}", value);
                self.PC += 1;
                Ok(Some(value))
            },
            OPCODE::HALT => {
                Err("HALT".to_string())
            }
        }
    }

    fn run(&mut self) -> Vec<StepResult> {
        let mut results = Vec::new();
        loop {
            // Save the current state of the machine as step can modify it
            let pc = self.PC;
            let opcode = self.Program[pc]; // TODO error handling
            let result = self.step();
            results.push(StepResult {
                Location: pc,
                OpCode: opcode,
                Result: result.clone()
            });
            if result.is_err() {
                break;
            }
        }
        results
    }
}

fn print_results(results: Vec<StepResult>) {
    println!("Results: ({} steps)", results.len());
    let output = results.clone().iter()
        .filter_map(|x| x.clone().Result.ok())
        .filter_map(|x| x)
        .map(|x| x.to_string() + "\n")
        .collect::<String>();
    for result in results.clone() {
        println!("Location: {}, OpCode: {:?}, Result: {:?}", result.Location, result.OpCode, result.Result);
    }
    println!("Output: {:?}", output);
    println!("Exited with: {:?}", results.last().unwrap().Result);
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut machine = StackMachine::new();
        machine.load(vec![OPCODE::LDC(1), OPCODE::LDC(2), OPCODE::ADD, OPCODE::POP, OPCODE::HALT]);
        let results = machine.run();
        print_results(results.clone());
        assert!(results.len() == 5);
        assert_eq!(results[0].Result, Ok(None));
        assert_eq!(results[1].Result, Ok(None));
        assert_eq!(results[2].Result, Ok(None));
        assert_eq!(results[3].Result, Ok(Some(3)));
    }

    #[test]
    fn test_sub() {
        let mut machine = StackMachine::new();
        machine.load(vec![OPCODE::LDC(1), OPCODE::LDC(2), OPCODE::SUB, OPCODE::POP, OPCODE::HALT]);
        let results = machine.run();
        print_results(results.clone());
        assert!(results.len() == 5);
        assert_eq!(results[0].Result, Ok(None));
        assert_eq!(results[1].Result, Ok(None));
        assert_eq!(results[2].Result, Ok(None));
        assert_eq!(results[3].Result, Ok(Some(1)));
    }

    #[test]
    fn test_mul() {
        let mut machine = StackMachine::new();
        machine.load(vec![OPCODE::LDC(1), OPCODE::LDC(2), OPCODE::MUL, OPCODE::POP, OPCODE::HALT]);
        let results = machine.run();
        print_results(results.clone());
        assert!(results.len() == 5);
        assert_eq!(results[0].Result, Ok(None));
        assert_eq!(results[1].Result, Ok(None));
        assert_eq!(results[2].Result, Ok(None));
        assert_eq!(results[3].Result, Ok(Some(2)));
    }

    #[test]
    fn test_q2() {
        /**
         * var
         *  x: int; i: int;
         * begin
         *  i := 0;
         *  x := 0;
         *  while i < 5 do
         *   x := x + i;
         *   i := i + 1
         *  end;
         * write x
         * end
         */
        let mut machine = StackMachine::new();
        machine.load(vec![
            OPCODE::LDC(0), OPCODE::LDC(0), OPCODE::STF, // x := 0; at address 0
            OPCODE::LDC(1), OPCODE::LDC(0), OPCODE::STF, // i := 0; at address 1

        ]);
        let results = machine.run();
        print_results(results.clone());

    }
}