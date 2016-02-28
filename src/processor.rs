pub struct Processor {
    pub reg: [u32; 32],
}

impl Processor {
    pub fn new() -> Self {
        Processor { reg: [0; 32] }
    }
}
