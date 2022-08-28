pub fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub fn combination(n: usize, m: usize) -> usize {
    if m == 0 {
        1
    } else if m > n {
        0
    } else {
        if n == 0 {
            0
        } else {
            combination(n - 1, m - 1) + combination(n - 1, m) //stack overflow go brr
        }
    }
}

pub fn max_combinations(n: usize) -> usize {
    combination(n, n / 2)
}
