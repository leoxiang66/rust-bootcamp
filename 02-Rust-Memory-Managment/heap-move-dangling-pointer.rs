fn main() {
    let vec = vec![1, 2];
    let p = &vec[0];
    let other_vec = vec;
    other_vec.push(1);
    println!("{p}");
}