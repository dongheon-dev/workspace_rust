#[derive(Debug)]
struct Shape<T, U> {
    name: T,
    points: U,
}

pub fn generic_struct() {
    let point   = Shape { name: "point", points: (0, 0) };
    println!("{:?}", point);

    let line = Shape { name: String::from("line"), points: [(1, 2), (3, 4)] };
    println!("{:?}", line);

    let poly = Shape::<String, Vec<(i32, i32)>> {
        name: String::from("polygon"),
        points: vec![(0, 0), (1, 1), (2, 0)],
    };
    println!("{:?} : {:?}", poly.name, poly.points);
}
