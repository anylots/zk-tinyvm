use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::bn256::Fr};

mod app;

fn main() {
    let k = 9;
    const SUM: usize = 15; // 3-bit value
    const MAX: usize = 10; // 3-bit value
    let circuit = app::zk_tinyvm::SumCircuit::<Fr, SUM, MAX> {
        a: Value::known(Fr::from(8 as u64).into()),
        b: Value::known(Fr::from(7 as u64).into()),
    };
    // let snark = gen_snark_shplonk(params, pk, circuit, &mut rng, None::<String>);

    let prover = MockProver::run(k, &circuit, vec![]).unwrap();
    assert_eq!(prover.verify().is_ok(), true);
    println!("{}", "verify success");
}
