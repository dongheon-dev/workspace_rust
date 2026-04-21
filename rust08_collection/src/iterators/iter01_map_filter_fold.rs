pub fn map_filter_fold() {
    let vec01 = vec![1, 2, 3, 4, 5];

    // map: 값 변환
    let vec02: Vec<i32> = vec01.iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", vec02);

    // filter: 조건 필터링
    let vec03: Vec<i32> = vec01.iter()
        .filter(|x| *x % 2 == 0)
        .cloned()
        .collect();

    println!("{:?}", vec03);

    // fold : 누적 연산
    let sum = vec01.iter().fold(0, |acc, x| acc + x);
    println!("{sum}");
    let product = vec01.iter().fold(1, |acc, x| acc * x);
    println!("{product}");
}
