pub fn string_test() {
    // string : utf-8 기반 heap 문자열 ( vec<u8> 구조 )
    // new 사용하여 생성
    let mut str01 = String::new();
    str01.push_str("hello");
    str01.push('!');

    println!("{str01}");

    // from 사용하여 생성
    let str02 = String::from("world");

    // &str (string slice)로 borrow
    let str03: &str = &str02;

    println!("{str03}");

    // concat (ownership 이동 주의)
    let a = str01;
    let b = str02;
    // a는 move됨 (이후 사용 불가)
    let c = a + " " + &b;
    // println!("{a}");
    println!("{b}");
    println!("{c}");
}
