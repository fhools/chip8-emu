use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::rom::ROM;

pub const MEMSIZE: usize = 4*1024;

#[derive(Debug)]
pub struct System {
   pub cpu: CPU,
   mem: Rc<RefCell<Vec<u8>>>
}

pub enum SystemError {
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

    pub fn load_rom(&mut self, rom: &ROM) -> Result<(), SystemError> {
        for (data, i) in rom.data().iter().zip(0..rom.size()){
            println!("i {}, data {:x}", i, data);
            self.mem.borrow_mut()[0x200 + i] = *data;
        }
        Ok(())
    }
}
