# aggproof

This folder contains an implementation of the aggregation proof using the `Nova` crate. The proof is constructed by incrementally summing the local updates that an aggregator receives from the client.

## Proof construction

Concretely, we implement a proof for the FedAvg algorithm. The prover claims the following statement:

$$
w_{glob} \leftarrow \sum_{k \in S} \frac{n_k}{n} w_k
$$

where $w_{glob}$ is the global model, $w_k$ is the local model of client $k$, and $\frac{n_k}{n}$ is a weight factor equal to the ratio of the data volume of client $k$ to the total data volume.

The proof is constructed as follows. For simplicity, we remove the weight factor $\frac{n_k}{n}$:

- Suppose we aggregate $n$ local models $\{w_0, w_1, ..., w_n\}$. Let $F$ represent the incremental addition of local models.
- At invocation $i$, $F_i$ represents the addition of $\{w_0, w_1, ..., w_i\}$.
- At invocation $n$, we obtain the global model by adding all local models $\{w_0, w_1, ..., w_n\}$.
- Due to implementation specifications, in this setup, the running input to $F$, denoted as $z_i$, will be set to $1$ if invocation $i$ is performed correctly. $z_i$ cannot hold the addition result of local models because of its high dimension.

## running the project

To build the project run:
```
cargo build
```
To run the project:
```
cargo run
```


