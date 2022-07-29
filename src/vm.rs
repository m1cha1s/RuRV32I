use crate::mmu::MMU;

pub trait Dev {
    fn new() -> Self;
    fn reset(&mut self) -> &mut Self;
}

pub trait BusIF {
    fn get_origin(&self) -> u64;
    fn get_end(&self) -> u64;

    fn load(&self, addr: u64, size: usize) -> Result<u64, ()>;
    fn store(&mut self, addr: u64, size: usize, val: u64) -> Result<(), ()>;

    fn tick(&mut self) -> &mut Self;
}

#[derive(Clone, Copy)]
pub enum CoreState {
    Running(u64),
    Waiting(u64),
    Exception(u64),
    Stopped(u64),
}

pub trait Core {
    fn tick<M: Dev + BusIF, P: Dev + BusIF>(
        &mut self,
        cycles: u64,
        mmu: &mut MMU<M, P>,
    ) -> CoreState;
    fn get_state(&self) -> CoreState;
    fn get_pc(&self) -> u64;
}

pub struct VM<C, M, P>
where
    C: Dev + Core,
    M: Dev + BusIF,
    P: Dev + BusIF,
{
    core: C,
    mmu: MMU<M, P>,
}

impl<C, M, P> VM<C, M, P>
where
    C: Dev + Core,
    M: Dev + BusIF,
    P: Dev + BusIF,
{
    pub fn new() -> Self {
        Self {
            core: C::new(),
            mmu: MMU::new(),
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.core.reset();
        self.mmu.reset();

        self
    }

    pub fn tick(&mut self, cycles: u64) -> &mut Self {
        let _core_state = self.core.tick(cycles, &mut self.mmu);

        self
    }
}
