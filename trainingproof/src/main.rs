use std::{fs::File, io::{BufReader, Write}, process::Output, time::Instant, u64};

use bellpepper_core::{num::AllocatedNum, ConstraintSystem, LinearCombination, SynthesisError};
use flate2::{write::ZlibEncoder, Compression};
use nova_snark::{provider::{Bn256EngineKZG, GrumpkinEngine}, traits::{circuit::{StepCircuit, TrivialCircuit}, snark::RelaxedR1CSSNARKTrait, Engine, Group}, CompressedSNARK, PublicParams, RecursiveSNARK, VerifierKey};
use pasta_curves::group::ff::derive::bitvec::vec;
use serde::Deserialize;


type E1 = Bn256EngineKZG;
type E2 = GrumpkinEngine;
type EE1 = nova_snark::provider::hyperkzg::EvaluationEngine<E1>;
type EE2 = nova_snark::provider::ipa_pc::EvaluationEngine<E2>;
type S1 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E1, EE1>; // non-preprocessing SNARK
type S2 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E2, EE2>; 

/*#[derive(Debug, Deserialize)]
struct Image{
    img: Vec<Vec<Vec<u64>>>,
}*/

#[derive(Clone, Debug)]
struct InferenceIteration<G: Group>{
    image: Vec<Vec<Vec<G::Scalar>>>,
    label: G::Scalar,
}

impl<G: Group> InferenceIteration<G> {
    fn new(i: usize) -> Self {
        /*
            read image from file
            transform elements to group elements
            return InferenceIteration object */
        let path = format!("inputs/images/image_{}.json", i);
        let file = File::open(path).expect("File not found");
        let reader = BufReader::new(file);
        let img: Vec<Vec<Vec<u64>>> = serde_json::from_reader(reader).expect("Error reading file content");

        let mut res_img: Vec<Vec<Vec<G::Scalar>>> = vec![];
        for i in 0..img.len() {
            let mut temp1 = vec![];
            for j in 0..img[i].len(){
                let mut temp2 = vec![];
                for k in 0..img[i][j].len(){
                    //let elem = slice_to_number(img[i][j][k].to_le_bytes());

                    temp2.push(G::Scalar::from(img[i][j][k]));   
                }
                temp1.push(temp2);
            }
            res_img.push(temp1);
        }

        Self{
            image: res_img,
            label: G::Scalar::from(u64::MAX)
        }
        
    }
}

#[derive(Clone, Debug)]
/* this structure holds the model weights*/
struct ModelWeight<G: Group>{
    weights_layer1: Vec<Vec<Vec<Vec<G::Scalar>>>>,
    bias_layer1: Vec<G::Scalar>,
    weights_layer2: Vec<Vec<Vec<Vec<G::Scalar>>>>,
    bias_layer2: Vec<G::Scalar>,
    weights_layer3: Vec<Vec<G::Scalar>>,
    bias_layer3: Vec<G::Scalar>,
    weights_layer4: Vec<Vec<G::Scalar>>,
    bias_layer4: Vec<G::Scalar>
}

