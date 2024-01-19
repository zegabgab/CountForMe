pub fn fibonacci(n: u32) -> u32 {
    fn f(n: u32, i: u32, f_i: u32, f_i_minus_1: u32) -> u32 {
        match n - i {
            0 => f_i,
            _ => f(n, i + 1, f_i + f_i_minus_1, f_i)
        }
    }

    f(n, 0, 0, 1)
}