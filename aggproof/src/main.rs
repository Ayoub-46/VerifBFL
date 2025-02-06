use std::{fs::File, io::Write, time::Instant};

use bellpepper_core::{num::AllocatedNum, ConstraintSystem, SynthesisError};
use flate2::{write::ZlibEncoder, Compression};
use nova_snark::{provider::{Bn256EngineKZG, GrumpkinEngine}, traits::{circuit::{StepCircuit, TrivialCircuit}, snark::RelaxedR1CSSNARKTrait, Engine, Group}, CompressedSNARK, PublicParams, RecursiveSNARK};
use rand::Rng;


type E1 = Bn256EngineKZG;
type E2 = GrumpkinEngine;
type EE1 = nova_snark::provider::hyperkzg::EvaluationEngine<E1>;
type EE2 = nova_snark::provider::ipa_pc::EvaluationEngine<E2>;
type S1 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E1, EE1>; // non-preprocessing SNARK
type S2 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E2, EE2>; // non-preprocessing SNARK


#[derive(Clone, Debug)]
struct AddCircuit<G: Group>{
    a: Vec<G::Scalar>,
    b: Vec<G::Scalar>
}

impl<G: Group> AddCircuit<G>  {
    fn new(len: usize) -> Self {
        let mut a_vec = Vec::new();
        let mut b_vec = Vec::new();
        for i in 0..len {
            let mut rng = rand::thread_rng();
            let a_rand: u64 = rng.gen();
            let b_rand: u64 = rng.gen();
            let a = G::Scalar::from(a_rand);
            let b = G::Scalar::from(b_rand);
            a_vec.push(a);
            b_vec.push(b)
        }
        

        Self { a:a_vec, b:b_vec }
    }
}

impl<G: Group> StepCircuit<G::Scalar> for AddCircuit<G> {
    fn arity(&self) -> usize {
        1
    }

    fn synthesize<CS: ConstraintSystem<G::Scalar>>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<G::Scalar>],
      ) -> Result<Vec<AllocatedNum<G::Scalar>>, SynthesisError> {
        assert_eq!(self.a.len(), self.b.len());

        for i in 0..self.a.len(){
            let a = AllocatedNum::alloc(cs.namespace(|| format!("a")), || Ok(self.a[i]))?;

            let b = AllocatedNum::alloc(cs.namespace(|| format!("b")), || Ok(self.b[i]))?;

            let c = AllocatedNum::alloc(
                cs.namespace(|| format!("c")),
                || Ok(self.a[i] * self.b[i]),
            )?;

            cs.enforce(
                || format!("c == a * b"),
                |lc| lc + a.get_variable(),
                |lc| lc + b.get_variable(),
                |lc| lc + c.get_variable(),
            );

        }
        
        Ok(z.to_vec())
    }
}

fn main() {
    let num_steps = 5;
   
    
        let circuit_primary = AddCircuit::new(8000);
        let circuit_secondary = TrivialCircuit::default();

        println!(
            "Proving {} add ops ",
            num_steps
        );
        // produce public parameters
        let start = Instant::now();
        println!("Producing public parameters...");
        let pp = PublicParams::<
            E1,
            E2,
            AddCircuit<<E1 as Engine>::GE>,
            TrivialCircuit<<E2 as Engine>::Scalar>,
        >::setup(
            &circuit_primary,
            &circuit_secondary,
            &*S1::ck_floor(),
            &*S2::ck_floor(),
        )
        .unwrap();
        println!("PublicParams::setup, took {:?} ", start.elapsed());

        let circuits = (0..num_steps)
            .map(|_| AddCircuit::new(8000))
            .collect::<Vec<_>>();

        type C1 = AddCircuit<<E1 as Engine>::GE>;
        type C2 = TrivialCircuit<<E2 as Engine>::Scalar>;

        println!("Generating a RecursiveSNARK...");
        let mut recursive_snark: RecursiveSNARK<E1, E2, C1, C2> =
            RecursiveSNARK::<E1, E2, C1, C2>::new(
                &pp,
                &circuits[0],
                &circuit_secondary,
                &[<E1 as Engine>::Scalar::zero()],
                &[<E2 as Engine>::Scalar::zero()],
            )
            .unwrap();

        let start = Instant::now();
        for circuit_primary in circuits.iter() {
            let res = recursive_snark.prove_step(&pp, circuit_primary, &circuit_secondary);
            assert!(res.is_ok());
        }
        println!(
            "RecursiveSNARK::prove {} add ops: took {:?} ",
            num_steps,
            start.elapsed()
        );

        // verify the recursive SNARK
        println!("Verifying a RecursiveSNARK...");
        let res = recursive_snark.verify(
            &pp,
            num_steps,
            &[<E1 as Engine>::Scalar::from(0)],
            &[<E2 as Engine>::Scalar::from(0)],
        );
        println!("RecursiveSNARK::verify: {:?}", res.is_ok(),);
        assert!(res.is_ok());

        // produce a compressed SNARK
        println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
        let (pk, vk) = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp).unwrap();

        let start = Instant::now();

        let res = CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &pk, &recursive_snark);
        println!(
            "CompressedSNARK::prove: {:?}, took {:?}",
            res.is_ok(),
            start.elapsed()
        );
        assert!(res.is_ok());
        let compressed_snark = res.unwrap();

        let file_path = "compressedsnark.json";
        let mut file = File::create(file_path).expect("Unable to create file");

        let serialized_data = serde_json::to_string_pretty(&compressed_snark).expect("Unable to serialize data");
        file.write_all(serialized_data.as_bytes()).expect("Unable to write to file");

        let file_path = "verifierkey.json";
        let mut file = File::create(file_path).expect("Unable to create file");

        let serialized_data = serde_json::to_string_pretty(&vk).expect("Unable to serialize data");
        file.write_all(serialized_data.as_bytes()).expect("Unable to write to file");

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        bincode::serialize_into(&mut encoder, &compressed_snark).unwrap();
        let compressed_snark_encoded = encoder.finish().unwrap();
        println!(
            "CompressedSNARK::len {:?} bytes",
            compressed_snark_encoded.len()
        );

        // verify the compressed SNARK
        println!("Verifying a CompressedSNARK...");
        let start = Instant::now();
        let res = compressed_snark.verify(
            &vk,
            num_steps,
            &[<E1 as Engine>::Scalar::from(0)],
            &[<E2 as Engine>::Scalar::from(0)],
        );
        println!(
            "CompressedSNARK::verify: {:?}, took {:?}",
            res.is_ok(),
            start.elapsed()
        );
        assert!(res.is_ok());
        println!("=========================================================");
   
}

