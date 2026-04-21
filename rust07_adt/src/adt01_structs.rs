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
    // &self : immutable borrow
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // new : associated function
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
