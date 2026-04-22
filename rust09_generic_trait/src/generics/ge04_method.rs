#[derive(Debug)]
struct Shape<T, U> {
    name: T,
    points: U,
}

// generic struct 에 method 추가
impl <T, U> Shape<T, U> {
    fn new(name: T, points: U) -> Shape<T, U> {
        Self { name, points }
    }

    fn get_points(&self) -> &U {
        &self.points
    }
}

// 특정 type에서만 동작
impl<T> Shape <T, Vec<(i32, i32)>> {
    fn count_points(&self) -> usize {
        self.points.len()
    }
}

pub fn generic_method() {
    let point = Shape::new("point", (0, 0));
    println!("{:?}", point);
    println!("{:#?}", point.get_points());

    let poly = Shape::<String, Vec<(i32, i32)>> {
        name: String::from("polygon"),
        points: vec![(0, 0), (1, 1), (2, 0), (1, -1)],
    };
    println!("{:?}", poly);
    println!("{:#?}", poly.name);
    println!("{:#?}", poly.get_points());
    println!("{:#?}", poly.count_points());
}
