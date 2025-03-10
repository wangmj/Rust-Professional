pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut fibnacci_num = 0;
    let mut sum = 0;
    let mut n = 0;
    while fibnacci_num < threshold {
        if fibnacci_num % 2 != 0 {
            sum += fibnacci_num;
        }
        n += 1;
        fibnacci_num = get_fibnacci(n);
    }
    sum
}
fn get_fibnacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        get_fibnacci(n - 1) + get_fibnacci(n - 2)
    }
}
