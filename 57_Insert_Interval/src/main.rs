use insert_interval::Solution;

fn main() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    println!("Result: {:?}", Solution::insert(intervals, new_interval));
}
