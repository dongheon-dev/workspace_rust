pub fn for_statements() {
    let arr = ["hello", "world", "rust", "is", "easy"];

    for value in arr {
        println!("{value}");
    }

    for i in 1..5 {
        println!("{i}");
    }

    for i in 1..=5 {
        println!("{i}");
    }

    for i in (1..=5).rev() {
        println!("{i}");
    }

    // index 필요할 때
    // .iter : iterator
    // .enumerate : 각 element 에 index 추가
    for (i, value) in arr.iter().enumerate() {
        println!("index={}, value={}", i, value);
    }
}
