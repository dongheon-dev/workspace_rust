# Rust01_Cargo



### 1. cargo new

```bash
# 새로운 프로젝트 생성
cargo new rust01_cargo
cd rust01_cargo

# 코드 실행
cargo run
# src/main.rs 실행됨 -> Hello, world!
```



### 2. cargo check

```rust
// src/main.rs에서
fn main() {
    let x = 5					// ; 필요한데 작성하지 않았음
    println!("x = {}", x)
}

```

```bash
# error 확인
cargo check
```

*terminal 결과*

```bash
    Checking rust01_cargo v0.1.0 (C:\workspaces\workspace_rust\rust01_cargo)
error: expected `;`, found `println`                                                                                                                                               
 --> src\main.rs:2:14
  |
2 |     let x = 5
  |              ^ help: add `;` here
3 |     println!("x = {}", x)
  |     ------- unexpected token

error: could not compile `rust01_cargo` (bin "rust01_cargo") due to 1 previous error                                                                                               
```



### 3. cargo test

```rust
fn main() {
    let x = 5;
    println!("x = {}", x)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ok() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fail() {
        let result = 1 + 1;
        assert_eq!(result, 3);
    }
}
```

```bash
# test 실행
cargo test
```

*terminal 결과*

```bash
   Compiling rust01_cargo v0.1.0 (C:\workspaces\workspace_rust\rust01_cargo)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.35s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\rust01_cargo-6ebe44281ca2fc7f.exe)

running 2 tests
test tests::test_ok ... ok
test tests::test_fail ... FAILED

failures:

---- tests::test_fail stdout ----

thread 'tests::test_fail' (78208) panicked at src\main.rs:17:9:
assertion `left == right` failed
  left: 2
 right: 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin rust01_cargo`

```

