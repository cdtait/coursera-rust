use coursera_rust::solutions_code::solution1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(&mut vec![1, 3, 6, 4, 1, 2]), 5);
        assert_eq!(solution1(&mut vec![1, 2, 3]), 4);
        assert_eq!(solution1(&mut vec![-1, -3]), 1);
    }
}