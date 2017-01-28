pub struct Mem {
    curr: usize,
    data: [u8; 30000]
}

impl Mem {
    pub fn new() -> Mem {
        trace!("Initialising memory with length 30000");
        return Mem {
            data: [0; 30000],
            curr: 0
        }
    }

    pub fn mv_right(&mut self) {
        trace!("Moving pointer right");
        self.curr = self.curr + 1;
    }

    pub fn mv_left(&mut self) {
        trace!("Moving pointer left");
        self.curr = self.curr - 1;
    }

    pub fn inc_curr(&mut self) {
        trace!("Incrementing current value");
        self.data[self.curr] = self.data[self.curr] + 1;
    }

    pub fn dec_curr(&mut self) {
        trace!("Decrementing current value");
        self.data[self.curr] = self.data[self.curr] - 1;
    }

    pub fn get_curr(&mut self) -> u8 {
        trace!("Getting current value");
        self.data[self.curr]
    }

    pub fn put_curr(&mut self, data: u8) {
        trace!("Setting current value");
        self.data[self.curr] = data;
    }
}