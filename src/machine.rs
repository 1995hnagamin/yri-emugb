use std::io::{Cursor, Read};

pub struct Machine {
    mem: Cursor<Vec<u8>>,
}

impl Machine {
    pub fn new(mem: Cursor<Vec<u8>>) -> Machine {
        Machine { mem: mem }
    }
}
