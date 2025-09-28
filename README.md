## Cyclofold: Efficient Lattice-based Folding

This repository serves as supplementary material for the anonymous submission to EUROCRYPT 2026.

In the Rust codebase, we evaluate the performance of cyclotomic ring arithmetic in a specific setting: modulus $q \approx 2^{50}$, degree $\varphi = 128$, where the cyclotomic ring does not fully split. For ring arithmetic, we utilize the HEXL library with custom Rust bindings. Arithmetic over quadratic extensions is performed manually. Benchmarking results are provided in the `report.out` file.

The Sage file includes measurements for all communication costs claimed in the paper.

### Running Benchmarks
    ```bash
    make hexl
    export LD_LIBRARY_PATH=./hexl-bindings/hexl/build/hexl/lib:$(pwd)
    cargo bench
    ```
