// <T> : T 라는 type 선언
fn make_tuple01<T>(x: T, y: T) -> (T, T) {
    (x, y)
}

// <T, U> : 서로 다른 type일 때
fn make_tuple02<T, U>(x: T, y: U) -> (T, U) {
    (x, y)
}

pub fn generic_function() {
    // type 명시 (강제)
    let t01 = make_tuple01::<i32>(10, 20);
    // type 생략 (컴파일러 추론)
    let t02 = make_tuple01("a", "b");
    println!("{:?}", t01);
    println!("{:?}", t02);

    let t03 = make_tuple02(1, "one");
    println!("{:?}", t03);
}
