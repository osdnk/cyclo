
#![feature(generic_const_exprs)]
use ring_arith::cyclotomic_ring::*;
use rand::Rng;
use tfhe_ntt::{prime::largest_prime_in_arithmetic_progression64, *};

fn main() {
    let mut rng = rand::thread_rng();
    let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
    let mut operand2 = CyclotomicRing::<MOD_Q, N>::random();
    operand1.to_incomplete_ntt_representation();
    operand2.to_incomplete_ntt_representation();
    let start = std::time::Instant::now();
    incomplete_ntt_multiplication(&mut operand1, &mut operand2, true);
    let duration = start.elapsed();
    println!("Time elapsed in multiplication() is: {:?}", duration);
}
