#![warn(clippy::all, clippy::pedantic)]

use::std::cmp::Ordering;

fn main() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let result = binary_search(&arr, 1);

    match result {
        Some((found_value, found_index)) => println!("Found value: {found_value}, found index: {found_index} "),
        None => println!("Not found")
    }
}

fn binary_search(arr: &[i32], desired_value: i32) -> Option<(i32, usize)> {
    let mut low_bound: usize = 0;
    let mut up_bound: usize = arr.len() - 1;
    let mut i: usize = 0;

    while low_bound <= up_bound {
        i += 1;
        
        let mid = usize::midpoint(low_bound, up_bound);

        let mid_value = arr[mid];

        match mid_value.cmp(&desired_value) {
            Ordering::Equal => return Some((mid_value, mid)),
            Ordering::Less => low_bound = mid + 1,
            Ordering::Greater => {
                up_bound = match mid.checked_sub(1) {
                    Some(result) => result,
                    _ => return None
                }
            }
        }

        println!("Step: {i}");
    }

    None
}