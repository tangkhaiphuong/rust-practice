pub fn combine<'a, T>(source: &'a [&[T]]) -> impl Iterator<Item = Vec<T>> + 'a
where
    T: Clone,
{
    let length = source.len();

    let mut indices = vec![0; length];

    let mut flag = true;

    std::iter::from_fn(move || {
        let mut result = Vec::with_capacity(length);

        let mut index = 0;

        for row in source {
            result.push(row[indices[index]].clone());
            index += 1;
        }

        if flag {
            flag = false;
            return Some(result);
        }

        // find the rightmost array that has more
        // elements left after the current element
        // in that array
        let mut next = (length - 1) as i32;

        while next >= 0 && (indices[next as usize] + 1 >= source[next as usize].len()) {
            next -= 1;
        }

        // no such array is found so no more
        // combinations left
        if next < 0 {
            return None;
        }

        // if found move to next element in that
        // array
        indices[next as usize] += 1;

        // for all arrays to the right of this
        // array current index again points to
        // first element
        let mut i = next + 1i32;
        while i < length as i32 {
            indices[i as usize] = 0;
            i += 1;
        }

        return Some(result);
    })
}
