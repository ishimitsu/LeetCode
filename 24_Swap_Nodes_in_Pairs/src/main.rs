use swap_nodes_in_pairs::{vec_to_list, list_to_vec, Solution};

fn main() {
    let head = vec_to_list(vec![1, 2, 3, 4]);
    let result = Solution::swap_pairs(head);
    println!("Result: {:?}", list_to_vec(result));
}
