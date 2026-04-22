#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn divide(f: f64, g: f64) -> MyResult<f64, String> {
    if g == 0.0 {
        MyResult::Err(String::from("0으로 나눌 수 없습니다."))
    } else {
        MyResult::Ok(f / g)
    }
}

pub fn generic_enum() {
    let dongheon: MyOption<String> = MyOption::Some(String::from("dongheon"));
    println!("{:?}", dongheon);
    let none: MyOption<String> = MyOption::None;
    println!("{:?}", none);


    let results = vec![
        divide(10.0, 2.0),
        divide(5.0, 0.0),
    ];

    for res in results {
        match res {
            MyResult::Ok(val) => println!("결과: {}", val),
            MyResult::Err(msg) => println!("에러 발생: {}", msg),
        }
    }
}
