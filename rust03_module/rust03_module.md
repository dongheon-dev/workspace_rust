# Rust03_Module



**mod** : module

**pub** : public (rust의 모든 요소는 기본적으로 private)

**use** : 경로를 scope에 가져와 단축



*main.rs*

```rust
// 1. mod : module 정의 (or 파일 선언)
pub mod hello {

    // 2. pub : public (rust에선 기본적으로 private)
    pub fn greetings() {
        println!("hello, rust!");
    }

    fn world() {
        println!("Hello, world!");
    }
}

// 3. use : 경로 단축
use hello::greetings;

fn main() {
    // 짧게 사용 가능
    greetings();

    // pub이 아니기 때문에 사용 불가
    // hello::world();
}
```



### file 분리

*hello01.rs*

```rust
pub fn greetings() {
    println!("hello02, rust!");
    world();
}

fn world() {
    println!("Hello, world!");
}

```

*main.rs*

```rust
/*
mod hello01;

// 1. mod : module 정의 (or 파일 선언)
pub mod hello {

    // 2. pub : public (rust에선 기본적으로 private)
    pub fn greetings() {
        println!("hello, rust!");
    }

    fn world() {
        println!("Hello, world!");
    }
}

// 3. use : 경로 단축
use hello::greetings;

fn main() {
    // 짧게 사용 가능
    greetings();

    // pub이 아니기 때문에 사용 불가
    // hello::world();
}
*/

// hello01 module
mod hello01;
// 경로 단축
use hello01::greetings;

fn main() {
    greetings()
}
```



### directory 구분 (mod.rs)

*hello02/communication.rs*

```rust
pub fn greetings() {
    println!("Hello, rust!");
}

```

*hello02/mod.rs*

```rust
pub mod communication;
```

*main.rs*

```rust
mod hello02;

use hello02::communication::greetings;

fn main() {
    greetings()
}
```



### mod.rs 없이 사용

*hello03/communication.rs*

```rust
pub fn greetings() {
    println!("Hello, rust!");
}

```

*hello03.rs*

```rust
pub mod communication;
```

*main.rs*

```rust
mod hello03;

use hello03::communication::greetings;

fn main() {
    greetings();
}

```



*다른 디렉토리에 mod.rs 라는 이름의 같은 파일이 많아져서 mod.rs 없이 바로 사용할 수 있도록 디렉토리이름.rs 를 사용하는 방식으로 발전됨*



