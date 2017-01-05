pub struct Mem {
    curr: usize,
    data: [u8; 30000]
}

impl Mem {
    pub fn new() -> Mem {
        return Mem {
            data: [0; 30000],
            curr: 0
        }
    }

    pub fn mv_right(&mut self) {
        self.curr = self.curr + 1;
    }

    pub fn mv_left(&mut self) {
        self.curr = self.curr - 1;
    }

    pub fn inc_curr(&mut self) {
        self.data[self.curr] = self.data[self.curr] + 1;
    }

    pub fn dec_curr(&mut self) {
        self.data[self.curr] = self.data[self.curr] - 1;
    }

    pub fn get_curr(&mut self) -> u8 {
        self.data[self.curr]
    }

    pub fn put_curr(&mut self, data: u8) {
        self.data[self.curr] = data;
    }
}