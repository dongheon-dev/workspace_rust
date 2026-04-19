# Rust



### 0. install

[windows]

```bash
# visual studio build tools 필요!
winget install -e --id Microsoft.VisualStudio.2022.BuildTools --override "--passive --wait --add Microsoft.VisualStudio.Workload.VCTools;includeRecommended"

winget install Rustlang.Rustup

# 새 창에서
cargo --version
rustc --version
# 2026.04.18 기준 1.95.0
```

<br>



**rust ide**

[rustrover](https://www.jetbrains.com/rust/) : jetbrains 에서 만든 rust용 ide. (하지만 대부분 vsc 사용하는 듯)

<br>



[공식문서](https://rust-lang.org/learn/)

*terminal 에서 `rustup doc` 쳐도 공식문서 html 실행됨*

<br>

### [1. Cargo: 빌드 시스템](https://github.com/dongheon-dev/workspace_rust/blob/master/rust01_cargo/rust01_cargo.md)


```bash
# 프로젝트 생성.
cargo new

# 컴파일 체크
cargo check 

# 빌드 및 실행
cargo build / run

# 유닛 및 통합 테스트 수행
cargo test

```

<br>

### [2. 기초 문법 (Basics)](https://github.com/dongheon-dev/workspace_rust/blob/master/rust02_basics/rust02_basics.md)

**variable** : let(불변), let mut(가변), shadowing

**type**:

- scalar: 정수, 실수, 논리, 문자

- compound: tuple, array

**function**

<br>

### [3. 모듈 (Modules)](https://github.com/dongheon-dev/workspace_rust/blob/master/rust03_module/rust03_module.md)

**mod** : module

**pub** : public (rust의 모든 요소는 기본적으로 private)

**use** : 경로를 scope에 가져와 단축 (aliasing)

<br>

### [**4. 소유권 (Ownership)**](https://github.com/dongheon-dev/workspace_rust/blob/master/rust04_ownership/rust04_ownership.md)

**move** : heap에 저장되는 타입은 대입 시 주인이 바뀜 -> 이전 변수 사용 불가

**copy** : stack에 저장되는 타입은 대입 시 값만 복사됨 -> 이전 변수 사용 가능

**clone** : 명시적 복제 (deep copy)

**borrowing** : 소유권을 넘기지 않고 데이터 참조 (reference) 

<br>

### [**5. 생명주기 (Lifetimes)**](https://github.com/dongheon-dev/workspace_rust/blob/master/rust05_lifetimes/rust05_lifetimes.md)

**dangling reference** : 허상 참조. 참조 변수가 가리키는 값이 먼저 drop되어 유효하지 않은 주소를 가리키게 되는 것

**'a** (lifetime annotation) : 참조된 값의 scope를 명시

**borrow** : ownership의 이전 없이 참조

**move** : ownership이 이전됨

**static** : binary에 저장. 프로그램 종료 시 까지 남아있음

<br>

### **6. 제어 흐름 (Control Flow)**

조건문: if 표현식
반복문: loop, while, for in
패턴 매칭:
match
if let
while let
match guard
destructuring
@ 바인딩

<br>

### **7. 사용자 정의 타입 (Custom Types)**

Structs: 데이터 구조화 + impl 메서드
Enums: 데이터를 담는 ADT
Option & Result:
null과 예외 처리 대체

<br>

### **8. 컬렉션과 반복자 (Collections & Iterators)**

Collections:
Vec<T>, String, HashMap<K, V>
Iterator:
map, filter, collect
into_iter / iter / iter_mut 차이
lazy evaluation

<br>

### **9. 제네릭과 트레이트 (Generics & Traits)**

Generics:
타입 매개변수
Traits:
인터페이스 정의
Trait Bounds
derive (Debug, Clone 등)
trait object (dyn Trait)
static vs dynamic dispatch

<br>

### **10. 에러 핸들링 (Error Handling)**

Unrecoverable:
panic! 매크로
Recoverable:
Result<T, E>
에러 전파:
? 연산자
추가:
Result combinator (map, and_then)
커스텀 에러 타입
(anyhow, thiserror 등 실무 크레이트)

<br>

### **11. 스마트 포인터 (Smart Pointers)**

Box<T> (힙 할당)
Rc<T> (참조 카운트)
Arc<T> (스레드 안전)
RefCell<T> (내부 가변성)

<br>

### **12. 동시성 (Concurrency)**

Thread
Mutex
Channel (메시지 패싱)
Send / Sync

<br>

### **13. Async**

Future
async / await
Tokio 런타임
Pin / Unpin
executor vs runtime

<br>

### **14. Macro**

macro_rules!
derive macro (개념)

<br>

### **15. Unsafe Rust**

unsafe 키워드
raw pointer
FFI (C 연동)

