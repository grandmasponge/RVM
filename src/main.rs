use core::fmt;
use std::{error::Error, fmt::format};

#[derive(Debug)]
struct Cpu {
    a: u8,
    b: u8,
    ic: usize,
    stack: Vec<u8>,
    ram: [u8; 0xFFFF],
    rom: [u8; 0xFFFF],
}

trait Instructions {
    fn add(&mut self) -> Result<(), CpuError>;
    fn sub(&mut self) -> Result<(), CpuError>;
    fn mult(&mut self) -> Result<(), CpuError>;
    fn div(&mut self) -> Result<(), CpuError>;
    fn mov(&mut self) -> Result<(), CpuError>;
    fn jmp(&mut self) -> Result<(), CpuError>;
    fn jmp_if_a(&mut self) -> Result<(), CpuError>;
    fn jmp_if_b(&mut self) -> Result<(), CpuError>;
    fn push(&mut self) -> Result<(), CpuError>;
    fn pop(&mut self) -> Result<(), CpuError>;
    fn print(&mut self) -> Result<(), CpuError>;
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            a: 0,
            b: 0,
            ic:0,
            stack: Vec::new(),
            ram: [0; 0xFFFF],
            rom: [0; 0xFFFF],
        }
    }

    fn counter_register_pos(&self) -> usize {
        self.ic
    }

    fn load_rom(&mut self, rom: [u8; 0xFFFF]) {
        self.rom = rom;
    }

    fn load_ram(&mut self, ram: [u8; 0xFFFF]) {
        self.ram = ram;
    }

}

impl Instructions for Cpu {
    fn add(&mut self) -> Result<(), CpuError> {
        let mut current_pos = self.counter_register_pos();
        let a = self.ram[current_pos];
        let reg: Result<u8, CpuError> = match a {
            1 => Ok(self.a),
            2 => Ok(self.b),
            _ => Err(CpuError)
        };
        self.ic += 1;
        current_pos = self.counter_register_pos();
        let number = self.ram[current_pos];
        self.a = reg.expect("failed to get register") + number;
        Ok(())
    }
    
    fn sub(&mut self) -> Result<(), CpuError> {
        let mut current_pos = self.counter_register_pos();
        let a = self.ram[current_pos];
        let reg: Result<u8, CpuError> = match a {
            1 => Ok(self.a),
            2 => Ok(self.b),
            _ => Err(CpuError)
        };
        self.ic += 1;
        current_pos = self.counter_register_pos();
        let number = self.ram[current_pos];
        self.a = reg.expect("failed to get register") - number;
        Ok(())
    }
    fn mult(&mut self) -> Result<(), CpuError> {
        let mut current_pos = self.counter_register_pos();
        let a = self.ram[current_pos];
        let reg: Result<u8, CpuError> = match a {
            1 => Ok(self.a),
            2 => Ok(self.b),
            _ => Err(CpuError)
        };
        self.ic += 1;
        current_pos = self.counter_register_pos();
        let number = self.ram[current_pos];
        self.a = reg.expect("failed to get register") * number;
        Ok(())
    }
    fn div(&mut self) -> Result<(), CpuError> {
        let mut current_pos = self.counter_register_pos();
        let a = self.ram[current_pos];
        let reg: Result<u8, CpuError> = match a {
            1 => Ok(self.a),
            2 => Ok(self.b),
            _ => Err(CpuError)
        };
        self.ic += 1;
        current_pos = self.counter_register_pos();
        let number = self.ram[current_pos];
        self.a = reg.expect("failed to get register") / number;
        Ok(()) 
    }
    fn jmp(&mut self) -> Result<(), CpuError> {
        let current_pos = self.counter_register_pos();
        let jump_pos = self.ram[current_pos+1] as usize;
        let data = self.ram[jump_pos];
        self.ic = data as usize;
        Ok(())
    }
    fn jmp_if_a(&mut self) -> Result<(), CpuError> {
        let current_pos = self.counter_register_pos();
        let a = self.ram[current_pos];
        let reg: Result<u8, CpuError> = match a {
            1 => Ok(self.a),
            2 => Err(CpuError),
            _ => Err(CpuError)
        };
        if reg.unwrap() == 0 {
            self.ic += 1;
            self.jmp()?;
        }
        else {
            self.ic += 2;
        }
        Ok(())
    }
    fn jmp_if_b(&mut self) -> Result<(), CpuError> {
        let current_pos = self.counter_register_pos();
        let a = self.ram[current_pos];
        let reg: Result<u8, CpuError> = match a {
            1 => Err(CpuError),
            2 => Ok(self.b),
            _ => Err(CpuError)
        };
        if reg.unwrap() == 0 {
            self.ic += 1;
            self.jmp()?;
        }
        else {
            self.ic += 2;
        }
        Ok(())
    }
    fn mov(&mut self) -> Result<(), CpuError> {
        todo!()
    }
    fn pop(&mut self) -> Result<(), CpuError> {
        self.ic += 1;
        let _ = self.stack.pop().expect("failed to pop stack");
        Ok(())
    }
    fn push(&mut self) -> Result<(), CpuError> {
        let current_pos = self.counter_register_pos();
        let data = self.ram[current_pos + 1];
        self.ic += 2;
        self.stack.push(data);
        Ok(())
    }
    fn print(&mut self) -> Result<(), CpuError> {
        todo!()
    }
}

#[derive(Debug)]
struct CpuError;

impl fmt::Display  for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = format!("CPU reg a:{} regb: {} ", self.a, self.b);
        write!(f, "{}", p)
    }
}


impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU Error")
    }
}

impl Error for CpuError {
    fn description(&self) -> &str {
        "CPU Error"
    }  
}

fn main() {
    let cpu = Cpu::new();
    println!("{}", cpu);
}
