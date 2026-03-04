use merge_intervals::Solution;

fn main() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    println!("Result: {:?}", Solution::merge(intervals));
}
