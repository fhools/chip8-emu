use std::cell::RefCell;
use std::rc::Rc;
use std::time;
use crate::cpu;
use crate::cpu::CPU;
use crate::rom::ROM;
use crate::display::Display;

pub const MEMSIZE: usize = 4 * 1024;
pub const ROM_OFFSET: usize = 0x200;

// character fonts. loaded into memory starting at address 0x0
pub const FONT_DATA  : [u8; 80 ]= [
    0xF0u8, 0x90u8, 0x90u8, 0x90u8, 0xF0u8, // 0 
    0x20u8, 0x60u8, 0x20u8, 0x20u8, 0x70u8, // 1
    0xF0u8, 0x10u8, 0xF0u8, 0x80u8, 0xF0u8, // 2
    0xF0u8, 0x10u8, 0xF0u8, 0x10u8, 0xF0u8, // 3
    0x90u8, 0x90u8, 0xF0u8, 0x10u8, 0x10u8, // 4
    0xF0u8, 0x10u8, 0xF0u8, 0x10u8, 0xF0u8, // 5
    0xF0u8, 0x80u8, 0xF0u8, 0x90u8, 0xF0u8, // 6
    0xF0u8, 0x10u8, 0x20u8, 0x40u8, 0x40u8, // 7
    0xF0u8, 0x90u8, 0xF0u8, 0x90u8, 0xF0u8, // 8
    0xF0u8, 0x90u8, 0xF0u8, 0x10u8, 0xF0u8, // 9
    0xf0u8, 0x90u8, 0xF0u8, 0x90u8, 0x90u8, // A
    0xE0u8, 0x90u8, 0xE0u8, 0x90u8, 0xE0u8, // B
    0xF0u8, 0x80u8, 0x80u8, 0x80u8, 0xF0u8, // C
    0xE0u8, 0x90u8, 0x90u8, 0x90u8, 0xE0u8, // D
    0xF0u8, 0x80u8, 0xF0u8, 0x80u8, 0xF0u8, // E
    0xF0u8, 0x80u8, 0xF0u8, 0x80u8, 0x80u8, // F
    ];
    
#[derive(Debug)]
pub struct System {
   pub cpu: CPU,
   mem: Rc<RefCell<Vec<u8>>>,

   // This holds the current instruction processed by the cpu, normally it is None, only used for 
   // certain instructions that _halt_ the cpu until a condition is met. i.e. LD VX, K 
   curr_instr : Option<Box<dyn cpu::Instruction>>,
   pub draw_screen : bool,
   pub display: Display,

   // To keep track of time to decrement DT/ST timers
   time_since_dt_update: f32
   

}

impl System {
    pub fn new() -> Self {
        let mem = Rc::new(RefCell::new(vec![0; MEMSIZE]));
        let system = System {
            cpu: CPU::new(mem.clone()),
            mem: mem.clone(),
            curr_instr:  None,
            display: Display::new(),
            draw_screen: false,
            time_since_dt_update: 0.0
        };

        // load font
        let mut i = 0;
        for sprite in  FONT_DATA.iter() {
            system.cpu.store_byte_mem(i, *sprite);
            i += 1;
        }
        system
    }

    pub fn load_rom(&mut self, rom: &ROM) {
        for (data, i) in rom.data().iter().zip(0..rom.size()){
            self.mem.borrow_mut()[ROM_OFFSET + i] = *data;
        }
    }

    pub fn dump_rom(&mut self, rom: &ROM) {
        let mut i = 0;
            for data in rom.into_iter() {
                let instr = cpu::CPU::decode_instr(data);
                if let Ok(instr) = instr {
                    println!("addr: {:0>4X} instr: {:0>4X} = {}", ROM_OFFSET + i, data, instr.print())
                }
                i += 2;
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
        self.cpu.vregs[cpu::VF] = self.display.draw_sprite(x,y, sprite) as u8;
        if self.cpu.vregs[cpu::VF] == 1 {
            println!("YES WE GOT A HIT at {}, {}", x ,y);
        } else { 
            println!("NO  HIT at {}, {}", x ,y);
        }
    }

    pub fn step(&mut self) {
        // We are currently not in the middle of an instruction
        if let None = self.curr_instr {
            let ins = self.cpu.fetch_instr_from_pc();
            match ins {
                Ok(instr) => {
                    instr.execute(&mut self.cpu);
                    println!("PC: {:X} OPCODE: {:X} INSTR: {}", self.cpu.pc,  self.cpu.fetch_instr_from_addr(self.cpu.pc as usize) , instr.print());
                    if instr.is_waited_instr() {
                        self.curr_instr = Some(instr)
                    } else {
                         // Test if any of these are special instructions we should handle
                         if  let Some(drw_instr) = instr.as_any().downcast_ref::<cpu::DrwInstr>() {
                           
                            self.do_drw_instr(drw_instr);
                            self.draw_screen = true
                        }
                        instr.incr_pc(&mut self.cpu);
                        
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
    pub fn run_tick(&mut self, delta: std::time::Duration) {
           self.step();
           self.update_dt_st(delta);
    }

    pub fn update_dt_st(&mut self, delta: std::time::Duration) {
        println!("{:?} time_since_update: {}", delta, self.time_since_dt_update);
        if self.time_since_dt_update >=  (1000.0 / 60.0) {
            if self.cpu.dt > 0 {
                println!("decrement dt");
                self.cpu.dt -= 1;
            }
    
            if self.cpu.st > 0 {
                self.cpu.st -= 1;
            }
            //reset counter
            self.time_since_dt_update = self.time_since_dt_update - (1000.0 / 60.0);
        } else {
            let dt = (delta.as_micros() as f32) / 1000.0;
            println!("dt update: {}", dt);
            self.time_since_dt_update += dt;
        }
        
    }
}
