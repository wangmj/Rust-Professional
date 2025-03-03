/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: PartialOrd + Clone,
{
    if array.len() > 2 {
        let mut x = 0;
        loop {
            let mut y = x + 1;
            loop {
                if let Some(std::cmp::Ordering::Greater) = array[x].partial_cmp(&array[y]) {
                    let tmp = array[y].clone();
                    array[y] = array[x].clone();
                    array[x] = tmp;
                }
                y += 1;
                if y >= array.len() {
                    break;
                }
            }
            x += 1;
            if x >= array.len() - 1 {
                break;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
