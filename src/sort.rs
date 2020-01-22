use std::ops::{Index, IndexMut};

pub fn sort<T:, F: Fn(&T, &T) -> bool>(array: &mut [T], compare: F) where T: Eq + PartialEq + Ord  {
    let mut i = 0;

    while i < array.len() {
        let mut j = i + 1;

        while j < array.len() {
            if compare(&array[i], &array[j]) {
                array.swap(i, j);
            }

            j += 1;
        }
        i += 1;
    }
}

#[test]
fn test_sort() {
    let mut data = [3, 4, 6, 2, 4, 5, 6, 8];
    sort(&mut data, |left, right| left > right);

    for item in &data {
        print!("{0} ", item);
    }

    println!();
}