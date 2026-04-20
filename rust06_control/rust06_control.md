# Rust06_Control



### 1. 조건문

*control01_if.rs*

```rust
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
    let size = if i > 5 { "big" } else { "smal
        l" };
    println!("{size}");
}

```

<br>



### 2. 반복문

#### 2-1. loop

*control02_loop.rs*

```rust
pub fn loop_expression() {
    let mut i = 0;

    loop {
        i += 1;

        if i == 2 {
            continue;
        }

        if i == 5 {
            println!("break");
            break;
        }

        println!("{i}");
    }

    // loop도 expression (값 반환)
    let mut j = 0;

    let result = loop {
        j += 1;

        if j == 10 {
            break j * 2;
        }
    };

    println!("{result}");

    // label (중첩 루프 탈출)
    let mut outer = 0;

    'outer_loop: loop {
        let mut inner = 0;

        loop {
            if inner == 2 {
                // 가장 가까운 loop break
                break;
            }

            if outer == 2 {
                // outer_loop break
                break 'outer_loop;
            }

            inner += 1;
            println!("outer : {outer} / inner : {inner}");
        }

        outer += 1;
    }
}

```

<br>



#### 2-2. while

*control03_while.rs*

```rust
pub fn while_statement() {
    let mut i = 5;

    while i > 0 {
        println!("{i}");
        i -= 1;
    }

    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < arr.len() {
        let value = arr[index];
        println!("arr[{index}] : {value}");
        index += 1;
    }
}

```

<br>



#### 2-3.for

*control04_for.rs*

```rust
pub fn for_statements() {
    let arr = ["hello", "world", "rust", "is", "easy"];

    for value in arr {
        println!("{value}");
    }

    for i in 1..5 {
        println!("{i}");
    }

    for i in 1..=5 {
        println!("{i}");
    }

    for i in (1..=5).rev() {
        println!("{i}");
    }

    // index 필요할 때
    // .iter : iterator
    // .enumerate : 각 element 에 index 추가
    for (i, value) in arr.iter().enumerate() {
        println!("index={}, value={}", i, value);
    }
}

```

<br>



### 3. match

- match guard : 추가하는 조건들의 순서 중요! 

  -> 위에서부터 평가, 먼저 매칭되면 종료

- @ binding : 패턴 매칭하면서 동시에 값을 변수에 바인딩 (조건 검사 + 값 저장)

  -> 패턴과 변수 바인딩을 동시에 처리

*control05_match.rs*

```rust
pub fn match_expression() {
    let i = 3;

    match i {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    let result = match i {
        1 => "1",
        2 => "2",
        3 => "3",
        _ => "...",
    };
    println!("i : {result}");

    let month = 5;
    let season = match month {
        12 | 1 | 2 => "겨울",
        3..=5 => "봄",
        6..=8 => "여름",
        9..=11 => "가을",
        _ => "잘못된 월",
    };

    println!("{month}월은 {season}입니다.");

    // tuple match
    let tp = (0, 1);
    match tp {
        (0, y) => println!("x : 0 \t y = {y}"),
        (x, 0) => println!("y : 0 \t x = {x}"),
        _ => println!("no match"),
    }
}

pub fn match_guard() {
    // match guard : match 내부에서 if 사용 가능
    let num = 3;
    match num {
        x if x % 2 == 0 => println!("짝수"),
        _ => println!("홀수"),
    }

    // 조건의 순서 중요!! (첫 번째 조건에서 match 되어 다음 조건 확인 안함)
    match num {
        x if x > 0 => println!("양수"),
        x if x > 10 => println!("10보다 큼"),
        _ => println!("기타"),
    }
}

pub fn at_binding() {
    // @ binding : 패턴 매칭하면서 동시에 값을 변수로 저장
    let i = 7;

    match i {
        x @ 1..=5 => println!("x : {x} (1 ~ 5)"),
        x @ 6..=10 => println!("x : {x} (6 ~ 10)"),
        _ => println!("x > 10"),
    }

    // tuple에서도 가능
    let tp = (2, 5);

    match tp {
        p @ (0, y) => println!("x = 0인 tuple ({:?})", p),
        p @ (x, 0) => println!("y = 0인 tuple ({:?})", p),
        p @ (x, y) => println!("(0,0) 이 아닌 tuple ({:?})", p),
    }

    // match guard + @ binding 같이 사용
    match i {
        x @ 1..=10 if x % 2 == 0 => println!("x : {x} (짝수)"),
        x @ 1..=10 => println!("x : {x} (홀수)"),
        _ => println!("x > 10"),
    }
}

```

<br>



### 4. let expression

- if let : match에서 특정 pattern 하나만 검사하고 싶을 때 (syntax sugar)

- while let : match + loop (pattern이 계속 match되는 동안 반복하고 싶을 때)

- let else : pattern match 실패 시 early exit, 성공 시 변수 바인딩 후 계속 실행

  -> **control flow를 끊는 코드(diverging 등)** 반드시 필요

*control06_let_expression.rs*

```rust
pub fn if_let() {
    /*
    let tp = (0, -2);
    match tp {
        (0, y) => println!("x : 0 \t y = {y}"),
        (x, 0) => println!("y : 0 \t x = {x}"),
        _ => println!("no match"),
    }
    */

    let tp = (0, 1);
    // 특정 pattern 하나 (지금은 x=0 인 pattern) 만 맞는지 확인하고 싶을 때. (syntax sugar)
    // match는 모든 패턴을 확인해야 함
    if let (0, y) = tp {
        println!("x : 0 \t y = {y}");
    } else {
        println!("no match");
    }
}

pub fn while_let() {
    let arr = [1, 2, 3, 0, 4, 5];
    let mut index = 0;

    // 배열에서 꺼낸 값이 0이 아닌 동안만 반복
    // pattern : arr[index]가 0이 아닌 어떤 숫자일 때 반복
    while let val = arr[index] {
        // irrefutable pattern : 이럴꺼면 loop 써라. (무조건 성공하는 pattern이니까)
        // 지금은 아직 option 안배워서 느낌 안남 (나중에 배우면 느낌 옴)
        if val == 0 {
            break;
        }
        println!("{val}");
        index += 1;
    }
}

pub fn let_else() {
    // let tp = (0, 1);
    let tp = (1, 1);
    let (0, y) = tp else {
        println!("x가 0이 아님!");
        // diverging (발산) 코드 필요 : loop or function을 탈출하는 코드
        return;
    };

    println!("(0, {y})");
}

```



----



*main.rs*

```rust
mod control01_if;
mod control02_loop;
mod control03_while;
mod control04_for;
mod control05_match;
mod control06_let_expression;

use control05_match::*;
use control06_let_expression::{if_let, while_let, let_else};

fn main() {
    control01_if::if_expression();
    control02_loop::loop_expression();
    control03_while::while_statement();
    control04_for::for_statements();

    match_expression();
    match_guard();
    at_binding();

    if_let();
    while_let();
    let_else();
}

```