impl<G: Group> ModelWeight<G> {
    fn new() -> Self {
        // this function has the same logic as in InferenceIteration
        
        // first two layers
        let file = File::open("./inputs/weights/layer1_w.json").expect("File not found");
        let reader = BufReader::new(file);
        let weights_layer1: Vec<Vec<Vec<Vec<u64>>>> = serde_json::from_reader(reader).expect("Error reading file content");

        let file = File::open("./inputs/weights/layer2_w.json").expect("File not found");
        let reader = BufReader::new(file);
        let weights_layer2: Vec<Vec<Vec<Vec<u64>>>> = serde_json::from_reader(reader).expect("Error reading file content");
        
        let file = File::open("./inputs/weights/layer3_in.json").expect("File not found");
        let reader = BufReader::new(file);
        let weights_layer3: Vec<Vec<u64>> = serde_json::from_reader(reader).expect("Error reading file content");

        let file = File::open("./inputs/weights/layer4_in.json").expect("File not found");
        let reader = BufReader::new(file);
        let weights_layer4: Vec<Vec<u64>> = serde_json::from_reader(reader).expect("Error reading file content");
        
        let file = File::open("./inputs/weights/layer1_b.json").expect("File not found");
        let reader = BufReader::new(file);
        let bias_layer1: Vec<u64> = serde_json::from_reader(reader).expect("Error reading file content");

        let file = File::open("./inputs/weights/layer2_b.json").expect("File not found");
        let reader = BufReader::new(file);
        let bias_layer2: Vec<u64> = serde_json::from_reader(reader).expect("Error reading file content");
        
        let file = File::open("./inputs/weights/layer3_out.json").expect("File not found");
        let reader = BufReader::new(file);
        let bias_layer3: Vec<u64> = serde_json::from_reader(reader).expect("Error reading file content");

        let file = File::open("./inputs/weights/layer4_out.json").expect("File not found");
        let reader = BufReader::new(file);
        let bias_layer4: Vec<u64> = serde_json::from_reader(reader).expect("Error reading file content");
        
        let mut res_weights_layer1 = vec![];

        for i in 0..weights_layer1.len() {
            let mut temp1 = vec![];
            for j in 0..weights_layer1[i].len(){
                let mut temp2 = vec![];
                for k in 0..weights_layer1[i][j].len(){
                    let mut temp3 = vec![];
                    for l in 0..weights_layer1[i][j][k].len(){
                        temp3.push(G::Scalar::from(weights_layer1[i][j][k][l])); 
                    }
                    temp2.push(temp3);  
                }
                temp1.push(temp2);
            }
            res_weights_layer1.push(temp1);
        }

        let mut res_weights_layer2 = vec![];

        for i in 0..weights_layer2.len() {
            let mut temp1 = vec![];
            for j in 0..weights_layer2[i].len(){
                let mut temp2 = vec![];
                for k in 0..weights_layer2[i][j].len(){
                    let mut temp3 = vec![];
                    for l in 0..weights_layer2[i][j][k].len(){
                        temp3.push(G::Scalar::from(weights_layer2[i][j][k][l])); 
                    }
                    temp2.push(temp3);  
                }
                temp1.push(temp2);
            }
            res_weights_layer2.push(temp1);
        }

        let mut res_bias_layer1 = vec![];
        let mut res_bias_layer2 = vec![];
        let mut res_bias_layer3 = vec![];
        let mut res_bias_layer4 = vec![];

        for (w, res) in [
            (bias_layer1, &mut res_bias_layer1), 
            (bias_layer2, &mut res_bias_layer2), 
            (bias_layer3, &mut res_bias_layer3), 
            (bias_layer4, &mut res_bias_layer4)] {

            for i in 0..w.len(){
                res.push(G::Scalar::from(w[i]));
            }
        }

        let mut res_weights_layer3 = vec![];
        let mut res_weights_layer4 = vec![];

        for (w, res) in [
            (weights_layer3, &mut res_weights_layer3), 
            (weights_layer4, &mut res_weights_layer4)] {

            for i in 0..w.len(){
                let mut temp = vec![];
                for j in 0..w[i].len(){
                    temp.push(G::Scalar::from(w[i][j]));
                }
                res.push(temp);    
            }
        }

        Self {
            weights_layer1: res_weights_layer1,
            bias_layer1: res_bias_layer1,
            weights_layer2: res_weights_layer2,
            bias_layer2: res_bias_layer2,
            weights_layer3: res_weights_layer3,
            bias_layer3: res_bias_layer3,
            weights_layer4: res_weights_layer4,
            bias_layer4: res_bias_layer4

        }        
    }
}

#[derive(Clone, Debug)]
struct InferenceCircuit<G:Group>{
    data: InferenceIteration<G>,
    weights: ModelWeight<G>,
}

impl<G: Group> InferenceCircuit<G> {
    fn new(index_of_image: usize) -> Self {
        // initialize all circuits data
        let data = InferenceIteration::new(index_of_image);
        
        let weights: ModelWeight<G> = ModelWeight::new();
        
        Self{
            data,
            weights
        }
    }
}

impl<G: Group> StepCircuit<G::Scalar> for InferenceCircuit<G>  {
    fn arity(&self) -> usize {
        1
    }

