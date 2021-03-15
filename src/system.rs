use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu;
use crate::cpu::CPU;
use crate::rom::ROM;

pub const MEMSIZE: usize = 4*1024;
pub const ROM_OFFSET: usize = 0x200;

#[derive(Debug)]
pub struct System {
   pub cpu: CPU,
   mem: Rc<RefCell<Vec<u8>>>,
   curr_instr : Option<Box<dyn cpu::Instruction>>
}

pub enum Error {
    Err
}


impl System {
    pub fn new() -> Self {
        let mem = Rc::new(RefCell::new(vec![0; MEMSIZE]));
        System {
            cpu: CPU::new(mem.clone()),
            mem: mem.clone(),
            curr_instr:  None
        }
    }

    pub fn load_rom(&mut self, rom: &ROM) {
        for (data, i) in rom.data().iter().zip(0..rom.size()){
            self.mem.borrow_mut()[ROM_OFFSET + i] = *data;
        }
    }

    pub fn step(&mut self) {
        // We are currently not in the middle of an instruction
        if let None = self.curr_instr {
            let ins = self.cpu.fetch_instr_from_pc();
            match ins {
                Ok(instr) => {
                    instr.execute(&mut self.cpu);
                    if instr.is_waited_instr() {
                        self.curr_instr = Some(instr)
                    } else {
                        instr.incr_pc(&mut self.cpu)
                    }
                }, 
                Err(err) => {
                    println!("Error fetching instruction {}", err);
                }
            }
            if self.cpu.is_halted() {
                println!("CPU is halted. Current pc is {}", self.cpu.pc);
            } 
           
        } else {
        // Still processing an instruction
            let curr_instr = self.curr_instr.take();
            match curr_instr {
                Some(instr) => {
                    if instr.check_completed(self) {
                        instr.incr_pc(&mut self.cpu);
                        self.curr_instr = None;
                    }  else {
                        println!("Still waiting for instr {} to complete", instr)
                    }
                },
                None => {
                    println!("Error unwrapping instruction! No instruction present");
                }
            }
        }
    }
    pub fn run(&mut self) {
        loop {
           self.step();
        }
    }
}
