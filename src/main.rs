fn main() {
    let op = lib::GlobalLpPool::default();
    let result = op.eval(vec![1.0f64, 2.0]);
    println!("{:?}", result);
}
