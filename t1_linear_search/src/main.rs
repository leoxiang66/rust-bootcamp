#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

fn linear_search_rec<T: PartialEq>(list: &List<T>, val: &T, index: usize) -> isize {
    match list {
        List::Nil => -1,
        List::Cons(x, xs) => {
            if x == val {
                index as isize
            } else {
                linear_search_rec(xs, val, index + 1)
            }
        }
    }
}

fn main() {
    use List::{Cons, Nil};

    let list = Cons(3, Box::new(Cons(5, Box::new(Cons(7, Box::new(Nil))))));

    let index = linear_search_rec(&list, &5, 0);
    println!("Index of 5: {}", index); // 输出：Index of 5: 1

    let not_found = linear_search_rec(&list, &9, 0);
    println!("Index of 9: {}", not_found); // 输出：Index of 9: -1
}
