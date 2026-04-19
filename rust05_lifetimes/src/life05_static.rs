// static variable (전역변수)
pub static GLOBAL: &str = "global";

pub fn static_scope() {
    // 타입 명시를 생략해도 컴파일러는 &'static str로 인식
    // hello rust 라는 값이 binary에 남아서, 모든 hello rust 리터럴 참조는 하나만 가리킴
    let s: &'static str = "hello rust";

    println!("value : {}", s);
    println!("addr : {:p}", s);
}
