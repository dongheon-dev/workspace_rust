#[derive(Debug)]
struct Shape<T, U> {
    name: T,
    points: U,
}

// 공통 기능 정의 (trait)
trait Draw {
    fn draw(&self);
}

// point 에 대한 Draw 구현
impl<T: std::fmt::Display> Draw for Shape<T, (i32, i32)> {
    fn draw(&self) {
        println!("[point] {}: 위치 {:?}", self.name, self.points);
    }
}

// polygon 에 대한 Draw 구현
impl<T: std::fmt::Display> Draw for Shape<T, Vec<(i32, i32)>> {
    fn draw(&self) {
        println!(
            "[polygon] {}: 점 개수 {}개, 데이터 {:?}",
            self.name,
            self.points.len(),
            self.points
        );
    }
}

// 동적 다형성 구현
pub fn trait_object() {
    let point = Shape { name: "point", points: (0, 0) };
    let square = Shape {
        name: "square",
        points: vec![(0,0), (1,0), (1,1), (0,1)]
    };

    // 서로 다른 타입을 하나의 벡터에
    // Box : smart pointer (나중에 나옴)
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(point),
        Box::new(square),
        Box::new(Shape { name: "triangle", points: vec![(0,0), (3,3), (6,0)] }),
    ];

    // 런타임에 각각 알맞은 draw()가 호출됨
    for item in shapes {
        item.draw();
    }
}
