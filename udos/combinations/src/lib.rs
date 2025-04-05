#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }
    if arr.is_empty() || arr.len() < k {
        return vec![];
    }
    // split our arr to its first elem + rest
    let (first, rest) = arr.split_first().unwrap();
    // recursively call combinations and add first to the beginning of
    // all combinations we get with remain elems
    let mut with_first = combinations(rest, k - 1)
        .into_iter()
        .map(|mut v| {
            v.insert(0, *first);
            v
        })
        .collect::<Vec<_>>();

    let mut without_first = combinations(rest, k);

    with_first.append(&mut without_first);
    with_first
}
