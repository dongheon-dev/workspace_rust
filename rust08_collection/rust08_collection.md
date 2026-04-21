# Rust08_Collection_Iterators



### 1. Collections

#### 1-1. vector

- 동적 배열
- 생성방법
  - Vec::new()
  - vec![]
- 값 조회
  - vec[index] : panic! - 위험
  - get : option return - 안전
- iteration
  - immutable
  - mutable

*collections/col01_vector.rs*

```rust
pub fn vec_test() {
    // vec : 동적 배열 -> heap에 저장

    // vec 생성
    let mut vec01 = Vec::new();
    vec01.push(1);
    vec01.push(2);
    vec01.push(3);

    println!("{:?}", vec01);

    // vec! (macro) 사용해서 생성
    let vec02 = vec![3, 4, 5, 6];
    println!("{:?}", vec02);

    // indexing (panic! : 위험)
    println!("{}", vec02[1]);

    // get (option return : 안전)
    match vec02.get(1) {
        Some(v) => println!("{v}"),
        None => println!("none"),
    }

    // immutable borrow iteration
    for i in &vec02 {
        println!("{i}");
    }

    // mutable borrow iteration
    let mut vec03 = vec![7, 8, 9];

    for i in &mut vec03 {
        // dereference 해서 값 수정
        *i *= 10;
    }

    println!("{:?}", vec03);
}

```

<br>



#### 1-2. string

- utf-8 기반 - 영어 1byte / 한글 3byte
- 생성 방법
  - String::new()
  - String::from()
- concat 시 ownership 이동 주의!!!
- &str (참조) 와 헷갈리지 않기!!!

*collections/col02_string.rs*

```rust
pub fn string_test() {
    // string : utf-8 기반 heap 문자열 ( vec<u8> 구조 )
    // new 사용하여 생성
    let mut str01 = String::new();
    str01.push_str("hello");
    str01.push('!');

    println!("{str01}");

    // from 사용하여 생성
    let str02 = String::from("world");

    // &str (string slice)로 borrow
    let str03: &str = &str02;

    println!("{str03}");

    // concat (ownership 이동 주의)
    let a = str01;
    let b = str02;
    // a는 move됨 (이후 사용 불가)
    let c = a + " " + &b;
    // println!("{a}");
    println!("{b}");
    println!("{c}");
}

```

<br>



#### 1-3. map

- `use std::collections` : standard library, collections module
- key - value 구조
- entry api
  - or_insert : 없으면 삽입
  - and_modify : 있으면 함수 실행
  - or_default : Default trait 구현되어 있으면 기본값으로 초기화
  - or_inster_with(closure) : 없을 때 closure를 사용하여 삽입
- hash map : 무작위 순서, key 가 hash를 구현
- btree map : key 순서대로 정렬, key가 ord(비교) 를 구현

*collections/col03_map.rs*

```rust
use std::collections::{HashMap, BTreeMap};
// standard library 의 collections module에서 hashmap, btreemap 사용

pub fn hashmap_test() {
    // hashmap<k, v> : key-value 구조 -> hashing 기반 저장
    let mut map = HashMap::new();
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));
    println!("{:?}", map);

    // &2 : key가 가리키고 있는 memory를 넘겨줘야 한다!
    match map.get(&2) {
        Some(v) => println!("{v}"),
        None => println!("None"),
    }

    // entry api
    // or_insert : 없으면 삽입
    map.entry(1).or_insert(String::from("하나"));
    map.entry(3).or_insert(String::from("three"));
    println!("{:?}", map);

    // and_modify : 있으면 함수 실행
    map.entry(3)
        // .and_modify(|v| {*v = v.to_uppercase()})
        .and_modify(|v| {v.make_ascii_uppercase()})
        .or_insert(String::from("셋"));

    // counting pattern
    let target = "a b a c b a";
    let mut counter = HashMap::new();

    for word in target.split_whitespace() {
        // 1. entry로 키를 찾고,
        // 2. 없으면 0을 삽입한 뒤,
        // 3. 그 값의 가변 참조자(&mut)를 가져와서 1을 더함
        let count = counter.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", counter);
}

pub fn btreemap_test() {
    // BTreeMap : HashMap + 정렬된 상태 유지
    let mut map = BTreeMap::new();
    map.insert(3, "c");
    map.insert(1, "a");
    map.insert(2, "b");

    // key 기준 자동 정렬
    println!("{:?}", map);

    for (k, v) in &map {
        println!("{k} : {v}");
    }
}

```

<br>



#### 1-4. set

- 중복 X
- 집합 연산

*collections/col04_set.rs*

```rust
use std::collections::HashSet;

pub fn hashset_test() {
    // hashset : 중복 X, 집합 연산
    let mut set = HashSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(2);
    set.insert(3);
    set.insert(3);

    println!("{:?}", set);

    if set.contains(&1) {
        println!("1 exists");
    }

    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [3, 4, 5].iter().cloned().collect();

    let union: HashSet<_> = a.union(&b).cloned().collect();

    println!("{:?}", union);
}

```

<br>



### 2. Iterators

#### 2-1. map, filter, fold

- map : 값 변환
- filter : 조건 필터링
- fold : 누적 연산
- collect : iterator 연산 실행

*iterators/iter01_map_filter_fold.rs*

```rust
pub fn map_filter_fold() {
    let vec01 = vec![1, 2, 3, 4, 5];

    // map: 값 변환
    let vec02: Vec<i32> = vec01.iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", vec02);

    // filter: 조건 필터링
    let vec03: Vec<i32> = vec01.iter()
        .filter(|x| *x % 2 == 0)
        .cloned()
        .collect();

    println!("{:?}", vec03);

    // fold : 누적 연산
    let sum = vec01.iter().fold(0, |acc, x| acc + x);
    println!("{sum}");
    let product = vec01.iter().fold(1, |acc, x| acc * x);
    println!("{product}");
}

```

<br>



#### 2-2. iterator types

- iter : borrow
- into_iter : ownership move
- iter_mut : modify
- flat_map : flatten

*iterators/iter02_types.rs*

```rust
pub fn iter_types_test() {
    // iterator ownership
    // 1. iter -> borrow
    let vec01 = vec![1, 2, 3];
    for i in vec01.iter() {
        println!("{i}");
    }

    // 2. into_iter -> ownership move
    let vec02 = vec![1, 2, 3];
    for i in vec02.into_iter() {
        println!("{i}");
    }

    // 3. iter_mut -> modify
    let mut vec03 = vec![1, 2, 3];
    for i in vec03.iter_mut() {
        *i *= 10;
    }
    println!("{:?}", vec03);
}

pub fn flatmap_test() {
    let v = vec![vec![1, 2], vec![3, 4], vec![5]];

    // flat_map : flatten
    let flat: Vec<i32> = v.into_iter()
        .flat_map(|x| x)
        .collect();

    println!("flat: {:?}", flat);
}

```

<br>



#### 2-3. lazy evaluation

- collect 호출시 iterator 실행

*iterators/iter03_lazy.rs*

```rust
pub fn lazy_test() {
    let vec = vec![1, 2, 3, 4];
    let iter = vec.iter().map(|x| {
        x * 2
    });
    // iterator 아직 실행안됨
    println!("{:?}", vec);

    // lazy evaluation (collect 시 실행)
    let result: Vec<i32> = iter.collect();
    println!("{:?}", result);
}

```

