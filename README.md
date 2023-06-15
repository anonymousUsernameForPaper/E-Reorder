# E-Reorder
Code and Benchmarks for Equidistant Reorder Operator for CGP

# Rust
The code is written in Rust only.  
For installation, see: https://github.com/rust-lang/rust/blob/master/README.md

# Building
You have to build everything yourself. You will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs/) is the simplest way to set this up on either Windows, Mac or Linux.

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release --features FEATURE
```
A `FEATURE` is either:
- vanilla: the baseline CGP implementation without extensions
- reorder: CGP with the Reorder extension, see: Analysis of Cartesian Genetic Programming’s Evolutionary Mechanisms, doi={10.1109/TEVC.2014.2324539}
- ereorder: CGP with the E-Reorder extension

# Usage
Run the build executable on your machine via:
```
./target/release/cgp
```
or 
```
./target/release/cgp.exe
```

Outputs will be placed into a folder called
`Experiments_Output`

You can configure the run via following command line arguments:
- `run-id`
  - The ID of the run
  - Only important for saving results
  - default: 0
- `dataset`
  - which dataset to use:
    - 0: Parity
    - 1: Encode
    - 2: Decode
    - 3: Multiply
  - default: 0
- `nbr-nodes`
  - the number of computational nodes for CGP
  - default: 100
- `cgp-type`
  - the CGP type used.
    - 0: Baseline
    - 2: Reorder
    - 3: E-Reorder
  - Only important for saving results
  - default: 2
- `mutation-type`
  - Single or Point mutation operator for CGP
    - 0: Single
    - 1: Point mutation
  - default: 0
- `mutation-prob`
  - mutation rate for point mutation.
  - only relevant if `mutation-type=1`
  - default: -1.0 



