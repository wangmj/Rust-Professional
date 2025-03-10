pub fn new_birthday_probability(n: u32) -> f64 {
    if n == 0 {
        0.
    } else if n > 365u32 {
        1.
    } else {
        let mut pro: f64 = 1.0;
        for i in 0..n {
            pro *= (365_f64 - i as f64) / 365_f64;
        }
        1.0 - pro
    }
}
