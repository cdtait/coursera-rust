/*
    int solution(vector<int> &A);

that, given an array A of N integers, returns the smallest positive integer (greater than 0) that does not occur in A.

For example, given A = [1, 3, 6, 4, 1, 2], the function should return 5.

Given A = [1, 2, 3], the function should return 4.

Given A = [−1, −3], the function should return 1.

Write an efficient algorithm for the following assumptions:

        N is an integer within the range [1..100,000];
        each element of array A is an integer within the range [−1,000,000..1,000,000].
 */
pub fn solution1(arr: &mut Vec<i32>) -> i32 {
    let n = arr.len() as i32;

    for i in 0..arr.len() {
        while arr[i] > 0 && arr[i] <= n {
            let pos = (arr[i] - 1) as usize;
            if arr[pos] != arr[i] {
                let temp = arr[pos];
                arr[pos] = arr[i];
                arr[i] = temp;
            } else {
                break;
            }
        }
    }

    for i in 0..arr.len() {
        if arr[i] != (i as i32) + 1 {
            return (i as i32) + 1;
        }
    }

    n + 1
}

/*
This is an example of an optimized C++ solution how can we get a more idiomatic Rust solution  using HashSet
*/
pub fn solution3(arr: &Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut positives = HashSet::new();
    for &num in arr.iter() {
        if num > 0 {
            positives.insert(num);
        }
    }

    let mut smallest_missing = 1;
    while positives.contains(&smallest_missing) {
        smallest_missing += 1;
    }

    smallest_missing
}
