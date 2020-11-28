use super::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]

fn bubblesort_test() {
    let mut unsorted = vec![3, 1, 2];
    let reference = vec![1, 2, 3];
    super::sort::<_, Bubblesort>(&mut unsorted);
    assert_eq!(unsorted, reference);
}
