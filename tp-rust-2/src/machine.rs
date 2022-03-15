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
    InvalidOpcode,
    InvalidRegisterNumb,
    InvalidMemAddr,
    WriteError
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

        let inst_addr = self.reg[IP] as usize;

        let opcode = self.read_mem(inst_addr)?; 

        match opcode {
            1 => return self.move_if(),
            2 => return self.store(),
            3 => return self.load(),
            4 => return self.loadimm(),
            5 => return self.sub(),
            6 => return self.out(fd),
            7 => return self.exit(),
            8 => return self.out_number(fd),
            _ => return Err(MachineError::InvalidOpcode)
        }
    }

    pub fn move_if(&mut self) -> Result<bool, MachineError> {

        let inst_addr = self.reg[IP] as usize;

        // Increment the IP
        self.reg[IP] += 4u32;

        // Decode
        let reg_a = self.read_mem(inst_addr + 1)? as usize;
        let reg_b = self.read_mem(inst_addr + 2)? as usize;
        let reg_c = self.read_mem(inst_addr + 3)? as usize;

        let reg_b_cont: u32 = self.read_reg(reg_b)?;
        let reg_c_cont: u32 = self.read_reg(reg_c)?;

        // Execute
        if reg_c_cont != 0 {
            self.set_reg(reg_a, reg_b_cont)?;
        }

        Ok(false)
    }

    pub fn store(&mut self) -> Result<bool, MachineError> {
        
        let inst_addr = self.reg[IP] as usize;

        // Increment the IP
        self.reg[IP] += 3u32;

        // Decode
        let reg_a = self.read_mem(inst_addr + 1)? as usize;
        let reg_b = self.read_mem(inst_addr + 2)? as usize;

        // Execute
        let reg_a_cont = self.read_reg(reg_a)? as usize;
        let reg_b_cont = self.read_reg(reg_b)?;

        let data: [u8; 4] = reg_b_cont.to_le_bytes();
        if reg_a_cont + 3 < MEMORY_SIZE {
            self.mem[reg_a_cont] = data[0];
            self.mem[reg_a_cont + 1] = data[1];
            self.mem[reg_a_cont + 2] = data[2];
            self.mem[reg_a_cont + 3] = data[3];
        }
        else {
            return Err(MachineError::InvalidMemAddr);
        }
        Ok(false)
        
    }

    pub fn load(&mut self) -> Result<bool, MachineError> {

        let instr_addr = self.reg[IP] as usize;

        // Increment the IP 
        self.reg[IP] += 3u32;

        // Decode
        let reg_a = self.read_mem(instr_addr + 1)? as usize;
        let reg_b = self.read_mem(instr_addr + 2)? as usize;

        // Execute
        let addr = self.read_reg(reg_b)? as usize;
        if addr + 3 < MEMORY_SIZE {
            let value: [u8; 4] = [self.read_mem(addr)?,
                                  self.read_mem(addr + 1)?,
                                  self.read_mem(addr + 2)?,
                                  self.read_mem(addr + 3)?
                                 ];
            self.set_reg(reg_a, u32::from_le_bytes(value))?;
        }
        else {
            return Err(MachineError::InvalidMemAddr);
        }
        
        Ok(false)

    }

    pub fn loadimm(&mut self) -> Result<bool, MachineError> {

        let inst_addr = self.reg[IP] as usize;

        // Increment the IP
        self.reg[IP] += 4u32;

        // Decode
        let reg_a = self.read_mem(inst_addr + 1)? as usize;
        let lh: [u8; 2] = [self.read_mem(inst_addr + 2)?,
                           self.read_mem(inst_addr + 3)?
                          ];
        
        // Execute
        let word = i16::from_le_bytes(lh) as i32;

        self.set_reg(reg_a, word as u32)?;

        Ok(false)

    }

    pub fn sub(&mut self) -> Result<bool, MachineError> {

        let inst_addr = self.reg[IP] as usize;

        // Increment the IP
        self.reg[IP] += 4u32;

        // Decode
        let reg_a = self.read_mem(inst_addr + 1)? as usize;
        let reg_b = self.read_mem(inst_addr + 2)? as usize;
        let reg_c = self.read_mem(inst_addr + 3)? as usize;

        // Execute
        let reg_b_cont = self.read_reg(reg_b)? as i32;
        let reg_c_cont = self.read_reg(reg_c)? as i32;

        //let result = i32::wrapping_sub(reg_b_cont, reg_c_cont);
        let result = reg_b_cont.wrapping_sub(reg_c_cont);

        self.set_reg(reg_a, result as u32)?;

        Ok(false)

    }

    pub fn out<T: Write>(&mut self, fd: &mut T) -> Result<bool, MachineError> {

        let inst_addr = self.reg[IP] as usize;

        // Increment the IP
        self.reg[IP] += 2u32;

        // Decode
        let reg_a = self.read_mem(inst_addr + 1)? as usize;

        // Execute
        let my_char = (self.read_reg(reg_a)? as u8) as char;

        match fd.write_all(my_char.to_string().as_bytes()) {
            Ok(_) => Ok(false),
            Err(_)=> Err(MachineError::WriteError)
        }

    }

    pub fn exit(&mut self) -> Result<bool, MachineError> {

        //Increment the IP
        self.reg[IP] += 1u32;

        Ok(true)
    }

    pub fn out_number<T: Write>(&mut self, fd: &mut T) -> Result<bool, MachineError> {

        let inst_addr = self.reg[IP] as usize;

        // Increment the IP
        self.reg[IP] += 2u32;

        // Decode
        let reg_a = self.read_mem(inst_addr + 1)? as usize;
        
        // Execute
        let number = self.read_reg(reg_a)? as i32;
        let _dec = i32::from_str_radix(&number.to_string(), 10).unwrap();

        match fd.write_all(_dec.to_string().as_bytes()) {
            Ok(_) => Ok(false),
            Err(_)=> Err(MachineError::WriteError)
        }
    }

    /// Similar to [step_on](Machine::step_on).
    /// If output instructions are run, they print on standard output.
    pub fn step(&mut self) -> Result<bool, MachineError> {
        self.step_on(&mut io::stdout().lock())
    }

    /// Reference onto the machine current set of registers.
    pub fn regs(&self) -> &[u32] {
        return &self.reg[..];
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, reg: usize, value: u32) -> Result<(), MachineError> {
        if reg >= NREGS {
            return Err(MachineError::InvalidRegisterNumb);
        }
        self.reg[reg] = value;
        Ok(())
    }

    /// Reference onto the machine current memory.
    pub fn memory(&self) -> &[u8] {
        return &self.mem[..];
    }


    /// Check if machine memory adress is located in the right memory space
    /// (from 0 to MEMORY_SIZE - 1) 
    pub fn read_mem(&self, addr: usize) -> Result<u8, MachineError> {
        match addr {
            n if n < MEMORY_SIZE => Ok(self.mem[addr]),
            _                           => Err(MachineError::InvalidMemAddr),
        }
    }

    /// Check if the register number exists
    /// (should be between 0 and NREGS - 1) 
    pub fn read_reg(&self, reg_num: usize) -> Result<u32, MachineError> {
        match reg_num {
            n if n < NREGS => Ok(self.reg[reg_num]),
            _                     => Err(MachineError::InvalidRegisterNumb),
        }
    }

}
