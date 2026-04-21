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