    fn synthesize<CS: ConstraintSystem<G::Scalar>>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<G::Scalar>],
      ) -> Result<Vec<AllocatedNum<G::Scalar>>, SynthesisError> {
        /* implement circuit logic
            for each image compute its inference and compare with the label
            if the result is a true positive, increment z */

        // padding
        let depth = self.data.image.len();
        let rows = self.data.image[0].len();
        let cols = self.data.image[0][0].len();
        let mut padded_image = vec![vec![vec![G::Scalar::from(0); cols + 2]; rows + 2]; depth];

        for d in 0..depth {
            for i in 0..rows {
                for j in 0..cols {
                    padded_image[d][i + 1][j + 1] = self.data.image[d][i][j];
                }
            }
        }

        // conv1
        let feature_maps = self.weights.weights_layer1.len();
        let kernel_depth = self.weights.weights_layer1[0].len();
        let kernel_size = self.weights.weights_layer1[0][0].len();
        let output_size = padded_image[0].len() - kernel_size + 1;
        let mut conv1_output = vec![vec![vec![G::Scalar::from(1); output_size]; output_size]; feature_maps];

        for f in 0..feature_maps {
            for i in 0..output_size {
                for j in 0..output_size {
                    let mut conv_result=G::Scalar::from(0);

                    for d in 0..kernel_depth {
                        for ki in 0..kernel_size {
                            for kj in 0..kernel_size {
                                // let input_value = AllocatedNum::alloc(cs.namespace(|| format!("padded_image[{}][{}][{}]", d, i+ki, j+kj)), || Ok(padded_image[d][i + ki][j + kj]))?;
                                // let kernel_value = AllocatedNum::alloc(cs.namespace(|| format!("layer1_weight[{}][{}][{}][{}]", f, d, ki, kj)), || Ok(self.weights.weights_layer1[f][d][ki][kj]))?;
                                conv_result = conv_result + padded_image[d][i + ki][j + kj] * self.weights.weights_layer1[f][d][ki][kj] ;
                            }
                        }
                    }
                    
                    conv1_output[f][i][j] = conv_result + self.weights.bias_layer1[f];
                }
            }
        }

        // max pooling

        let mut output1 = Vec::new();
        let len = conv1_output[0].len();
        let poolsize = 2;
        let stride = 1;

        for f in 0..feature_maps{
            let mut col = Vec::new();
            for i in (0..len - poolsize +1).step_by(stride) {
                let mut row: Vec<<G as Group>::Scalar> = Vec::new();
                for j in (0..len - poolsize + 1).step_by(stride){
                    let mut max_value = conv1_output[f][i][j];
                    for ki in 0..poolsize {
                        for kj in 0..poolsize {
                            
                            max_value = conv1_output[f][i + ki][j + kj];
                            
                        }
                        
                    }
                    row.push(max_value);
                }
                col.push(row);
            }
            output1.push(col);

        }
        // padding

        let depth = output1.len();
        let rows = output1[0].len();
        let cols = output1[0][0].len();
        let mut padded_image = vec![vec![vec![G::Scalar::from(0); cols + 2]; rows + 2]; depth];

        for d in 0..depth {
            for i in 0..rows {
                for j in 0..cols {
                    padded_image[d][i + 1][j + 1] = output1[d][i][j];
                }
            }
        }

        // conv2

        let feature_maps = self.weights.weights_layer2.len();
        let kernel_depth = self.weights.weights_layer2[0].len();
        let kernel_size = self.weights.weights_layer2[0][0].len();
        let output_size = padded_image[0].len() - kernel_size + 1;
        let mut conv2_output = vec![vec![vec![G::Scalar::from(1); output_size]; output_size]; feature_maps];

        for f in 0..feature_maps {
            for i in 0..output_size {
                for j in 0..output_size {
                    let mut conv_result=G::Scalar::from(0);

                    for d in 0..kernel_depth {
                        for ki in 0..kernel_size {
                            for kj in 0..kernel_size {
                                // let input_value = AllocatedNum::alloc(cs.namespace(|| format!("padded_image[{}][{}][{}]", d, i+ki, j+kj)), || Ok(padded_image[d][i + ki][j + kj]))?;
                                // let kernel_value = AllocatedNum::alloc(cs.namespace(|| format!("layer1_weight[{}][{}][{}][{}]", f, d, ki, kj)), || Ok(self.weights.weights_layer1[f][d][ki][kj]))?;
                                conv_result = conv_result + padded_image[d][i + ki][j + kj] * self.weights.weights_layer2[f][d][ki][kj] ;
                            }
                        }
                    }
                    
                    conv2_output[f][i][j] = conv_result + self.weights.bias_layer2[f];
                }
            }
        }

        // max pooling

        let mut output2 = Vec::new();
        let len = conv2_output[0].len();
        let poolsize = 2;
        let stride = 1;

        for f in 0..feature_maps{
            let mut col = Vec::new();
            for i in (0..len - poolsize +1).step_by(stride) {
                let mut row: Vec<<G as Group>::Scalar> = Vec::new();
                for j in (0..len - poolsize + 1).step_by(stride){
                    let mut max_value = conv2_output[f][i][j];
                    for ki in 0..poolsize {
                        for kj in 0..poolsize {
                            
                            max_value = conv2_output[f][i + ki][j + kj];
                            
                        }
                        
                    }
                    row.push(max_value);
                }
                col.push(row);
            }
            output2.push(col);

        }


        // flatten

        let output: Vec<G::Scalar> = output2.into_iter()
        .flat_map(|vec2d| vec2d.into_iter())
        .flatten()
        .collect();

        // dense 1

        let mut out_dense1: Vec<G::Scalar> = vec![G::Scalar::from(0);self.weights.weights_layer3.len()];

        for (i, row) in self.weights.weights_layer3.iter().enumerate() {
            for (j, &weight) in row.iter().enumerate() {
                out_dense1[i] += output[j] * weight;
            }
            out_dense1[i] += self.weights.bias_layer3[i];
            
        }

        // dense 2

        let mut out_dense2: Vec<G::Scalar> = vec![G::Scalar::from(0);self.weights.weights_layer4.len()];

        for (i, row) in self.weights.weights_layer4.iter().enumerate() {
            for (j, &weight) in row.iter().enumerate() {
                out_dense2[i] += out_dense1[j] * weight;
            }
            out_dense2[i] += self.weights.bias_layer4[i];
            
        }

        let mut correct_prediction = false;

        for i in out_dense2.iter(){
            if (self.data.label == *i) {
                correct_prediction = true;
            }
        }

        if correct_prediction{

            
            let z_out = AllocatedNum::alloc(cs.namespace(|| "z_out"), || Ok(z[0].get_value().unwrap() + G::Scalar::from(1)))?;
        
            Ok([z_out].to_vec())
        }else{
            Ok(z.to_vec())
        }
        
    }

    
}



