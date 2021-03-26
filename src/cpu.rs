use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{self};
use std::any::Any;

use rand::Rng;
use crate::System;
#[derive(Debug)]
pub struct CPU {
    pub pc: u16,
    pub vregs: [u8; 16],
    pub i: u16,
    pub vf: bool,
    pub stack: Vec<u16>,
    pub memory: Rc<RefCell<Vec<u8>>>,
    pub curr_keys: [Option<bool>; 16],
    pub dt: u8,                        // Delay Timer, decrements a tick every 60HZ
    pub st: u8,                        // Sound Timer, decrements a tick every 60HZ
    pub is_halted: bool                // Is the program done executing? Self Jp
}

impl CPU {
    pub fn stack_push(&mut self, addr: u16) {
        assert_eq!(self.stack.len() < 16, true);
        self.stack.push(addr);
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }
}
#[derive(Debug)]
pub enum DecodeError {
    GenericError,
    GenericErrorEx(String)
}
impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::GenericError =>
                write!(f, "Something went wrong decoding error"),
            DecodeError::GenericErrorEx(ref errmsg) =>
                write!(f, "GenericError err: {}", errmsg)
        }
    }
}
use core::fmt::Debug;
impl Debug for dyn Instruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Series{{{}}}", self.print())
    }
}

pub trait Instruction : fmt::Display  {
    fn print(&self) -> String;

    fn do_instr(&self, _cpu: &mut CPU) {
        println!("\t\t{}", self);
    }
    // Must be called after execute() to finish executing instruction including setting up for pc for next step
    fn incr_pc(&self, cpu: &mut CPU) {
        cpu.pc += 2;
    }

    fn execute(&self, cpu: &mut CPU) {
        //println!("Executing instr pc = 0x{:X}, instr: {}", cpu.pc, self.print());
        self.do_instr(cpu);
    }

    fn check_completed(&self, _system: &mut System) -> bool {
        return true
    }

    fn is_waited_instr(&self) -> bool {
        return false
    }

    fn as_any(&self) -> &dyn Any;
}

pub struct SysInstr {
    pub addr: u16
}

