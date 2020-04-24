fn main() {
    let op = lib::GlobalLpPool::default();
    let result = op.eval_t(ndarray::arr2(&[[1.0f64, 2.0]]).view().into_dyn());
    println!("{:?}", result);
}
