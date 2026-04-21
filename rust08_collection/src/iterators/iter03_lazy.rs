pub fn lazy_test() {
    let vec = vec![1, 2, 3, 4];
    let iter = vec.iter().map(|x| {
        x * 2
    });
    // iterator 아직 실행안됨
    println!("{:?}", vec);

    // lazy evaluation (collect 시 실행)
    let result: Vec<i32> = iter.collect();
    println!("{:?}", result);
}
