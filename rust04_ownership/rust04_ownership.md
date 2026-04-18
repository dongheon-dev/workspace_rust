# Rust04_Ownership

<br>

### 1. move

heep memory를 사용하는 type들 (String, ...) 에서 발생

- shallow copy 후, double free (메모리 중복 해제) 오류를 방지하기 위해 이전 변수 무효화

*owner01_move.rs*

```rust
pub fn move_str() {
    // String (heap)
    let s1 = String::from("hello");

    // 2. move
    let s2 = s1;

    // 3. Error 발생!
    // -> Dangling Pointer 방지
    // println!("{}", s1);

    println!("{}", s2);
}

```

<br>

### 2. copy

stack memory에 저장되는 type들 (i32, ...) 에서 발생

- copy trait 구현되어 있음 (trait -> interface. 기능 구현 공유)
- bitwise copy

*owner02_copy.rs*

```rust
pub fn copy_str() {
    // "hello"는 binary의 read-only memory에 저장
    // s1 : &'static str (fat pointer: ptr + len)
    let s1 = "hello";

    // copy trait (나중에 배움)
    let s2 = s1;

    println!("{}", s1);
    println!("{}", s2);
}

```

<br>

### 3. clone

- deep copy를 가능하게 만드는  trait

- 새로운 heap memory에 값 복제

*owner03_clone.rs*

```rust
pub fn clone_str() {
    let s1 = String::from("hello");

    // 1. clone : heap 영역의 s1을 deep copy
    let s2 = s1.clone();

    // 2. s1과 s2는 서로 다른 메모리를 참조
    println!("{}", s1);
    println!("{}", s2);
}
```

<br>

### 4. borrowing

**borrowing**:

- `&T` (Immutable) : 읽기 (borrower 여러 명 가능)
- `&mut T` (Mutable) : 쓰기 (borrower 한명)
- **Dangling Reference 방지**: 참조하는 데이터가 사라지기 전에 참조의 생명주기가 끝나도록 Rust가 컴파일 타임에 체크함.

*owner04_borrowing.rs*

```rust
pub fn borrowing_str() {
    let mut s1 = String::from("hello");

    // immutable borrowing
    // &를 사용해 소유권을 넘기지 않고 읽기 권한만 빌림
    let len = calculate_length(&s1);
    println!("s1 : {} / len : {}", s1, len);

    // mutable borrowing
    // &mut를 사용해 소유권을 넘기지 않고 수정 권한까지 빌림
    // 해당 스코프에 가변 참조자는 딱 하나만 존재!!
    change(&mut s1);
    println!("s1 : {}", s1);
}

fn calculate_length(s: &String) -> usize {
    // 여기서 s는 스코프를 벗어나지만, 소유권이 없으므로 메모리가 해제되지 않음
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```



---



*main.rs*

```rust
mod owner01_move;
mod owner02_copy;
mod owner03_clone;
mod owner04_borrowing;

use owner02_copy::copy_str;
use owner03_clone::clone_str;

fn main() {
    owner01_move::move_str();
    copy_str();
    clone_str();
    owner04_borrowing::borrowing_str();
}

```

