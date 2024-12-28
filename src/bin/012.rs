use num_prime::nt_funcs::factorize;
use project_euler::project_euler_solution;

project_euler_solution!(012);

// Return the number of factors of a number.
fn factor_num(num: u32) -> usize
{
    let factors = factorize(num);

    factors
        .iter()
        .map(|(&_base, &exp)| exp + 1)
        .product()
}

// Return an iterator of triangular numbers.
fn triangulars() -> impl Iterator<Item = u32>
{
    (1..).scan(0, |state, x| {
        *state += x;
        Some(*state)
    })
}

/// # Highly Divisible Triangular Number
///
/// The sequence of triangle numbers is generated by adding the natural numbers.
/// So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.
/// The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers:
///
/// 1: 1
///
/// 3: 1,3
///
/// 6: 1,2,3,6
///
/// 10: 1,2,5,10
///
/// 15: 1,3,5,15
///
/// 21: 1,3,7,21
///
/// 28: 1,2,4,7,14,28
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred
/// divisors?
fn project_euler_012() -> u32
{
    triangulars()
        .find(|&num| factor_num(num) > 500)
        .unwrap()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_factor_num()
    {
        assert_eq!(factor_num(28), 6);
        assert_eq!(factor_num(100), 9);
    }

    #[test]
    fn test_triangulars()
    {
        let mut triangulars = triangulars();
        assert_eq!(triangulars.next(), Some(1));
        assert_eq!(triangulars.next(), Some(3));
        assert_eq!(triangulars.next(), Some(6));
    }
}