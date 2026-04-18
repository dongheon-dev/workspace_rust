pub fn clone_str() {
    let s1 = String::from("hello");

    // 1. clone : heap 영역의 s1을 deep copy
    let s2 = s1.clone();

    // 2. s1과 s2는 서로 다른 메모리를 참조
    println!("{}", s1);
    println!("{}", s2);
}