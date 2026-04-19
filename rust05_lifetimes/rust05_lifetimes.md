# Rust05_Lifetimes



*main.rs*

```rust
mod life01_block;
mod life02_function;
mod life03_borrow;
mod life04_closure;
mod life05_static;

use life02_function::scope_test;
use life03_borrow::borrow_scope;
use  life04_closure::*;
use  life05_static::*;

fn main() {
    life01_block::block_scope();

    scope_test();

    borrow_scope();

    closure_borrow();
    closure_move();

    static_scope();

    let s = "hello rust";
    println!("value : {}", s);
    println!("addr : {:p}", s);
}

```

<br>

### 1. dangling reference

- 허상 참조. 참조 변수가 가리키는 값이 먼저 drop되어 유효하지 않은 주소를 가리키게 되는 것
- rust는 compiler 가 막아줌 (build 조차 안됨)

*life01_block.rs*

```rust
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

```

<br>

### 2. 'a (lifetime annotation)

- 참조된 값의 scope를 명시
- dangling reference 방지
- 대부분의 경우 Rust가 자동으로 추론 (**lifetime elision**)

*life02_function.rs*

```rust
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

```

<br>

### 3. mutable borrow & NLL

- mutable borrow 중에는 원본 데이터 사용 불가
- **NLL** (Non-Lexical Lifetime) : borrow 가 실제 사용되는 범위까지만 유효

*life03_borrow.rs*

```rust
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

```

<br>

### 4. move

- closure 에서 변수를 참조하면 immutable borrow
- move keyword를 사용하면 변수의 ownership이 closure 안으로 이동됨

*life04_closure.rs*

```rust
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

```

<br>

### 5. static

- 프로그램의 실행부터 종료까지 메모리에 고정
- `&'static` 처럼 바이너리에 저장된 리터럴, 혹은 static으로 선언된 전역변수

*life05_static.rs*

```rust
// static variable (전역변수)
pub static GLOBAL: &str = "global";

pub fn static_scope() {
    // 타입 명시를 생략해도 컴파일러는 &'static str로 인식
    // hello rust 라는 값이 binary에 남아서, 모든 hello rust 리터럴 참조는 하나만 가리킴
    let s: &'static str = "hello rust";

    println!("value : {}", s);
    println!("addr : {:p}", s);
}

```

