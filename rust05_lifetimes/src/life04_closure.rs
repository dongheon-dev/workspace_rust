pub fn closure_borrow() {
    let x = String::from("hello");

    // | param | { body } (=lambda)
    // closure 가 x를 capture (immutable borrow)
    let print_x = || println!("{}", x);

    print_x();
    // closure (print_x) 가 x를 borrow 한 것이기 때문에 x가 살아있는 동안 closure가 유효
    println!("{}", x);
}

pub fn closure_move() {
    let x = String::from("world");

    // move keyword를 통해 x의 ownership이 closure 내부로 이동
    let print_x = move || {
        println!("{}", x);
    };

    print_x();

    // closure 가 x를 소유하고 있으므로 closure 외부에서 사용 불가
    // println!("{}", x);
}
