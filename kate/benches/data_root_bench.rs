use criterion::{black_box, criterion_group, criterion_main, Criterion};
use beefy_merkle_tree::merkle_root;
use sp_core::H256;
use da_primitives::data_proof::HasherSha256;
use std::vec::IntoIter;
use rand::Rng;


fn root<I: Iterator<Item = Vec<u8>>>(data: I) -> H256 {
    let root = merkle_root::<HasherSha256, _, _>(data).into();
    root
}

fn input2() -> IntoIter<Vec<u8>>{
    let input2 = generate_data_vec();
    let mut test:Vec<Vec<u8>> = Vec::new();
    (0..100).for_each(|_| { 
        test.push(input2.clone());
    });
    test.into_iter()
}

fn generate_data_vec() -> Vec<u8>{
    let mut vector:Vec<u8> = Vec::new();
    (0..20000).for_each(|_|{
        let ran:u8 = rand::thread_rng().gen();
        vector.push(ran);
    });
    vector
}


fn criterion_benchmark(c: &mut Criterion){
    let input = vec![generate_data_vec()].into_iter();


    let inp = input2();
    c.bench_function("single_data", |c| c.iter(||root(black_box(input.clone()))));
    c.bench_function("multi_data", |c| c.iter(||root(inp.clone())));

}


criterion_group! {name = data_root_bench; config = Criterion::default().sample_size(50); targets = criterion_benchmark}
criterion_main!(data_root_bench);
