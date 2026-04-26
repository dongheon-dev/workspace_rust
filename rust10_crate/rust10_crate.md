# Rust10_crate

```bash
# --lib : library crate 생성
cargo new rust10_crate_lib --lib

# rust10_crate에서 rand dependency 추가
cd rust10_crate
cargo add rand
```

*cargo add 를 하면 dependencies 에 추가됨*

*Cargo.toml*

```toml
[package]
name = "rust10_crate"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10.1"

```



- **crate** : compile 최소 단위
  - binary crate : src/main.rs + fn main() -> 실행 가능한 program
  - library crate : src/lib.rs -> 다른 program 에서 사용할 수 있도록 하는 기능들의 집합
- **package** : crate 의 묶음 (Cargo.toml) = project 
- **library** : 재사용 가능한 crate 



*crate01_type.rs*

```rust
use rust10_crate_lib::add;

pub fn binary_crate() {
    // binary crate : 실행 가능한 program. src/main.rs 파일 안의 fn main() 이 주 진입점
    println!("Binary crate");
}

pub fn library_crate() {
    // library crate : 다른 program 에서 사용할 수 있도록 하는 기능들의 집합. src/lib.rs
    let lib = add(1, 2);
    println!("{lib}");
}

```

<br>

*crate02_dependency.rs*

```rust
use rand::{random, rng, RngExt};

pub fn rand_test(i: i32, j: i32) {
    let random_num:i32 = random();
    println!("{random_num}");

    let mut rng = rng();
    // i ~ j-1 까지
    let num01 = rng.random_range(i..j);
    // i ~ j 까지
    let num02 = rng.random_range(i..=j);
    println!("{num01}");
    println!("{num02}");
}
```

<br>

*main.rs*

```rust
mod crate01_types;
mod crate02_dependency;

fn main() {
    crate01_types::binary_crate();
    crate01_types::library_crate();

    crate02_dependency::rand_test(5, 10);
}

```

<br>

*rust10_crate_lib 는 예제코드 그대로 사용*

