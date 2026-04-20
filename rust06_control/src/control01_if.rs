pub fn if_expression() {
    let i = 9;

    // if
    if i % 2 == 0 {
        println!("{i} : 짝수");
    } else {
        println!("{i} : 홀수");
    }

    // if ~ else if
    let score = 85;

    let mut grade = "F";

    if score >= 90 {
        grade = "A";
    } else if score >= 80 {
        grade = "B";
    } else if score >= 70 {
        grade = "C";
    }

    println!("{grade}");

    // if 는 expression (표현식) 이라 값 반환 가능
    let condition = true;

    let result = if condition { 10 } else { 20 };
    println!("{result}");

    // 타입은 반드시 같아야 함
    let size = if i > 5 { "big" } else { "small" };
    println!("{size}");
}
