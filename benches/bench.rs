#![feature(stdarch_x86_avx512)]

use std::{hint::black_box, time::Duration};
use ring_arith::{cyclotomic_ring::*, hexl::bindings::{eltwise_add_mod, eltwise_reduce_mod}};
use criterion::{criterion_group, criterion_main, Criterion};

const N: usize = 128;
const MOD_Q: u64 = 1125899906839937; // Example modulus IFMA, not fully splitting



fn bench_lfpp(c: &mut Criterion) {
    c.bench_function("NTT transform", |b| {
        b.iter_with_setup(
            || {
                let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
                (operand1)
            },
            |mut operand1| {
                operand1.to_incomplete_ntt_representation();
            }, 
        )
    });


    c.bench_function("NTT add no mod" , |b| {
        b.iter_with_setup(
            || {
                let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
                let mut operand2 = CyclotomicRing::<MOD_Q, N>::random();
                (operand1, operand2)
            },
            |(mut operand1, mut operand2)| {
                unsafe {
                    black_box(operand1.clone()).add_no_overflow(
                        black_box(operand2.clone()),
                    );
                }                
            }, 
        )
    });

    c.bench_function("NTT multiplication", |b| {
        b.iter_with_setup(
            || {
                let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
                operand1.to_incomplete_ntt_representation();
                let mut operand2: CyclotomicRing<1125899904679937, 128> = CyclotomicRing::<MOD_Q, N>::random();
                operand2.to_incomplete_ntt_representation();
                (operand1, operand2)
            },
            |(mut operand1, mut operand2)| {
                incomplete_ntt_multiplication(&mut operand1, &mut operand2, true);
            }, 
        )
    });

    c.bench_function("NTT reduction", |b| {
        b.iter_with_setup(
            || {
                let mut operand0 = CyclotomicRing::<MOD_Q_BIG, N>::random();
                let mut operand1 = CyclotomicRing::<MOD_Q_BIG, N>::new();
                operand1.data = operand0.data;
                (operand1)
            },
            |mut operand1| {
                unsafe {
                    eltwise_reduce_mod(
                        black_box(operand1.clone().data).as_mut_ptr(),
                        black_box(operand1.clone().data).as_ptr(),
                        N as u64,
                        MOD_Q,
                    );
                }
            }, 
        )
    });
}




fn configure_criterion() -> Criterion {
    Criterion::default().sample_size(10)
    .warm_up_time(Duration::from_secs(30))
}

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = bench_lfpp
}
criterion_main!(benches);
