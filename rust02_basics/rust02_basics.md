# Rust02_Basics



[공식문서](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

지금은 간단하게 넘어가지만, 공식문서 확인 필!



### 1. variable

**let**  : immutable

**let mut** : mutable

**shadowing** : 같은 이름의 새로운 변수 선언



### 2. type

- **Scalar** : 값 하나를 담을 수 있는 변수
  - 정수 : i32
  - 실수 : f64
  - 논리 : bool
  - 문자 : char
- **Compound** : 여러 개의 값을 담을 수 있는 변수
  - tuple : 서로 다른 타입의 값을 담을 때
  - array : 같은 타입의 값을 담을 때



### 3. function

```rust
fn function (){}

fn function (a:i32){}

fn function (a:i32) -> i32{}
```

