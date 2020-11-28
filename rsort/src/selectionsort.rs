use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let mut smallest = unsorted;

            for i in unsorted + 1..slice.len() {
                if slice[i] < slice[smallest] {
                    smallest = i;
                }
            }

            if unsorted != smallest {
                slice.swap(unsorted, smallest);
            }
        }
    }
}

#[test]

fn insertionsort_test() {
    let mut unsorted = vec![3, 1, 2];
    let reference = vec![1, 2, 3];
    SelectionSort.sort(&mut unsorted);
    assert_eq!(unsorted, reference);
}
