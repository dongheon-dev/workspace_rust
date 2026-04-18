fn main() {

    // let : immutable
    let x = 1;
    println!("x = {}", x);
    // 대입 불가!
    // x = 2;

    // let mut : mutable
    let mut y = 2;
    println!("y = {}", y);
    y = 3;
    println!("y = {}", y);
    // y = "hello";

    // shadowing : 같은 이름으로 새로운 변수 생성
    // -> let mut 은 같은 type의 값으로만 변경 가능, shadowing은 새로운 변수 생성
    let z = 4;
    println!("z = {}", z);
    let z = "hello";
    println!("z = {}", z);

    println!("---");

    // scalar type
    // 정수 (i32)
    let a: i32 = 11;

    // 실수 (f64)
    let b: f64 = 23.45;

    // 논리 (bool)
    let c: bool = true;

    // 문자 (char는 4바이트, 유니코드)
    let d: char = 'd';

    println!("{}, {}, {}, {}", a, b, c, d);

    println!("---");

    // compound type
    // tuple : ()
    let tp:(i32, char) = (1, 'a');
    // index로 접근
    println!("{}", tp.1);
    println!("{}", tp.0);

    // 구조분해
    let (e, f) = tp;
    println!("{}, {}", e, f);

    // array : []
    let arr = [1, 2, 3];
    // {:?} : debug trait
    println!("{:?}", arr);
    // {:#?} : pretty print
    println!("{:#?}", arr);
    println!("{}", arr[2]);

    let result = add(4, 5);
    println!("{}", result);
}

fn add(a: i32, b: i32) -> i32 {
    // ; (semicolon) 없음!! -> return
    a + b
}