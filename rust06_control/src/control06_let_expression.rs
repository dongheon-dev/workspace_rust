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
