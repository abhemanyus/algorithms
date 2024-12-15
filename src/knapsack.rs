pub fn knapsack(cap: i32, items: &[(i32, i32, i32)]) -> (f32, Vec<i32>) {
    let mut items = items.to_owned();
    items.sort_by_key(|(_, w, p)| w / p);
    let mut cap = cap;
    let mut stored = vec![];
    let mut profit = 0.0;
    for item in items {
        let frac = cap as f32 / item.1 as f32;
        if frac >= 1.0 {
            cap -= item.1;
            stored.push(item.0);
            profit += item.2 as f32;
        } else {
            stored.push(item.0);
            profit += frac * item.2 as f32;
            break;
        }
    }
    (profit, stored)
}

#[test]
fn test_knapsack() {
    let items = [(1, 1, 2), (2, 2, 4), (3, 2, 1)];
    let cap = 4;
    let res = knapsack(cap, &items);
    assert_eq!(res, (6.5, vec![1, 2, 3]));
}
