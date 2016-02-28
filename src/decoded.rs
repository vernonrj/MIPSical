pub trait Decoded {
    fn name(&self) -> &'static str;
    fn inputs(&self) -> Vec<IO>;
    fn outputs(&self) -> Option<IO>;
    fn is_trap_on_overflow(&self) -> bool {
        false
    }
}

pub trait Executable: Decoded {
    fn execute(&self, registers: &[u64]) -> Option<u64>;
}


pub enum IO {
    Register(u8),
    Memory,
}
