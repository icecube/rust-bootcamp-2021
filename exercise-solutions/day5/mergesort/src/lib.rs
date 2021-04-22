use std::thread;

fn mergesort_helper(list: &mut [i32], depth: usize) {
    let len = list.len();
    if len < 2 {
        return; // list is sorted
    } else if len < 100 {
        // insertion sort for small lists
        let mut i = 1;
        while i < len {
            let mut j = i;
            while j > 0 && list[j-1] > list[j] {
                list.swap(j, j-1);
                j -= 1;
            }
            i += 1;
        }
    } else {
        // recursive merge sort
        let mid = len/2;
        let (left, right) = list.split_at_mut(mid);
        let mut left_vec = vec![0; mid];
        left_vec.copy_from_slice(left);
        let mut right_vec = vec![0; len-mid];
        right_vec.copy_from_slice(right);
        if depth < 4 {
            let left_handle = thread::spawn(move || {
                mergesort_helper(&mut left_vec, depth+1);
                left_vec
            });
            let right_handle = thread::spawn(move || {
                mergesort_helper(&mut right_vec, depth+1);
                right_vec
            });
            left_vec = left_handle.join().unwrap();
            right_vec = right_handle.join().unwrap();
        } else {
            mergesort_helper(&mut left_vec, depth+1);
            mergesort_helper(&mut right_vec, depth+2);
        }

        // now merge
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let left_len = left_vec.len();
        let right_len = right_vec.len();
        while i < left_len && j < right_len {
            if left_vec[i] < right_vec[j] {
                list[k] = left_vec[i];
                i += 1;
            } else {
                list[k] = right_vec[j];
                j += 1;
            }
            k += 1;
        }
        if i < left_len {
            list[len-left_len+i..].copy_from_slice(&left_vec[i..]);
        }
        else if j < right_len {
            list[len-right_len+j..].copy_from_slice(&right_vec[j..]);
        }
    }
}

pub fn mergesort(list: &mut [i32]) {
    mergesort_helper(list, 0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut unsorted = vec![3,5,7,2,1,2,3,5,6,7,3,9,6];
        let sorted =       vec![1,2,2,3,3,3,5,5,6,6,7,7,9];
        mergesort(&mut unsorted);
        assert_eq!(unsorted, sorted);
    }

    #[test]
    fn large() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let sorted: Vec<_> = (1..1000000).collect();
        let mut unsorted = sorted.clone();
        unsorted.shuffle(&mut rng);
        mergesort(&mut unsorted);
        assert_eq!(unsorted, sorted);
    }
}
