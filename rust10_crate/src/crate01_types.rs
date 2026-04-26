use rust10_crate_lib::add;

pub fn binary_crate() {
    // binary crate : 실행 가능한 program. src/main.rs 파일 안의 fn main() 이 주 진입점
    println!("Binary crate");
}

pub fn library_crate() {
    // library crate : 다른 program 에서 사용할 수 있도록 하는 기능들의 집합. src/lib.rs
    let lib = add(1, 2);
    println!("{lib}");
}
