fn binary_search<T: Ord>(arr: &[T], val: &T) -> isize {
    let len = arr.len();
    if len == 0 {
        return -1;
    }
    if len == 1 {
        return if &arr[0] == val { 0 } else { -1 };
    }
    if len == 2 {
        if &arr[0] == val {
            return 0;
        } else if &arr[1] == val {
            return 1;
        } else {
            return -1;
        }
    }

    let mid = len / 2;
    if &arr[mid] == val {
        mid as isize
    } else if val < &arr[mid] {
        binary_search(&arr[..mid], val)
    } else {
        let result = binary_search(&arr[mid + 1..], val);
        if result == -1 {
            -1
        } else {
            (mid + 1) as isize + result
        }
    }
}


fn main() {
    let arr = vec![1, 3, 5, 7, 9, 11];
    println!("Index of 5: {}", binary_search(&arr, &5)); // 输出 2
    println!("Index of 6: {}", binary_search(&arr, &6)); // 输出 -1
}
