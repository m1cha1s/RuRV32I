use crate::vm::{Core, CoreState, Dev};

mod inst;

pub struct RV32I {
    state: CoreState,

    x: [u32; 32],
    pc: u32,
}

impl Dev for RV32I {
    fn new() -> Self {
        Self {
            state: CoreState::Stopped(0),
            x: [0; 32],
            pc: 0,
        }
    }

    fn reset(&mut self) -> &mut Self {
        self.state = CoreState::Stopped(self.pc as u64);
        self.x = [0; 32];
        self.pc = 0;

        self
    }
}

impl Core for RV32I {
    fn tick<M: Dev + crate::vm::BusIF, P: Dev + crate::vm::BusIF>(
        &mut self,
        cycles: u64,
        mmu: &mut crate::mmu::MMU<M, P>,
    ) -> crate::vm::CoreState {
        todo!()
    }

    fn get_state(&self) -> crate::vm::CoreState {
        self.state
    }

    fn get_pc(&self) -> u64 {
        self.pc as u64
    }
}
