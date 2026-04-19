// 함수가 종료된 후 x 의 값을 참조하려고 하면
// x는 이미 drop되었기 때문에 참조할 곳을 잃어버림 (dangling reference)
/*
pub fn function_scope() -> &String{
    let x =String::from("hello");
    &x
}
*/

// 'a (lifetime annotation) : 참조된 값의 scope를 명시
// 지금 예제에선 'a 생략 가능
//pub fn function_scope<'a>(x: &'a String) -> &'a String {
pub fn function_scope(x: &String) -> &String {

        println!("function : {}", x);
    x
}

pub fn scope_test() {
    let s = String::from("scope");
    function_scope(&s);
}
