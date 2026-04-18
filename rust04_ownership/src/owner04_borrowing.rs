pub fn borrowing_str() {
    let mut s1 = String::from("hello");

    // immutable borrowing
    // &를 사용해 소유권을 넘기지 않고 읽기 권한만 빌림
    let len = calculate_length(&s1);
    println!("s1 : {} / len : {}", s1, len);

    // mutable borrowing
    // &mut를 사용해 소유권을 넘기지 않고 수정 권한까지 빌림
    // 해당 스코프에 가변 참조자는 딱 하나만 존재!!
    change(&mut s1);
    println!("s1 : {}", s1);
}

fn calculate_length(s: &String) -> usize {
    // 여기서 s는 스코프를 벗어나지만, 소유권이 없으므로 메모리가 해제되지 않음
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}