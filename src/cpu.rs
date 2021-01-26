use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct CPU {
    pc: u16,
    vregs: [u8; 16],
    memory: Rc<RefCell<Vec<u8>>>
}

impl CPU {
    pub fn new(mem: Rc<RefCell<Vec<u8>>>) -> Self {
        CPU {
            pc: 0x200,
            vregs: [0; 16],
            memory: mem 
        }
    }


    pub fn fetch(&self) {

    }

    pub fn get_byte_mem(&self, addr: usize) -> u8 {
        self.memory.borrow()[addr]
    }

    pub fn store_byte_mem(&self, addr: usize, value: u8)  {
        self.memory.borrow_mut()[addr] = value;
    }


}
