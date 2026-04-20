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
    // match guard : match 내부에서 if 사용 가능 (pattern matching 후 조건을 추가)
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
