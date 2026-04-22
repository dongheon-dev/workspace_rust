# Rust09_Generic_Trait

### 1. Generic

- type 유연성을 위해 사용 
- 종류
  - function
  - struct
  - enum
  - method 추가



*generics/ge01_function.rs*

```rust
// <T> : T 라는 type 선언
fn make_tuple01<T>(x: T, y: T) -> (T, T) {
    (x, y)
}

// <T, U> : 서로 다른 type일 때
fn make_tuple02<T, U>(x: T, y: U) -> (T, U) {
    (x, y)
}

pub fn generic_function() {
    // type 명시 (강제)
    let t01 = make_tuple01::<i32>(10, 20);
    // type 생략 (컴파일러 추론)
    let t02 = make_tuple01("a", "b");
    println!("{:?}", t01);
    println!("{:?}", t02);

    let t03 = make_tuple02(1, "one");
    println!("{:?}", t03);
}

```



*generics/ge02_struct.rs*

```rust
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

```



*generics/ge03_enum.rs*

```rust
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn divide(f: f64, g: f64) -> MyResult<f64, String> {
    if g == 0.0 {
        MyResult::Err(String::from("0으로 나눌 수 없습니다."))
    } else {
        MyResult::Ok(f / g)
    }
}

pub fn generic_enum() {
    let dongheon: MyOption<String> = MyOption::Some(String::from("dongheon"));
    println!("{:?}", dongheon);
    let none: MyOption<String> = MyOption::None;
    println!("{:?}", none);


    let results = vec![
        divide(10.0, 2.0),
        divide(5.0, 0.0),
    ];

    for res in results {
        match res {
            MyResult::Ok(val) => println!("결과: {}", val),
            MyResult::Err(msg) => println!("에러 발생: {}", msg),
        }
    }
}

```



*generics/ge04_method.rs*

```rust
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

```





### 2. Trait

#### 2-1. trait

- behavior 정의 및 구현 (interface)



*traits/tr01_trait.rs*

```rust
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

```



#### 2-2. trait bound

- inline bound : <T: trait> T 라는 type은 반드시 trait 구현!

- where clause bound : 함수에 trait bound가 길어질 때 조건을 아래로 빼서 정리 (syntax sugar)

  -> 제약을 여러 개 붙일 때 '+' 기호 사용



*tr02_bound.rs*

```rust
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

```



#### 2-3. trait object (= dyn trait)

- 서로 다른 type을 하나의 type으로 묶을 때

- 동적 다형성

*traits/tr03_object.rs*

```rust
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

```



#### 2-4. dispatch

-  런타임 시 함수가 어떻게 호출되는지를 결정
- static dispatch : <T: trait>
  - 컴파일 시점에 함수가 각각 만들어짐 (Monomorphization)
  - 속도가 빠르지만, 코드 크기가 커질 수 있음
- dynamic dispatch (동적 디스패치) -> &dyn trait 또는 Box \<dyn trait>
  - 런타임에 vtable(가상 함수 테이블)을 보고 어떤 함수를 실행할지 결정
  - 반드시 포인터 타입(&, Box, Arc 등) 뒤에 와야 함 (사이즈를 알 수 없기 때문)

- vtable : trait 함수들의 함수 포인터 테이블 (type이 trait을 구현할 때, 함수 호출을 대신 해줄 주소 테이블)

```rust
let animal: &dyn bark = &dog;
/*
animal = {
    data pointer  -> Dog 데이터
    vtable pointer -> Dog용 Speak 함수 테이블
}

+----------------------+
| drop_in_place        |  // 소멸자
| size                 |  // 타입 크기
| align                |  // 정렬
| bark (fn pointer)    |  // trait method들
+----------------------+
*/
```

*traits/tr04_dispatch.rs*

```rust
trait Bark {
    fn bark(&self);
}

struct Dog;
struct Cat;

impl Bark for Dog {
    fn bark(&self) { println!("멍멍!"); }
}

impl Bark for Cat {
    fn bark(&self) { println!("야옹~"); }
}

// static dispatch (정적 디스패치) -> <T: Bark>
// - 컴파일 시점에 Dog용 함수와 Cat용 함수가 각각 만들어짐 (Monomorphization)
// - 속도가 빠르지만, 코드 크기가 커질 수 있음
fn static_bark<T: Bark>(animal: T) {
    animal.bark();
}

// dynamic dispatch (동적 디스패치) -> &dyn Bark 또는 Box<dyn Bark>
// - 런타임에 vtable(가상 함수 테이블)을 보고 어떤 함수를 실행할지 결정
// - 반드시 포인터 타입(&, Box, Arc 등) 뒤에 와야 함 (사이즈를 알 수 없기 때문)
fn dynamic_bark(animal: &dyn Bark) {
    animal.bark();
}

pub fn trait_dispatch() {
    let dog = Dog;
    let cat = Cat;

    // static
    // 컴파일러가 play_static::<Dog>(dog)를 생성함
    static_bark(dog);
    // 컴파일러가 play_static::<Cat>(cat)를 생성함
    static_bark(cat);

    // dynamic
    // 하나의 리스트(Vec)에 서로 다른 타입을 담고 싶을 때 강력함
    let zoo: Vec<Box<dyn Bark>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in zoo {
        // 런타임에 실제 타입이 Dog인지 Cat인지 확인(*)하고 메서드 호출(^)
        // 역참조(*) -> 참조(&)
        dynamic_bark(&*animal);
        animal.bark();
    }
}

```