impl Instruction for SysInstr {
    fn print(&self) -> String {
        "SYS".to_string()
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        //println!("executed SYS");
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SysInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

pub struct ClrInstr {

}

impl Instruction for ClrInstr {
    fn print(&self) -> String {
        "CLR".to_string()
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        println!("excuted CLR");
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

pub struct RetInstr {}

impl Instruction for RetInstr {
    fn print(&self) -> String {
        "RET".to_string()
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        //println!("executed RET");
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        let return_addr = cpu.stack.pop();
        match return_addr {
            Some(addr) => {
                cpu.pc = addr;
            },
            None => {
                println!("Error excuting return. Empty stack");
            }
        }
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for RetInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}


pub struct CallInstr {
    addr: u16
}
impl Instruction for CallInstr {
    fn print(&self) -> String {
        let result = format!("CALL {:x} ", self.addr);
        result
    }

    fn do_instr(&self, cpu: &mut CPU) {
        //println!("executed {}", self);
        cpu.stack_push(cpu.pc + 2);
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        cpu.pc = self.addr;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for CallInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl fmt::Display for ClrInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

pub struct JpInstr {
    pub addr : u16,
}

impl Instruction for JpInstr {
    fn print(&self) -> String {
        let jp = format!("JP {:x}", self.addr);
        jp
    }


    fn do_instr(&self, cpu: &mut CPU) {
        //println!("Executing {}", self);
        if cpu.pc == self.addr {
            println!("JP to same address. Must be the end of the program");
            cpu.is_halted = true
        }
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        cpu.pc = self.addr;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for JpInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

pub struct SeInstr {
    pub vx: u8,
    pub val: u8,
}

impl Instruction for SeInstr {
    fn print(&self) -> String {
        let skipeq = format!("SE V{:x}, {:x}", self.vx, self.val);
        skipeq
    }

    fn do_instr(&self, cpu: &mut CPU) {
        //println!("executed SE");
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        if cpu.vregs[self.vx as usize] == self.val {
            cpu.pc += 4;
        } else {
            cpu.pc +=2;
        }
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SeInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

pub struct SneInstr {
    pub vx: u8,
    pub val: u8,
}

impl Instruction for SneInstr {
    fn print(&self) -> String {
        let sne = format!("SNE V{:x}, {:x}", self.vx, self.val);
        sne 
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        //println!("executed {}", self);
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        if cpu.vregs[self.vx as usize] != self.val {
            cpu.pc += 4;
        } else {
            cpu.pc +=2;
        }
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SneInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct SeRegsInstr {
    pub vx: u8,
    pub vy: u8
}

impl Instruction for SeRegsInstr {
    fn print(&self) -> String {
        let se = format!("SE V{:x}, V{:x}", self.vx, self.vy);
        se
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        //println!("executed {}", self);
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        if cpu.vregs[self.vx as usize] == cpu.vregs[self.vy as usize] {
            cpu.pc += 4;
        } else {
            cpu.pc +=2;
        }
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SeRegsInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct LdValInstr {
    vx: u8,
    value: u8
}

impl Instruction for LdValInstr {
    fn print(&self) -> String {
        let ld = format!("LD V{:X}, {:x}", self.vx, self.value); 
        ld
    }

    fn do_instr(&self, cpu: &mut CPU) {
        cpu.vregs[self.vx as usize] = self.value;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for LdValInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct AddImmediateInstr {
    vx: u8,
    value: u8
}

impl Instruction for AddImmediateInstr {
    fn print(&self) -> String {
        let add = format!("ADD V{:X}, {:x}", self.vx, self.value); 
        add
    }

    fn do_instr(&self, cpu: &mut CPU) {
        println!("vregs {:X} = {:X} value = {:X}", self.vx,cpu.vregs[self.vx as usize], self.value );
        cpu.vregs[self.vx as usize] = cpu.vregs[self.vx as usize].wrapping_add(self.value);
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for AddImmediateInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct LdRegInstr {
    vx: u8,
    vy: u8
}

impl Instruction for LdRegInstr {
    fn print(&self) -> String {
        let ld = format!("LD V{:X}, V{:X}", self.vx, self.vy);
        ld
    }

    fn do_instr(&self, cpu: &mut CPU) {
        cpu.vregs[self.vx as usize] = cpu.vregs[self.vy as usize];
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for LdRegInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct OrInstr {
    vx: u8,
    vy: u8
}

impl Instruction for OrInstr {
    fn print(&self) -> String {
        let ld = format!("OR V{:X}, V{:X}", self.vx, self.vy);
        ld
    }

    fn do_instr(&self, cpu: &mut CPU) {
        cpu.vregs[self.vx as usize] = cpu.vregs[self.vx as usize] | cpu.vregs[self.vy as usize];
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for OrInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct AndInstr {
    vx: u8,
    vy: u8
}

impl Instruction for AndInstr {
    fn print(&self) -> String {
        let ld = format!("AND V{:X}, V{:X}", self.vx, self.vy);
        ld
    }
    
    fn do_instr(&self, cpu: &mut CPU) {
        cpu.vregs[self.vx as usize] = cpu.vregs[self.vx as usize] & cpu.vregs[self.vy as usize];
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for AndInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct XorInstr {
    vx: u8,
    vy: u8
}

impl Instruction for XorInstr {
    fn print(&self) -> String {
        let xor = format!("XOR V{:X}, V{:X}", self.vx, self.vy);
        xor 
    }

    fn do_instr(&self, cpu: &mut CPU) {
        cpu.vregs[self.vx as usize] = cpu.vregs[self.vx as usize] ^ cpu.vregs[self.vy as usize];
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for XorInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct AddInstr {
    vx: u8,
    vy: u8
}

impl Instruction for AddInstr {
    fn print(&self) -> String {
        let add = format!("ADD V{:X}, V{:X}", self.vx, self.vy);
        add
    }

    fn do_instr(&self, cpu: &mut CPU) {
        let (add_result, carry) = cpu.vregs[self.vx as usize].overflowing_add(cpu.vregs[self.vy as usize]); 
        cpu.vregs[self.vx as usize] = add_result;
        cpu.vf = carry;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for AddInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct SubInstr {
    vx: u8,
    vy: u8
}

impl Instruction for SubInstr {
    fn print(&self) -> String {
        let sub = format!("SUB V{:X}, V{:X}", self.vx, self.vy);
        sub 
    }

    fn do_instr(&self, cpu: &mut CPU) {
        let (sub_result, not_underflow) = cpu.vregs[self.vx as usize].overflowing_sub(cpu.vregs[self.vy as usize]); 
        cpu.vregs[self.vx as usize] = sub_result;
        cpu.vf = !not_underflow;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SubInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct ShrInstr {
    vx: u8,
    vy: u8
}

impl Instruction for ShrInstr {
    fn print(&self) -> String {
        let shr = format!("SHR V{:X}, V{:X}", self.vx, self.vy);
        shr 
    }

    fn do_instr(&self, cpu: &mut CPU) {
       if cpu.vregs[self.vx as usize] & 1 == 1 {
           cpu.vf = true;
       }
       cpu.vregs[self.vx as usize] = cpu.vregs[self.vx as usize].wrapping_shr(1); 
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for ShrInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct SubnInstr {
    vx: u8,
    vy: u8
}

impl Instruction for SubnInstr {
    fn print(&self) -> String {
        let subn = format!("SUBN V{:X}, V{:X}", self.vx, self.vy);
        subn
    }

    fn do_instr(&self, cpu: &mut CPU) {
        let (sub_result, not_underflow) = cpu.vregs[self.vx as usize].overflowing_sub(cpu.vregs[self.vy as usize]); 
        cpu.vregs[self.vx as usize] = sub_result;
        cpu.vf = not_underflow;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SubnInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct ShlInstr {
    vx: u8,
    vy: u8
}

impl Instruction for ShlInstr {
    fn print(&self) -> String {
        let shl = format!("SHL V{:X}, V{:X}", self.vx, self.vy);
        shl 
    }

    fn do_instr(&self, cpu: &mut CPU) {
       if cpu.vregs[self.vx as usize] & 0x80 == 0x80 {
           cpu.vf = true;
       }
       cpu.vregs[self.vx as usize] = cpu.vregs[self.vx as usize].wrapping_shl(1); 
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for ShlInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct SneRegInstr {
    vx: u8,
    vy: u8
}

impl Instruction for SneRegInstr {
    fn print(&self) -> String {
        let sne = format!("SNE V{:X}, V{:X}", self.vx, self.vy);
        sne 
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        //println!("executed {}", self);
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        if cpu.vregs[self.vx as usize] != cpu.vregs[self.vy as usize] {
            cpu.pc += 4;
        } else {
            cpu.pc +=2;
        }
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SneRegInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct LdIInstr {
    addr: u16
}

impl Instruction for LdIInstr {
    fn print(&self) -> String {
        let ld_i = format!("LD I, {:X}", self.addr);
        ld_i 
    }

    fn do_instr(&self, cpu: &mut CPU) {
        cpu.i = self.addr; 
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for LdIInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct RndInstr {
    vx: u8,
    value: u8
}

impl Instruction for RndInstr {
    fn print(&self) -> String {
        let rnd = format!("RND V{:X}, 0x{:X}", self.vx, self.value);
        rnd 
    }

    fn do_instr(&self, cpu: &mut CPU) {
        //println!("executed {}", self);
        let mut rng = rand::thread_rng();
        let random_byte : u8 = rng.gen();
        cpu.vregs[self.vx as usize] = random_byte & self.value; 
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for RndInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct JpV0Instr {
    addr: u16
}

impl Instruction for JpV0Instr {
    fn print(&self) -> String {
        let jpv0 = format!("JP V0, 0x{:X}", self.addr);
        jpv0 
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        //println!("executed {}", self);
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        cpu.pc = cpu.vregs[0] as u16 + self.addr;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for JpV0Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

pub struct DrwInstr {
    pub vx: u8,
    pub vy: u8,
    pub n: u8
}

impl Instruction for DrwInstr {
    fn print(&self) -> String {
        let drw = format!("DRW V{:X}, V{:X}, {:x} ", self.vx, self.vy, self.n);
        drw
    }

    fn do_instr(&self, cpu: &mut CPU)  {
        //println!("Drawing pixel at V{:X}, {:X}, {} bytes", self.vx, self.vy, self.n)
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }

    fn is_waited_instr(&self) -> bool {
        false
    }
}

impl fmt::Display for DrwInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct SkpInstr {
    vx: u8,
}

impl Instruction for SkpInstr {
    fn print(&self) -> String {
        format!("SKP V{:X}", self.vx)
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        //println!("executed {}", self)
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        match cpu.curr_keys[self.vx as usize] {
            Some(_) => {
                    cpu.pc += 4;
                    return;
            },
            None => {}
        }
        cpu.pc += 2;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SkpInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct SknpInstr {
    vx: u8,
}

impl Instruction for SknpInstr {
    fn print(&self) -> String {
        format!("SKNP V{:X}", self.vx)
    }

    fn do_instr(&self, _cpu: &mut CPU) {
        //println!("executed {}", self);
    }

    fn incr_pc(&self, cpu: &mut CPU) {
        match cpu.curr_keys[self.vx as usize] {
            Some(_) => {},
            None => {
                cpu.pc += 4;
                return;
            }
        }
        cpu.pc += 2;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for SknpInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct LdDtInstr {
    vx: u8,
}

impl Instruction for LdDtInstr {
    fn print(&self) -> String {
        format!("LD V{:X}, DT", self.vx)
    }

    fn do_instr(&self, cpu: &mut CPU) {
        cpu.vregs[self.vx as usize] = cpu.dt;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for LdDtInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct LdKeyInstr {
    vx: u8,
}

impl Instruction for LdKeyInstr {
    fn print(&self) -> String {
        format!("LD V{:X}, K", self.vx)
    }

    fn do_instr(&self, _cpu: &mut CPU) {

    }

    fn is_waited_instr(&self) -> bool {
        return true;
    }

    fn check_completed(&self, _system: &mut System) -> bool {
        return false
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for LdKeyInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct StoreDtInstr {
    vx: u8,
}

impl Instruction for StoreDtInstr {
    fn print(&self) -> String {
        format!("LD DT, V{:X}", self.vx)
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for StoreDtInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct StoreStInstr {
    vx: u8,
}

impl Instruction for StoreStInstr {
    fn print(&self) -> String {
        format!("LD ST, V{:X}", self.vx)
    }

    fn do_instr(&self, cpu: &mut CPU) {
        cpu.st = cpu.vregs[self.vx as usize];
    }
    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for StoreStInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct AddIInstr {
    vx: u8,
}

impl Instruction for AddIInstr {
    fn print(&self) -> String {
        format!("ADD I,  V{:X}", self.vx)
    }

    fn do_instr(&self, cpu: &mut CPU) {
        cpu.i = cpu.i + (cpu.vregs[self.vx as usize] as u16);
    }
    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for AddIInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct StoreSpriteInstr {
    vx: u8,
}

impl Instruction for StoreSpriteInstr {
    fn print(&self) -> String {
        format!("LD F, V{:X}", self.vx)
    }

    fn do_instr(&self, cpu: &mut CPU) {
        let vx_val = cpu.vregs[self.vx as usize];
        cpu.i = vx_val as u16;
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for StoreSpriteInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct StoreBcdInstr {
    vx: u8,
}

impl Instruction for StoreBcdInstr {
    fn print(&self) -> String {
        format!("LD B, V{:X}", self.vx)
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for StoreBcdInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct StoreVxsIntoIInstr {
    vx: u8,
}

impl Instruction for StoreVxsIntoIInstr{
    fn print(&self) -> String {
        format!("LD [I], V{:X}", self.vx)
    }

    fn do_instr(&self, cpu: &mut CPU) {
        for i in 0..self.vx {
            cpu.store_byte_mem((cpu.i + (i as u16)) as usize, cpu.vregs[i as usize]);
        }
    }
    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for StoreVxsIntoIInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

struct LdVxsFromIInstr {
    vx: u8,
}

impl Instruction for LdVxsFromIInstr{
    fn print(&self) -> String {
        format!("LD V{:X}, [I]", self.vx)
    }

    fn do_instr(&self, cpu: &mut CPU) {
        for i in 0..self.vx {
            cpu.vregs[i as usize] = cpu.get_byte_mem((cpu.i + (i as u16)) as usize);
        }
    }
    fn as_any(&self) ->  &dyn Any {
        self
    }
}

impl fmt::Display for LdVxsFromIInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl CPU {
    pub fn new(mem: Rc<RefCell<Vec<u8>>>) -> Self {
        CPU {
            pc: 0x200,
            vregs: [0; 16],
            stack: vec![],
            i: 0,
            vf: false,
            memory: mem,
            curr_keys: [None; 16],
            is_halted: false,
            dt: 255,
            st: 255
        }
    }

    pub fn get_byte_mem(&self, addr: usize) -> u8 {
        self.memory.borrow()[addr]
    }

    pub fn store_byte_mem(&self, addr: usize, value: u8)  {
        self.memory.borrow_mut()[addr] = value;
    }

    pub fn fetch_instr_from_addr(&self, addr: usize) -> u16 {
        let instr = u16::from(self.memory.borrow()[addr]) << 8 | 
                    u16::from(self.memory.borrow()[addr+1]);
        instr.into()
    }

    pub fn decode_instr(instr: u16) -> Result<Box<dyn Instruction>, DecodeError> {
        let bits15_12 : u8 = (instr >> 12) as u8;
        let bits11_8 : u8 = ((instr >> 8) & 0xF) as u8;
        let bits7_4 : u8 = ((instr >> 4) & 0xF) as u8;
        let bits3_0 : u8 = ((instr) & 0xF) as u8;
        let nnn : u16 = ((u16::from(bits11_8) << 8) | (u16::from(bits7_4) << 4) | u16::from(bits3_0)) as u16; 
        let result : Result<Box<dyn Instruction>, DecodeError>;
        match bits15_12 {
            // CLR
            // RET
            0x0 => {
                if bits11_8  == 0 {
                    if bits7_4 == 0xE {
                        if bits3_0 == 0 {
                            result = Ok(Box::new(ClrInstr{}))   
                        } else if bits3_0 == 0xE {
                            result = Ok(Box::new(RetInstr{})) 
                        } else {
                            result = Err(DecodeError::GenericError)
                        }
                    } else {
                        result = Err(DecodeError::GenericError) 
                    }                   
                } else {
                    result = Ok(Box::new(SysInstr{ addr: nnn })) 
                }
            },
            // JP NNN
            0x1 => {
                result =  Ok(Box::new(JpInstr { addr: nnn})) 
            },
            // CALL NNN
            0x2 => {
                result = Ok(Box::new(CallInstr { addr: nnn }))
            },

            // SE Vx, kk --- Skip next instruction if Vx == kk
            0x3 => {
                let byte = bits7_4 << 4 | bits3_0;
                result = Ok(Box::new(SeInstr { vx: bits11_8, val: byte }))
            },

            // SNE Vx, kk --- Skip next instruction if Vx != kk
            0x4 => {
                let byte = bits7_4 << 4 | bits3_0;
                result = Ok(Box::new(SneInstr { vx: bits11_8, val: byte }))
            },

            0x5 => {
                if bits3_0 == 0 {
                    result = Ok(Box::new(SeRegsInstr { vx: bits11_8, vy: bits7_4}))
                } else {
                    let unsupported_5xxx = format!("unsupported 5 instr:{:x}", instr);
                    result = Err(DecodeError::GenericErrorEx(unsupported_5xxx,))
                }
            }
            // LD Vx, KK
            0x6 => {
                let byte = bits7_4 << 4 | bits3_0;
                result = Ok(Box::new(LdValInstr{ vx: bits11_8, value: byte}))
            },
            0x7 => {
                let byte = bits7_4 << 4 | bits3_0;
                result = Ok(Box::new(AddImmediateInstr{ vx: bits11_8, value: byte}))
            },
            // LD Vx,Vy and ALU instructions
            0x8 => {
                match bits3_0 {
                    0x0 => {
                        result = Ok(Box::new(LdRegInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    0x1 => {
                        result = Ok(Box::new(OrInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    0x2 => {
                        result = Ok(Box::new(AndInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    0x3 => {
                        result = Ok(Box::new(XorInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    0x4 => {
                        result = Ok(Box::new(AddInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    0x5 => {
                        result = Ok(Box::new(SubInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    0x6 => {
                        result = Ok(Box::new(ShrInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    0x7 => {
                        result = Ok(Box::new(SubnInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    0xE => {
                        result = Ok(Box::new(ShlInstr{ vx: bits11_8, vy: bits7_4}))
                    },
                    _ => {
                        let unsupported = format!("unsupported {:x}", instr);
                        result = Err(DecodeError::GenericErrorEx(unsupported))
                    }
                }
            },
            0x9 => {
                result = Ok(Box::new(SneRegInstr{ vx: bits11_8, vy: bits7_4}))
            },
            0xA => {
                result = Ok(Box::new(LdIInstr{ addr: nnn }))
            },
            0xB => {
                result = Ok(Box::new(JpV0Instr{ addr: nnn }))
            }
            0xC => {
                let byte : u8 = (bits7_4 as u8) << 4 | bits3_0; 
                result = Ok(Box::new(RndInstr{ vx: bits11_8, value: byte }))
            }
            0xD => {
                result = Ok(Box::new(DrwInstr{ vx: bits11_8, vy: bits7_4, n: bits3_0}))
            },
            0xE => {
                let lsbyte = bits7_4 << 4 | bits3_0;
                match lsbyte {
                    0x9E => {
                        result = Ok(Box::new(SkpInstr{ vx: bits11_8 }))
                    },
                    0xA1 => {
                        result = Ok(Box::new(SknpInstr{ vx: bits11_8 }))
                    },
                    _ => {
                        let unsupported_e = format!("unsupported E instr {}", instr);
                        result = Err(DecodeError::GenericErrorEx(unsupported_e))
                    }
                }
            },


            0xF => {
                let f_opcode = bits7_4 << 4 | bits3_0;  
                match f_opcode {
                    0x07 => {
                        result = Ok(Box::new(LdDtInstr{ vx: bits11_8 }))
                    },
                    0x0A => {
                        result = Ok(Box::new(LdKeyInstr{ vx: bits11_8 }))
                    },
                    0x15 => {
                        result = Ok(Box::new(StoreDtInstr{ vx: bits11_8 }))
                    },
                    0x18 => {
                        result = Ok(Box::new(StoreStInstr{ vx: bits11_8 }))
                    },
                    0x1E => {
                        result = Ok(Box::new(AddIInstr{ vx: bits11_8 }))
                    },
                    0x29 => {
                        result = Ok(Box::new(StoreSpriteInstr{ vx: bits11_8 }))
                    },
                    0x33 => {
                        result = Ok(Box::new(StoreBcdInstr{ vx: bits11_8}))
                    },
                    0x55 => {
                        result = Ok(Box::new(StoreVxsIntoIInstr{ vx: bits11_8}))
                    },
                    0x65 => {
                        result = Ok(Box::new(LdVxsFromIInstr{ vx: bits11_8}))
                    },
                    _ => {
                        let unsupported_f = format!("unsupported F instr: {}", instr);
                        result = Err(DecodeError::GenericErrorEx(unsupported_f))
                    }
                }
            },


            _ => {
                let unsupported = format!("unsupported {:x}", instr);
                result = Err(DecodeError::GenericErrorEx(unsupported))
            }
        }
        result
    }


    pub fn fetch_instr_from_pc(&self) -> Result<Box<dyn Instruction>, DecodeError> {
        let instr = self.fetch_instr_from_addr(self.pc as usize);
        let decoded_instr = CPU::decode_instr(instr);
        decoded_instr
    }
}
