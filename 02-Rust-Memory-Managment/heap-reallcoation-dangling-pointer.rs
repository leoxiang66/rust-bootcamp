fn main() {
    let mut vec = vec![1, 2];
    let p = &vec[0];
    vec.push(3);
    vec.push(4);
    println!("{p}");
}