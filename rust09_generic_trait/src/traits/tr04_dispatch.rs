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
