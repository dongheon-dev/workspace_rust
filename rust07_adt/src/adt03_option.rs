pub fn option_test() {
    // Option: None 을 사용하는 방법
    let dongheon: Option<String> = Some(String::from("dongheon"));
    let none: Option<String> = None;

    println!("{:?}", dongheon);
    println!("{:?}", none);

    // 1. unwrap_or: 값이 없으면 기본값 사용
    let unwrap01 = dongheon.clone().unwrap_or(String::from("Guest"));
    let unwrap02 = none.unwrap_or(String::from("Guest"));
    println!("{:?}", unwrap01);
    println!("{:?}", unwrap02);

    // 2. map: 값이 있을 때만 연산 수행 (데이터 변환)
    let length = dongheon.map(|s| s.len());
    println!("{:?}", length);

    // 3. and_then: map + flatten. 옵션을 반환하는 함수를 연속해서 부를 때 유용 (chaining)
    // result : Some(Some())
    let result = Some("dongheon").map(|s| s.chars().next());
    println!("{:?}", result);
    // andhthen : Some()
    let andthen01 = Some("dongheon").and_then(|s| s.chars().next());
    println!("{:?}", andthen01);
    let andthen02 = Some("dongheon")
                        .and_then(|s| s.chars().next())
                        .and_then(|c| c.to_uppercase().next());
    println!("{:?}", andthen02);
}
