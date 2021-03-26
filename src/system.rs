use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu;
use crate::cpu::CPU;
use crate::rom::ROM;
use crate::display::Display;

pub const MEMSIZE: usize = 4*1024;
pub const ROM_OFFSET: usize = 0x200;

#[derive(Debug)]
pub struct System {
   pub cpu: CPU,
   mem: Rc<RefCell<Vec<u8>>>,

   // This holds the current instruction processed by the cpu, normally it is None, only used for 
   // certain instructions that _halt_ the cpu until a condition is met. i.e. LD VX, K 
   curr_instr : Option<Box<dyn cpu::Instruction>>,
   pub draw_screen : bool,
   pub display: Display
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
            curr_instr:  None,
            display: Display::new(),
            draw_screen: false
        }
    }

    pub fn load_rom(&mut self, rom: &ROM) {
        for (data, i) in rom.data().iter().zip(0..rom.size()){
            self.mem.borrow_mut()[ROM_OFFSET + i] = *data;
        }
    }

    pub fn do_drw_instr(&mut self, draw_instr: &cpu::DrwInstr) {
        let mut sprite: Vec<u8> = vec!();
        let i_reg = self.cpu.i;
        for i in 0..draw_instr.n {
            sprite.push(self.cpu.get_byte_mem((i_reg + (i as u16)) as usize));
        }
        let x = self.cpu.vregs[draw_instr.vx as usize];
        let y = self.cpu.vregs[draw_instr.vy as usize];
        self.cpu.vf = self.display.draw_sprite(x,y, sprite);
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
                        instr.incr_pc(&mut self.cpu);
                         // Test if any of these are special instructions we should handle
                        if  let Some(drw_instr) = instr.as_any().downcast_ref::<cpu::DrwInstr>() {
                            println!("DRAW: {}", instr.print());
                            self.do_drw_instr(drw_instr);
                            self.draw_screen = true
                        }
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
    pub fn run_tick(&mut self) {
           self.step();
           self.update_dt_st();
    }

    pub fn update_dt_st(&mut self) {
        if self.cpu.dt > 0 {
            self.cpu.dt -= 1;
        }

        if self.cpu.st > 0 {
            self.cpu.st -= 1;
        }
    }
}
