use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::rom::ROM;

pub const MEMSIZE: usize = 4*1024;
pub const ROM_OFFSET: usize = 0x200;

#[derive(Debug)]
pub struct System {
   pub cpu: CPU,
   mem: Rc<RefCell<Vec<u8>>>
}

pub enum Error {
    None,
    Err
}


impl System {
    pub fn new() -> Self {
        let mem = Rc::new(RefCell::new(vec![0; MEMSIZE]));
        mem.borrow_mut()[100] = 200;
        System {
            cpu: CPU::new(mem.clone()),
            mem: mem.clone()
        }
    }

    pub fn load_rom(&mut self, rom: &ROM) {
        for (data, i) in rom.data().iter().zip(0..rom.size()){
            self.mem.borrow_mut()[ROM_OFFSET + i] = *data;
        }
    }
}
