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



[홈페이지](https://rust-lang.org/) -> learn 

-> read the book! : [개념](https://doc.rust-lang.org/book/)

-> the standard library : [api](https://doc.rust-lang.org/std/index.html)

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

- scalar : 정수, 실수, 논리, 문자

- compound : tuple, array

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

### [**6. 제어 흐름 (Control Flow)**](https://github.com/dongheon-dev/workspace_rust/blob/master/rust06_control/rust06_control.md)

**조건문**

- if

**반복문**

- loop
- while
- for

**match**

- match guard : match + 조건 (패턴 매칭 후 추가 조건 검사)
- @ binding : 패턴 매칭 + 값 바인딩 (전체 값을 변수로 저장하면서 매칭)

**let expression**

- if let : match 간결화 (syntax sugar) -> 특정 pattern 하나만 확인
- while let : 패턴이 계속 매칭되는 동안 반복 (match + loop 형태)
- let else : 패턴이 실패하면 즉시 탈출 (early exit), 성공하면 변수 바인딩

<br>

### [**7. ADT (Algebraic Data Type)**](https://github.com/dongheon-dev/workspace_rust/blob/master/rust07_adt/rust07_adt.md)

**Product Type (And)**

- Struct : 데이터 구조화

**Sum Type (OR)**

- Enums : 여러 variant를 하나의 type으로
- Option : 값의 유무
- Result : 성공 / 실패

<br>

### [**8. 컬렉션 (Collections & Iterators)**](https://github.com/dongheon-dev/workspace_rust/blob/master/rust08_collection/rust08_collection.md)

**Collections**

- vec : 가변 배열
- string : 문자열
- map : key-value 구조
- set : 중복 X

**Iterators**

- map (값 변환), filter (조건 필터링), fold (누적 연산)
- iter (borrow), into_iter (ownership move), iter_mut (modify), flat_map(flatten)
- lazy evaluation : collect 호출 시 실행

<br>

### [**9. 제네릭과 트레이트 (Generics & Traits)**](https://github.com/dongheon-dev/workspace_rust/blob/master/rust09_generic_trait/rust09_generic_trait.md)

**generic** : type 유연성

- function : 함수 뒤에 <T>
- struct : 구조체 뒤에 <T>
- method generic : impl 시 <T>

**trait** : behavior 정의 및 구현 (interface)

- trait bound : 특정 기능 (trait) 을 가진 type으로 generic 제한 -> where (syntax sugar)
- trait object (dyn trait) : 서로 다른 type을 하나의 type으로 묶을 때
- static / dynamic dispatch : 함수 호출 시 생성

<br>

### **10. crate**
<br>

### **11. io**

<br>

### 12. 에러 처리 (Error Handling)**

**panic!** (Unrecoverable)

**Result<T, E>** (Recoverable)

anyhow, thiserror

<br>

### **13. 스마트 포인터 (Smart Pointers)**

Box<T> (힙 할당)
Rc<T> (참조 카운트)
Arc<T> (스레드 안전)
RefCell<T> (내부 가변성)

<br>

### **14. 동시성 (Concurrency)**

Thread
Mutex
Channel (메시지 패싱)
Send / Sync

<br>

### 15. Async**

Future
async / await
Tokio 런타임
Pin / Unpin
executor vs runtime

<br>

### **16. Macro**

macro_rules!
derive macro (개념)

<br>

### **17. Unsafe Rust**

unsafe 키워드
raw pointer
FFI (C 연동)

