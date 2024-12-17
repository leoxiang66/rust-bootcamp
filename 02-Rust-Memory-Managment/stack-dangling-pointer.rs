fn main() {

}

fn stack_dangling_pointer() -> &u32 {
    let number = 10;
    return &number;
}