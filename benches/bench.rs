#![feature(stdarch_x86_avx512)]

use std::{hint::black_box, time::Duration};
use ring_arith::{cyclotomic_ring::*, hexl::bindings::{eltwise_add_mod, eltwise_reduce_mod}};
use criterion::{criterion_group, criterion_main, Criterion};

const N: usize = 128;
const MOD_Q_BIG: u64 = 4546383823830515713; // Example modulus
const MOD_Q: u64 = 1125899904679937; // Example modulus IFMA
// 1125899904679937
const K: usize = 2; 
const WIT_DIM: usize = 1048576; // 2^20
// const WIT_DIM: usize = 1024; // 2^10
const LOG_B:usize = 11; // 10 for unbalanced, 11 for signed


fn add_avx512(data: [u64; N], other: [u64; N]) -> [u64; N] {
    use std::arch::x86_64::*;
    #[cfg(target_feature = "avx512f")]
    unsafe {
        let mut result = [0u64; N];
        let chunks = N / 8;
        for i in 0..chunks {
            let a = _mm512_loadu_si512(data.as_ptr().add(i * 8) as *const _);
            let b = _mm512_loadu_si512(other.as_ptr().add(i * 8) as *const _);
            let sum = _mm512_add_epi64(a, b);
            _mm512_storeu_si512(result.as_mut_ptr().add(i * 8) as *mut _, sum);
        }
        return result;
    }
    panic!("AVX512 is not supported on this architecture");
}



fn bench_lfpp(c: &mut Criterion) {
    // 3.2999s
    // c.bench_function("lfp compute double commitment", |b| {
    //     b.iter_with_setup(
    //         || {
    //             let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
    //             let mut operand2 = CyclotomicRing::<MOD_Q, N>::random();
    //             (operand1, operand2)
    //         },
    //         |(mut operand1, mut operand2)| {
    //             unsafe {
    //                 for _ in 0..WIT_DIM * K * N {
    //                     eltwise_add_mod(
    //                         black_box(operand1.clone().data).as_mut_ptr(),
    //                         black_box(operand1.clone().data).as_ptr(),
    //                         black_box(operand2.clone().data).as_ptr(),
    //                         N as u64,
    //                         MOD_Q,
    //                     );
    //                 }
    //             }
    //         }, 
    //     )
    // });

    // const KAPPA_LFP: usize = 12;

    // // 1.9497 s
    // const L: usize = 3; // one for input two for accumulation
    // c.bench_function("lfp compute double commitment no mod", |b| {
    //     b.iter_with_setup(
    //         || {
    //             let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
    //             let mut operand2 = CyclotomicRing::<MOD_Q, N>::random();
    //             (operand1, operand2)
    //         },
    //         |(mut operand1, mut operand2)| {
    //             unsafe {
    //                 for _ in 0..WIT_DIM * K * N * L * KAPPA_LFP {
    //                     add_avx512(
    //                         black_box(operand1.clone().data),
    //                         black_box(operand2.clone().data),
    //                     );
    //                 }
    //             }
    //         }, 
    //     )
    // });

    // const KAPPA_LFPP: usize = 11;
    // // 1.6817 s
    // c.bench_function("lfpp compute extension commitment", |b| {
    //     b.iter_with_setup(
    //         || {
    //             let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
    //             operand1.to_incomplete_ntt_representation();
    //             let mut operand2 = CyclotomicRing::<MOD_Q, N>::random_bounded(2);
    //             operand2.to_incomplete_ntt_representation();
    //             let mut operand3 = CyclotomicRing::<MOD_Q, N>::random();
    //             (operand1, operand2, operand3)
    //         },
    //         |(mut operand1, mut operand2, mut operand3)| {
    //             for _ in 0..WIT_DIM * LOG_B {
    //                 operand3.clone().to_incomplete_ntt_representation();
    //             }
    //             for _ in 0..WIT_DIM * LOG_B * KAPPA_LFPP {
    //                 incomplete_ntt_multiplication(&mut operand1, &mut operand2, true);
    //             }
    //         }, 
    //     )
    // });

    // const KAPPA_DIL_LFP: usize = 7;
    //     // 1.9497 s
    // // const L: usize = 3; // one for input two for accumulation
    // c.bench_function("DIL lfp compute double commitment no mod", |b| {
    //     b.iter_with_setup(
    //         || {
    //             let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
    //             let mut operand2 = CyclotomicRing::<MOD_Q, N>::random();
    //             (operand1, operand2)
    //         },
    //         |(mut operand1, mut operand2)| {
    //             unsafe {
    //                 for _ in 0..WIT_DIM * K * N * L * KAPPA_DIL_LFP {
    //                     add_avx512(
    //                         black_box(operand1.clone().data),
    //                         black_box(operand2.clone().data),
    //                     );
    //                 }
    //             }
    //         }, 
    //     )
    // });

    // const KAPPA_DIL_LFPP: usize = 6;

    // 1.6817 s
    // c.bench_function("DIL lfpp compute extension commitment", |b| {
    //     b.iter_with_setup(
    //         || {
    //             let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
    //             operand1.to_incomplete_ntt_representation();
    //             let mut operand2 = CyclotomicRing::<MOD_Q, N>::random_bounded(2);
    //             operand2.to_incomplete_ntt_representation();
    //             let mut operand3 = CyclotomicRing::<MOD_Q, N>::random();
    //             (operand1, operand2, operand3)
    //         },
    //         |(mut operand1, mut operand2, mut operand3)| {
    //             for _ in 0..WIT_DIM * LOG_B {
    //                 operand3.clone().to_incomplete_ntt_representation();
    //             }
    //             for _ in 0..WIT_DIM * LOG_B * KAPPA_DIL_LFPP {
    //                 incomplete_ntt_multiplication(&mut operand1, &mut operand2, true);
    //             }
    //         }, 
    //     )
    // });

    // // 841.72 ms s
    // c.bench_function("lfpp compute extension commitment larger decomp", |b| {
    //     b.iter_with_setup(
    //         || {
    //             let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
    //             operand1.to_ntt_representation();
    //             let mut operand2 = CyclotomicRing::<MOD_Q, N>::random_bounded(4);
    //             operand2.to_ntt_representation();
    //             (operand1, operand2)
    //         },
    //         |(mut operand1, mut operand2)| {
    //             for _ in 0..WIT_DIM * LOG_B / 2 {
    //                 incomplete_ntt_multiplication(&mut operand1, &mut operand2.clone(), true);
    //             }

    //         }, 
    //     )
    // });

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

    c.bench_function("NTT transform big", |b| {
        b.iter_with_setup(
            || {
                let mut operand1 = CyclotomicRing::<MOD_Q_BIG, N>::random();
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
                    add_avx512(
                        black_box(operand1.clone().data),
                        black_box(operand2.clone().data),
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
