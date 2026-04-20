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
