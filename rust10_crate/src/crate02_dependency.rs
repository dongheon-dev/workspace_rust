use rand::{random, rng, RngExt};

pub fn rand_test(i: i32, j: i32) {
    let random_num:i32 = random();
    println!("{random_num}");

    let mut rng = rng();
    // i ~ j-1 까지
    let num01 = rng.random_range(i..j);
    // i ~ j 까지
    let num02 = rng.random_range(i..=j);
    println!("{num01}");
    println!("{num02}");
}