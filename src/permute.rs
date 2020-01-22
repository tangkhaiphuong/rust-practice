use std::collections::LinkedList;

pub fn permute<T>(array: &mut [T]) -> impl Iterator<Item = Vec<T>>
where
    T: Clone,
{
    fn permute_from_end<T, F>(
        array: &mut [T],
        begin_index: usize,
        end_index: usize,
        callback: &mut F,
    ) where
        T: Clone,
        F: FnMut(&[T]),
    {
        if begin_index >= end_index {
            callback(array);
        }

        let mut i = begin_index;
        while i < end_index {
            array.swap(begin_index, i);

            permute_from_end(array, begin_index + 1, end_index, callback);

            array.swap(begin_index, i);

            i += 1;
        }
    }

    let mut results = LinkedList::<Vec<T>>::new();

    permute_from_end(array, 0, array.len(), &mut |array2| {
        let item = array2.to_vec();
        results.push_back(item);
    });

    let x = std::iter::from_fn(move || results.pop_front());

    return x;
}
