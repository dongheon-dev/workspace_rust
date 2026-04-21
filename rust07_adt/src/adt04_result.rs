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
