# trainingproof


This folder contains an implementation of the accuracy proof using the `Nova` crate. The proof is constructed by calculating the precision of a model in an incremental fashion.

## Proof construction

The trainers, upon local training, generate proofs to attest to the accuracy of their trained model. The statement that a prover claims is that their local model $M$ has a given accuracy $Acc$. To construct such proofs, we express the accuracy computation as an incrementally verifiable computation (IVC) as introduced in the previous section. The intuition is presented in the following:

- Let $F$ represent the execution of one inference operation of our model. Notably, an invocation of $F$ refers to the computation of a model prediction on an input.
- In our construction, $z_i$ represents a counter for the number of correct predictions accomplished in the set of invocations $\{0,1,...,i-1\}$ of $F$.
- Finally, invocation $n$, $F^{(n)}$ represents the execution of $n$ invocations of $F$, i.e., the inference of $n$ inputs to our model. Consequently, $z_n$ will hold the total number of correct predictions for our model. The accuracy of the model can be obtained as follows:

$$
Acc = \frac{\text{number of correct predictions}}{\text{number of total predictions}} = \frac{z_n}{n}
$$

## running the project

To build the project run:
```
cargo build
```
To run the project:
```
cargo run
```
