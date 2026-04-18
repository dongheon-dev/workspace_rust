# Rust



### 0. install

[windows]

```bash
# visual studio build tools 필요!
winget install -e --id Microsoft.VisualStudio.2022.BuildTools --override "--passive --wait --add Microsoft.VisualStudio.Workload.VCTools;includeRecommended"

# terminal
winget install Rustlang.Rustup

# 새 창에서
cargo --version
rustc --version
# 2026.04.18 기준 1.95.0
```



**rust ide**

[rustrover](https://www.jetbrains.com/rust/) : jetbrains 에서 만든 rust용 ide. (하지만 대부분 vsc 사용하는 듯)



[공식문서](https://rust-lang.org/learn/)

*terminal 에서 `rustup doc` 쳐도 공식문서 html 실행됨*

<br>

### 1. Cargo: 빌드 시스템


```bash
# 프로젝트 생성.
cargo new

# 컴파일 체크
cargo check 

# 빌드 및 실행
cargo build / run

# 유닛 및 통합 테스트 수행
cargo test

# dependency 관리
cargo.toml
```

<br>

### 2. 기초 문법 (Basics)

변수: let(불변), let mut(가변), Shadowing.

데이터 타입:

Scalar: i32, f64, bool, char.

Compound: tuple, array.

함수: fn 정의 및 반환 타입(->)

<br>

### 3. 소유권과 빌림 (Ownership & Borrowing)

소유권 규칙:

owner는 하나

scope 벗어나면 drop

move 발생

move vs copy

Clone

빌림(Borrowing):

&T(불변 참조)

&mut T(가변 참조)

dangling reference 방지

<br>

### 4. 제어 흐름 & 패턴 매칭 (Control Flow & Pattern Matching)

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

### 5. 표현식 vs 구문 (Expressions & Statements)

statements : 값 없음 (;)

expression : 값 반환

<br>

### 6. 사용자 정의 타입 (Custom Types)

Structs: 데이터 구조화 + impl 메서드

Enums: 데이터를 담는 ADT

Option & Result:

null과 예외 처리 대체

<br>

### 7. 에러 핸들링 (Error Handling)

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

### 8. 컬렉션과 반복자 (Collections & Iterators)

Collections:

Vec<T>, String, HashMap<K, V>

Iterator:

map, filter, collect

into_iter / iter / iter_mut 차이

lazy evaluation

<br>

### 9. 제네릭과 트레이트 (Generics & Traits)

Generics:

타입 매개변수

Traits:

인터페이스 정의

Trait Bounds

derive (Debug, Clone 등)

trait object (dyn Trait)

static vs dynamic dispatch

<br>

### 10. 모듈 & 패키지 (Modules & Packages)

모듈 시스템:

mod, use, pub

파일 기반 모듈 구조 (mod.rs 대체)

crate

외부 라이브러리 사용 (Cargo.toml)

<br>

### 11. 생명주기 (Lifetimes)

개념:

참조 유효 범위 명시

표기법:

<'a>

생략 규칙

<br>

### 12. 스마트 포인터 (Smart Pointers)

Box<T> (힙 할당)

Rc<T> (참조 카운트)

Arc<T> (스레드 안전)

RefCell<T> (내부 가변성)

<br>

### 13. 동시성 (Concurrency)

Thread

Mutex

Channel (메시지 패싱)

Send / Sync

<br>

### 14. Async

Future

async / await

Tokio 런타임

Pin / Unpin

executor vs runtime

<br>

### 15. Macro

macro_rules!

derive macro (개념)

<br>

### 16. Unsafe Rust

unsafe 키워드

raw pointer

FFI (C 연동)

