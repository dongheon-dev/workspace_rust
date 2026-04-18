pub fn copy_str() {
    // "hello"는 binary의 read-only memory에 저장
    // s1 : &'static str (fat pointer: ptr + len)
    let s1 = "hello";

    // copy trait (나중에 배움)
    let s2 = s1;

    println!("{}", s1);
    println!("{}", s2);
}
