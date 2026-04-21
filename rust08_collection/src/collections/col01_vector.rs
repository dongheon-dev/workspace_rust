pub fn vec_test() {
    // vec : 동적 배열 -> heap에 저장

    // vec 생성
    let mut vec01 = Vec::new();
    vec01.push(1);
    vec01.push(2);
    vec01.push(3);

    println!("{:?}", vec01);

    // vec! (macro) 사용해서 생성
    let vec02 = vec![3, 4, 5, 6];
    println!("{:?}", vec02);

    // indexing (panic! : 위험)
    println!("{}", vec02[1]);

    // get (option return : 안전)
    match vec02.get(1) {
        Some(v) => println!("{v}"),
        None => println!("none"),
    }

    // immutable borrow iteration
    for i in &vec02 {
        println!("{i}");
    }

    // mutable borrow iteration
    let mut vec03 = vec![7, 8, 9];

    for i in &mut vec03 {
        // dereference 해서 값 수정
        *i *= 10;
    }

    println!("{:?}", vec03);
}
