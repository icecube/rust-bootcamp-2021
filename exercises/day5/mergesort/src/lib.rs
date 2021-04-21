
pub fn mergesort(list: &mut [i32]) {
    let len = list.len();
    if len < 2 {
        return; // list is sorted
    } else if len < 10 {
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
        mergesort(left);
        mergesort(right);

        // now merge
        let mut i = 0;
        let mut j = mid;
        let mut sorted = Vec::with_capacity(len);
        while i < mid && j < len {
            if list[i] < list[j] {
                sorted.push(list[i]);
                i += 1;
            } else {
                sorted.push(list[j]);
                j += 1;
            }
        }
        if i < mid {
            sorted.extend_from_slice(&list[i..mid]);
        }
        else if j < len {
            sorted.extend_from_slice(&list[j..len]);
        }
        list.swap_with_slice(&mut sorted);
    }
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
