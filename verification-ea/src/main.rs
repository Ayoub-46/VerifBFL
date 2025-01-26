#[macro_use] extern crate rocket;



use bellpepper_core::num::AllocatedNum;
use bellpepper_core::{ConstraintSystem, SynthesisError};
use nova_snark::provider::{Bn256EngineKZG, GrumpkinEngine, PallasEngine, VestaEngine};
use nova_snark::traits::circuit::{StepCircuit, TrivialCircuit};
use nova_snark::traits::snark::RelaxedR1CSSNARKTrait;
use nova_snark::traits::{Engine, Group};
use nova_snark::{CompressedSNARK, PublicParams, VerifierKey};
use std::fs::File;
use std::io::BufReader;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;



type E1 = Bn256EngineKZG;
type E2 = GrumpkinEngine;
type EE1 = nova_snark::provider::hyperkzg::EvaluationEngine<E1>;
type EE2 = nova_snark::provider::ipa_pc::EvaluationEngine<E2>;
type S1 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E1, EE1>; // non-preprocessing SNARK
type S2 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E2, EE2>; // non-preprocessing SNARK

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
        let file = File::open(path).expect("unable to open file");
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




type C1 = InferenceCircuit<<E1 as Engine>::GE>;
type C2 = TrivialCircuit<<E2 as Engine>::Scalar>;

#[derive(Deserialize)]
struct ChainlinkEARequest {
    pub id: String,
    pub proof: CompressedSNARK<E1, E2, C1, C2, S1, S2>,
    pub meta: Option<serde_json::Value>,
    pub response_url: Option<String>
}

#[derive(Serialize)]
struct ComputeResponse {
    result: bool,
}

#[post("/compute", format = "json", data = "<compute_request>")]
fn compute(compute_request: Json<ChainlinkEARequest>) -> Json<ComputeResponse> {
    let circuit_primary = InferenceCircuit::new(1);
    let circuit_secondary = TrivialCircuit::default();

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
    ).unwrap();

    //let (pk, vk) = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp).unwrap();
    
    let file = File::open("verifierkey.json").expect("unable to open file");
    let reader = BufReader::new(file);
    let vk: VerifierKey<E1, E2, C1, C2, S1, S2> = serde_json::from_reader(reader).unwrap();
    
    let proof = &compute_request.proof;
    let res = proof.verify(&vk, 4, &[<E1 as Engine>::Scalar::from(0)], &[<E2 as Engine>::Scalar::from(0)]);
    // assert!(res.is_ok());
    Json(ComputeResponse { result: res.is_ok() })
}


// #[get("/")]
// fn hello_world() -> &'static str{
//     "Hello World"
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![compute])
}

