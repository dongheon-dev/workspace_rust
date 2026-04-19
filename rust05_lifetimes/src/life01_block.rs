pub fn block_scope() {
    let r;

    {
        let x = 1;
        // 허상 참조(dangling reference)
        // r은 function 내부 scope. x는 block 내부 scope
        // -> x가 drop 된 후 r의 값은 참조할 곳을 잃어버림
        // r = &x;

        // 이건 copy (정수니까 copy trait 구현되어있음)
        r = x;
        println!("block : {}", r);
        println!("block : {}", x);
    }

    println!("function : {}", r);
    // println!("function : {}", x);
}

