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
