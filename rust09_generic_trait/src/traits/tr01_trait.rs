#[derive(Debug)]
struct Dog {
    name: String,
}

#[derive(Debug)]
struct Cat {
    age: i32,
}

// trait : behavior 정의 및 구현
trait Bark {
    fn bark(&self) -> String;
}

// trait 구현
impl Bark for Dog {
    fn bark(&self) -> String {
        format!("강아지 {} 이(가) 멍멍!", self.name)
    }
}

impl Bark for Cat {
    fn bark(&self) -> String {
        format!("{} 살 고양이가 야옹~", self.age)
    }
}

pub fn trait_basic() {
    let dog = Dog { name: "스누피".into() };
    let cat = Cat { age: 3 };

    println!("{:?}", dog);
    println!("{:?}", cat);

    println!("{}", dog.bark());
    println!("{}", cat.bark());
}
