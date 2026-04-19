# Rust02_Basics

[공식문서](https://doc.rust-lang.org/book/ch03-02-data-types.html)

### 1. variable

**let**  : immutable

**let mut** : mutable

**shadowing** : 같은 이름의 새로운 변수 선언



### 2. type

- **Scalar** : 값 하나를 담을 수 있는 변수
  - 정수 : i32
  - 실수 : f64
  - 논리 : bool
  - 문자 : char
- **Compound** : 여러 개의 값을 담을 수 있는 변수
  - tuple : 서로 다른 타입의 값을 담을 때
  - array : 같은 타입의 값을 담을 때

- **String**
  - &str : 문자열 슬라이스 (불변, 리터럴)
  - String : 힙에 저장되는 가변 문자열 (ownership에서 자세히)



*integer types*

| Length                 | Signed  | Unsigned |
| ---------------------- | ------- | -------- |
| 8-bit                  | `i8`    | `u8`     |
| 16-bit                 | `i16`   | `u16`    |
| 32-bit                 | `i32`   | `u32`    |
| 64-bit                 | `i64`   | `u64`    |
| 128-bit                | `i128`  | `u128`   |
| Architecture-dependent | `isize` | `usize`  |



*integer literals*

| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |






### 3. function

```rust
fn function (){}

fn function (a:i32){}

fn function (a:i32) -> i32{}
```







*main.rs*

```rust
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

    // 문자열
    // &str : 문자열 슬라이스 (불변, 리터럴)
    // String : 힙에 저장되는 가변 문자열 (ownership에서 자세히)
    let s1: &str = "hello";
    println!("{}", s1);
    let s2 = String::from("hello");
    println!("{}", s2);
}

fn add(a: i32, b: i32) -> i32 {
    // ; (semicolon) 없음!! -> return
    a + b
}
```

