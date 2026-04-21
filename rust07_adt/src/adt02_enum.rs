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
