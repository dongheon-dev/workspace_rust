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
