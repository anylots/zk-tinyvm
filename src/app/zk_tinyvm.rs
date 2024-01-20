use crate::app::add::SumCheckConfig;
use halo2_proofs::{
    circuit::{floor_planner::V1, Layouter, Value},
    halo2curves::FieldExt,
    plonk::{Assigned, Circuit, ConstraintSystem, Error},
};

#[derive(Default)]
pub struct SumCircuit<F: FieldExt, const SUM: usize, const MAX: usize> {
    pub a: Value<Assigned<F>>,
    pub b: Value<Assigned<F>>,
}

impl<F: FieldExt, const SUM: usize, const MAX: usize> Circuit<F> for SumCircuit<F, SUM, MAX> {
    type Config = SumCheckConfig<F, SUM, MAX>;
    type FloorPlanner = V1;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let a = meta.advice_column();
        let b = meta.advice_column();
        SumCheckConfig::configure(meta, a, b)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        config.table.load(&mut layouter)?;
        config.assign(layouter.namespace(|| "Assign value"), self.a, self.b)?;
        config.assign_lookup(layouter.namespace(|| "Assign lookup value"), self.a)?;
        Ok(())
    }
}
