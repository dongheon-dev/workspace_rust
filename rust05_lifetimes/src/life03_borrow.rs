pub fn borrow_scope() {
    let mut x = String::from("hello");

    // mutable borrow
    let r1 = &mut x;
    r1.push_str(" world");

    // borrow 중이라 x 사용 불가
    // println!("{}", x);

    // 이거 없으면 NLL (Non-Lexical Lifetimes)
    // r1의 lifetime을 마지막 사용 시점까지 자동으로 종료시켜, x를 사용 가능하게 만듦
    println!("{}", r1);
}
