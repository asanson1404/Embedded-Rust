use std::io::{self, Write};

const MEMORY_SIZE: usize = 4096;
const NREGS: usize = 16;

const IP: usize = 0;

pub struct Machine {
    reg: [u32; NREGS],
    mem: [u8; MEMORY_SIZE]
}

#[derive(Debug)]
pub enum MachineError {
    WrongRegAddr,
    InvalidOpcode,
    InvalidRegisterNumb,
    InvalidMemNumb,
}

impl Machine {
    /// Create a new machine in its reset state. The `memory` parameter will
    /// be copied at the beginning of the machine memory.
    ///
    /// # Panics
    /// This function panics when `memory` is larger than the machine memory.
    pub fn new(memory: &[u8]) -> Self {

        assert!(memory.len() <= MEMORY_SIZE, "The memory is larger than the machine memory");

        let mut initial_mem: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
        //for i in 0..memory.len() {
        //    initial_mem[i] = memory[i];
        //}
        initial_mem[0..memory.len()].copy_from_slice(memory);
        Machine {mem: initial_mem, reg: [0; NREGS]}
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on `fd`.
    pub fn run_on<T: Write>(&mut self, fd: &mut T) -> Result<(), MachineError> {
        while !self.step_on(fd)? {}
        Ok(())
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on standard output.
    pub fn run(&mut self) -> Result<(), MachineError> {
        self.run_on(&mut io::stdout().lock())
    }

    /// Execute the next instruction by doing the following steps:
    ///   - decode the instruction located at IP (register 0)
    ///   - increment the IP by the size of the instruction
    ///   - execute the decoded instruction
    ///
    /// If output instructions are run, they print on `fd`.
    /// If an error happens at either of those steps, an error is
    /// returned.
    ///
    /// In case of success, `true` is returned if the program is
    /// terminated (upon encountering an exit instruction), or
    /// `false` if the execution must continue.
    pub fn step_on<T: Write>(&mut self, fd: &mut T) -> Result<bool, MachineError> {

        //let num_inst: u8 = self.mem[self.reg[IP] as usize];
        let index = self.reg[IP] as usize;

        if check_mem(index) {
            return Err(MachineError::InvalidMemNumb);
        }

        let num_inst: u8 = self.mem[index];

        match num_inst {
            1 => return self.move_if(),
            2 => return self.store(),
            _ => return Err(MachineError::InvalidOpcode)
        }
        //Ok(true)
    }

    pub fn move_if(&mut self) -> Result<bool, MachineError> {

        let index = self.reg[IP] as usize;

        if !check_mem(index + 1) || !check_mem(index + 2) || !check_mem(index + 3) {
            return Err(MachineError::InvalidMemNumb);
        }

        let reg_a: u8 = self.mem[index + 1];
        let reg_b: u8 = self.mem[index + 2];
        let reg_c: u8 = self.mem[index + 3];

        if !check_reg(reg_a) || !check_reg(reg_b) || !check_reg(reg_c) {
            return Err(MachineError::InvalidRegisterNumb);
        }

        else {
            if self.reg[reg_c as usize] != 0 {
                self.reg[reg_a as usize] = self.reg[reg_b as usize];
            }
        }
        Ok(false)
    }

    pub fn store(& mut self) -> Result<bool, MachineError> {
        
        let index = self.reg[IP] as usize;

        if !check_mem(index + 1) || !check_mem(index + 2) {
            return Err(MachineError::InvalidMemNumb);
        }

        let reg_a: u8 = self.mem[index + 1];
        let reg_b: u8 = self.mem[index + 2];

        if !check_reg(reg_a) || !check_reg(reg_b) {
            return Err(MachineError::InvalidRegisterNumb);
        }

        let data: usize = self.reg[reg_b as usize] as usize;
        let add: u32 = self.reg[reg_a as usize];

        //vérifier les index

        self.mem[add as usize] = data

        self.mem[reg_a as usize]

        Ok(false)

    }

    /// Similar to [step_on](Machine::step_on).
    /// If output instructions are run, they print on standard output.
    pub fn step(&mut self) -> Result<bool, MachineError> {
        self.step_on(&mut io::stdout().lock())
    }

    /// Reference onto the machine current set of registers.
    pub fn regs(&self) -> &[u32] {
        unimplemented!()  // Implement me!
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, reg: usize, value: u32) -> Result<(), MachineError> {
        if reg > NREGS {
            return Err(MachineError::WrongRegAddr);
        }
        self.reg[reg] = value;
        Ok(())
    }

    /// Reference onto the machine current memory.
    pub fn memory(&self) -> &[u8] {
        unimplemented!()  // Implement me!
    }
}

pub fn check_reg(reg: u8) -> bool {
    if reg > (NREGS - 1) as u8 {
        return true
    }
    false
}

pub fn check_mem(index: usize) -> bool {
    if index > (MEMORY_SIZE - 1) as usize {
        return true;
    }
    false
}
