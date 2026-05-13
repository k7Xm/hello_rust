fn apply_all(inputs: &[i32], ops: &[Box<dyn Fn(i32) -> i32>]) -> Vec<i32> {
    inputs
        .iter()
        .map(|&x| ops.iter().fold(x, |acc, op| op(acc)))
        .collect()
}
