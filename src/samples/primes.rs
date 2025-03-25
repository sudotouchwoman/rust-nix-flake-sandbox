// factorize does integer factorization
pub fn factorize_loop(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;

    for i in 2..=(n as f64).sqrt().ceil() as u64 {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorize() {
        assert_eq!(vec![2, 5], factorize_loop(10));
        assert_eq!(vec![2], factorize_loop(2));
        assert_eq!(vec![2, 3], factorize_loop(6));
        assert_eq!(vec![5, 5], factorize_loop(25));
        assert_eq!(vec![17, 53], factorize_loop(901));
    }
}
