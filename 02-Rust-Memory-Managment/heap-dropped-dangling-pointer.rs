fn main() {
    let p;
    {
        let vec = vec![1, 2];
        p = &vec[0];
    }
    println!("{p}");
}