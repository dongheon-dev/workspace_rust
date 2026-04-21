# Rust07_ADT (Algebraic Data Type)

### 1. 대수적 데이터 타입

- **Product Type** (AND) : 여러 값을 동시에 포함 (struct, tuple)

- **Sum Type** (OR) : 여러 variant  중 하나 (enum, option, result) 

<br>



### 2. Struct

- 구조체 : 데이터를 구조화 -> 서로 관련된 여러 개의 데이터들을 하나의 type으로 정의
- impl : type에 기능 추가
  - &self : immutable borrow (읽기 전용)
  - &mut self : mutable borrow (수정 가능)
  - self : ownership 이동
  - new : associated function (생성자처럼 사용)

*adt01_structs.rs*

```rust
// struct(구조체) : 데이터를 구조화 (데이터들을 하나의 type으로)
// 1. 기본 -> field : type
struct User {
    pub name: String,
    pub email: String,
    pub id: u64
}

// 2. tuple -> type만 작성 (좌표, 색상 등)
// #[derive(Debug)] : debug trait 구현 속성
#[derive(Debug)]
struct Point(pub i32, pub i32);

// 3. impl : method 정의
struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    // &self : immutable borrow (읽기 전용)
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // new : (associated function)
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub fn struct_test() {

    let user = User{
        name: String::from("동헌"),
        email: String::from("dongheon@dongheon.com"),
        id: 1
    };

    println!("({}) {} : {}", user.id, user.name, user.email);

    let point = Point(1, 2);
    println!("({}, {})", point.0, point.1);
    // #[derive(Debug)] 추가 후 출력
    println!("{:?}", point);

    let rect = Rectangle::new(3, 3);
    println!("{}", rect.area());
}
```

<br>



### 3. enum

- 여러 variant 를 하나의 type으로 정의
- exhaustive : match 사용 시 반드시 모든 variant 에 대한 처리 필요 ( _ 사용해서 처리도 가능)

*adt02_enum.rs*

```rust
// enum : variant (여러 개의 상황) 를 하나의 type으로
enum Color {
    RED,
    BLUE,
    GREEN
}

// data가 포함된 enum
enum Response {
    Success(i32),
    Error{code:i32, message: String}
}

// impl : enum에 method 추가
impl Response {
    fn print(&self) {
        match self {
            Response::Success(x) => println!("Success ({x})"),
            // destructuring
            Response::Error{code, message} => {
                println!("Error ({code}) : {message}");
            }
        }
    }
}

pub fn enum_test() {
    let color = Color::RED;

    // exhaustive : 반드시 모든 variant를 구현해 줘야 함
    match color {
        Color::RED => println!("red"),
        Color::BLUE => println!("blue"),
        Color::GREEN => println!("green"),
    }

    // impl 을 붙여서 객체처럼 사용
    let resp01 = Response::Success(200);
    resp01.print();

    let resp02 = Response::Error{code:404, message:String::from("not found")};
    resp02.print();
}

```

<br>



### 4. option

- 값이 있다 / 없다 를 표현하는 **enum**
- Some
  - unwrap_or : 값이 없으면 기본값 사용
  - map : 값이 있을 때만 연산 수행
  - and_then : map + flatten. 결과가 option인 함수를 연결 (chaining)

*adt03_option.rs*

```rust
pub fn option_test() {
    // Option: None 을 사용하는 방법
    let dongheon: Option<String> = Some(String::from("dongheon"));
    let none: Option<String> = None;

    println!("{:?}", dongheon);
    println!("{:?}", none);

    // 1. unwrap_or: 값이 없으면 기본값 사용
    let unwrap01 = dongheon.clone().unwrap_or(String::from("Guest"));
    let unwrap02 = none.unwrap_or(String::from("Guest"));
    println!("{:?}", unwrap01);
    println!("{:?}", unwrap02);

    // 2. map: 값이 있을 때만 연산 수행 (데이터 변환)
    let length = dongheon.map(|s| s.len());
    println!("{:?}", length);

    // 3. and_then: map + flatten. 옵션을 반환하는 함수를 연속해서 부를 때 유용 (chaining)
    // result : Some(Some())
    let result = Some("dongheon").map(|s| s.chars().next());
    println!("{:?}", result);
    // andhthen : Some()
    let andthen01 = Some("dongheon").and_then(|s| s.chars().next());
    println!("{:?}", andthen01);
    let andthen02 = Some("dongheon")
                        .and_then(|s| s.chars().next())
                        .and_then(|c| c.to_uppercase().next());
    println!("{:?}", andthen02);
}

```



### 5. result

- 성공 (Ok) / 실패 (Err) 를 표현하는 **enum**

- ?  operator : 실패면 바로 return (early return) 

  -> error propagation  

  ->  pattern matching + return + type casting

*adt04_result.rs*

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    // Result : 성공 (Ok) / 실패 (Err) 처리 : 예외처리 대신
    if b == 0 {
        return Err(String::from("0으로 나눌 수 없습니다"));
    }
    Ok(a / b)
}

fn err_propagation() -> Result<i32, String> {
    // ? : early return (error propagation)
    let a = divide(10, 2)?;
    let b = divide(a, 1)?;
    Ok(b)
}

pub fn result_test() {
    let result01 = divide(3, 2);
    println!("{:?}", result01);
    let result02 = divide(3, 0);
    println!("{:?}", result02);

    match divide(3, 0) {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}")
    }

    println!("{:?}", err_propagation());
}

```



---



*main.rs*

```rust
mod adt01_structs;
mod adt02_enum;
mod adt03_option;
mod adt04_result;

fn main() {
    adt01_structs::struct_test();
    adt02_enum::enum_test();
    adt03_option::option_test();
    adt04_result::result_test();
}

```