fn main() {
    let num_steps: usize = 4;
    
        let circuit_primary = InferenceCircuit::new(1);
        let circuit_secondary = TrivialCircuit::default();

        println!(
            "Proving {} inferences", num_steps
        );
        // produce public parameters
        let start = Instant::now();
        println!("Producing public parameters...");
        let pp = PublicParams::<
            E1,
            E2,
            InferenceCircuit<<E1 as Engine>::GE>,
            TrivialCircuit<<E2 as Engine>::Scalar>,
        >::setup(
            &circuit_primary,
            &circuit_secondary,
            &*S1::ck_floor(),
            &*S2::ck_floor(),
        ).unwrap()
        ;
        println!("PublicParams::setup, took {:?} ", start.elapsed());

        let circuits = (0..num_steps)
            .map(|i| InferenceCircuit::new(i))
            .collect::<Vec<_>>();

        type C1 = InferenceCircuit<<E1 as Engine>::GE>;
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
            "RecursiveSNARK::prove {} inference: took {:?} ",
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


        println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
        let (pk, vk) = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp).unwrap();

        
        let start = Instant::now();

        /*let file = File::open("compressedsnark.json").expect("unable to open file");
        let reader = BufReader::new(file);
        let compressed_snark : CompressedSNARK<E1, E2, C1, C2, S1, S2>= serde_json::from_reader(reader).unwrap();
        
        let file = File::open("verifierkey.json").expect("unable to open file");
        let reader = BufReader::new(file);
        let vk : VerifierKey<E1, E2, C1, C2, S1, S2>= serde_json::from_reader(reader).unwrap();*/
        
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



