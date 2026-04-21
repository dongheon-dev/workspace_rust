pub fn iter_types_test() {
    // iterator ownership
    // 1. iter -> borrow
    let vec01 = vec![1, 2, 3];
    for i in vec01.iter() {
        println!("{i}");
    }

    // 2. into_iter -> ownership move
    let vec02 = vec![1, 2, 3];
    for i in vec02.into_iter() {
        println!("{i}");
    }

    // 3. iter_mut -> modify
    let mut vec03 = vec![1, 2, 3];
    for i in vec03.iter_mut() {
        *i *= 10;
    }
    println!("{:?}", vec03);
}

pub fn flatmap_test() {
    let v = vec![vec![1, 2], vec![3, 4], vec![5]];

    // flat_map : flatten
    let flat: Vec<i32> = v.into_iter()
        .flat_map(|x| x)
        .collect();

    println!("flat: {:?}", flat);
}
