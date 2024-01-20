use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, Value},
    plonk::{ConstraintSystem, Error, TableColumn, Column, Fixed},
};

#[derive(Debug, Clone)]
pub struct LimitIntTable<F: FieldExt, const MAX: usize> {
    pub value: Column<Fixed>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const MAX: usize> LimitIntTable<F, MAX> {
    pub fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let value = meta.fixed_column();
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_region(
            || "load table",
            |mut table| {
                let mut offset = 0;
                for value in 0..MAX {
                    table.assign_fixed(
                        || "num_bits",
                        self.value,
                        offset,
                        || Value::known(F::from(value as u64)),
                    )?;
                    offset += 1;
                }
                Ok(())
            },
        )
    }
}
