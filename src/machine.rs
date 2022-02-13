use std::error::Error;
use std::fmt;
use std::io::{Cursor, Read};

#[derive(Debug)]
struct MachineError {
    msg: String,
}

impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for MachineError {}

enum Opcode {
    Nop,
    Jpnn(u16),
}

pub struct Machine {
    mem: Cursor<Vec<u8>>,
}

impl Machine {
    pub fn new(mem: Cursor<Vec<u8>>) -> Machine {
        Machine { mem: mem }
    }

    fn fetch(&mut self) -> Result<Opcode, Box<dyn Error>> {
        let data: &mut [u8] = &mut [0; 1];
        self.mem.read(data)?;
        match data[0] {
            0x00 => Ok(Opcode::Nop),
            0xC3 => {
                let data: &mut [u8] = &mut [0; 2];
                self.mem.read(data)?;
                let n = ((data[1] as u16) << 8) | (data[0] as u16);
                Ok(Opcode::Jpnn(n))
            }
            x => Err(Box::new(MachineError {
                msg: format!("unimplemented ({:02x?})", x),
            })),
        }
    }

    fn exec(&mut self, op: Opcode) -> Result<(), Box<dyn Error>> {
        match op {
            Opcode::Nop => {
                println!("nop");
                Ok(())
            }
            Opcode::Jpnn(n) => {
                println!("jp {:x?}", n);
                self.mem.set_position(n.into());
                Ok(())
            }
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.mem.set_position(0x100);
        loop {
            let op = self.fetch()?;
            self.exec(op)?;
        }
    }
}
