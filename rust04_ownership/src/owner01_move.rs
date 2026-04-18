pub fn move_str() {
    // String (heap)
    let s1 = String::from("hello");

    // 2. move
    let s2 = s1;

    // 3. Error 발생!
    // -> Dangling Pointer 방지
    // println!("{}", s1);

    println!("{}", s2);
}
