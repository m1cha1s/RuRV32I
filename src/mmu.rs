use crate::vm::{BusIF, Dev};

pub struct MMU<M, P>
where
    M: Dev + BusIF,
    P: Dev + BusIF,
{
    mem: M,
    periph: P,
}

impl<M, P> Dev for MMU<M, P>
where
    M: Dev + BusIF,
    P: Dev + BusIF,
{
    fn new() -> Self {
        Self {
            mem: M::new(),
            periph: P::new(),
        }
    }

    fn reset(&mut self) -> &mut Self {
        self.mem.reset();
        self.periph.reset();

        self
    }
}

impl<M, P> BusIF for MMU<M, P>
where
    M: Dev + BusIF,
    P: Dev + BusIF,
{
    fn get_origin(&self) -> u64 {
        0
    }

    fn get_end(&self) -> u64 {
        u64::MAX
    }

    fn load(&self, addr: u64, size: usize) -> Result<u64, ()> {
        if self.mem.get_origin() <= addr && self.mem.get_end() > addr + (size / 8) as u64 {
            self.mem.load(addr, size)
        } else if self.periph.get_origin() <= addr
            && self.periph.get_end() > addr + (size / 8) as u64
        {
            self.periph.load(addr, size)
        } else {
            Err(())
        }
    }

    fn store(&mut self, addr: u64, size: usize, val: u64) -> Result<(), ()> {
        if self.mem.get_origin() <= addr && self.mem.get_end() > addr + (size / 8) as u64 {
            self.mem.store(addr, size, val)
        } else if self.periph.get_origin() <= addr
            && self.periph.get_end() > addr + (size / 8) as u64
        {
            self.periph.store(addr, size, val)
        } else {
            Err(())
        }
    }
}
