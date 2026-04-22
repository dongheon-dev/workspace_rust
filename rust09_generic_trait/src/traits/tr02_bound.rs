#[derive(Debug)]
struct Dog { pub name: String }
#[derive(Debug)]
struct Cat { pub age: i32 }

trait Bark {
    fn bark(&self) -> String;
}

impl Bark for Dog {
    fn bark(&self) -> String { format!("강아지 {} 이(가) 멍멍!", self.name) }
}

impl Bark for Cat {
    fn bark(&self) -> String { format!("{}살 고양이가 야옹!", self.age) }
}

// inline bound: <T: Bark>
// T라는 타입은 무엇이든 될 수 있지만, 반드시 Bark 트레이트를 구현해야 함!
fn inline_bark<T: Bark>(animal: T) {
    println!("[inline] {}", animal.bark());
}

// where clause bound: 함수에 trait bound가 길어질 때 조건을 아래로 빼서 정리 (syntax sugar)
fn where_bark<T>(animal: T)
where
    // 복합 제약 (Bark도 해야하고 출력도 가능해야 함)
    T: Bark + std::fmt::Debug,
{
    println!("[where] {}", animal.bark());
}

pub fn trait_bound() {
    let dog = Dog { name: "스누피".into() };
    let cat = Cat { age: 3 };

    inline_bark(dog);
    where_bark(cat);

    // i32는 Bark trait을 구현하지 않았기 때문에 사용 불가
    // shout_inline(10);
}
