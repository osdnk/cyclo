#!/bin/bash -l
#SBATCH --mem=10G
#SBATCH --cpus-per-task 1
#SBATCH --time=01:00:20
#SBATCH --constraint=emerald
module load gcc cmake
make hexl-triton
make wrapper
export LD_LIBRARY_PATH=./hexl-bindings/hexl/build/hexl/lib:$(pwd)
# export RUSTFLAGS="-C linker=gcc"
RUSTFLAGS="-C target-feature=+avx2,+avx,+sse2,+avx512f,+avx512bw -C linker=gcc" cargo bench



