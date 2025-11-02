use coursera_rust::solutions_code::solution1;
use coursera_rust::solutions_code::solution3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(&mut vec![1, 3, 6, 4, 1, 2]), 5);
        assert_eq!(solution1(&mut vec![1, 2, 3]), 4);
        assert_eq!(solution1(&mut vec![-1, -3]), 1);
    }
    #[test]
    fn test_solution2() {
        assert_eq!(solution3(&mut vec![1, 3, 6, 4, 1, 2]), 5);
        assert_eq!(solution3(&mut   vec![1, 2, 3]), 4);
        assert_eq!(solution3(&mut vec![-1, -3]), 1);
    }
}