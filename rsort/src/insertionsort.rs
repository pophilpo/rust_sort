use super::Sorter;

pub struct InsertionSort {
    pub smart: bool,
}

//TODO: impl smarter sort with binary search

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let mut i = unsorted;

            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[test]

fn insertionsort_test() {
    let mut unsorted = vec![3, 1, 2];
    let reference = vec![1, 2, 3];
    InsertionSort { smart: false }.sort(&mut unsorted);
    assert_eq!(unsorted, reference);
}
