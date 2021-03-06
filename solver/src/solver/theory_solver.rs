use crate::solver::{Binding, BindingResult};
use crate::{Contradiction, Theory};
use aries_backtrack::ObsTrail;
use aries_backtrack::{Backtrack, DecLvl};
use aries_model::bounds::Bound;
use aries_model::expressions::ExprHandle;
use aries_model::int_model::DiscreteModel;
use aries_model::Model;

// TODO: remove this useless layer
pub struct TheorySolver {
    pub theory: Box<dyn Theory>,
}

impl TheorySolver {
    pub fn new(theory: Box<dyn Theory>) -> TheorySolver {
        TheorySolver { theory }
    }

    pub fn bind(
        &mut self,
        lit: Bound,
        expr: ExprHandle,
        interner: &mut Model,
        queue: &mut ObsTrail<Binding>,
    ) -> BindingResult {
        self.theory.bind(lit, expr, interner, queue)
    }

    pub fn process(&mut self, model: &mut DiscreteModel) -> Result<(), Contradiction> {
        self.theory.propagate(model)
    }

    pub fn print_stats(&self) {
        self.theory.print_stats()
    }
}

impl Backtrack for TheorySolver {
    fn save_state(&mut self) -> DecLvl {
        self.theory.save_state()
    }

    fn num_saved(&self) -> u32 {
        self.theory.num_saved()
    }

    fn restore_last(&mut self) {
        self.theory.restore_last()
    }

    fn restore(&mut self, saved_id: DecLvl) {
        self.theory.restore(saved_id)
    }
}
