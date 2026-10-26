/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]){
    for index in 1..array.len() {
        let mut position = index;
        while position > 0 && array[position] < array[position - 1] {
            array.swap(position, position - 1);
            position -= 1;
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
#[cfg(test)]
mod edge_tests {
    use super::*;
    #[test]
    fn empty_duplicates_and_owned_strings() {
        let mut empty: Vec<i32> = vec![];
        sort(&mut empty);
        let mut values = vec![3, -1, 3, 0, -1];
        sort(&mut values);
        assert_eq!(values, [-1, -1, 0, 3, 3]);
        let mut words = vec![String::from("z"), String::from("a")];
        sort(&mut words);
        assert_eq!(words, ["a", "z"]);
    }
}
