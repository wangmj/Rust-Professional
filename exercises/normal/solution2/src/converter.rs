use std::collections::HashMap;

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let mut num_chars = Vec::new();
    let mut radix = String::new();
    let mut is_radix = false;
    let mut map = HashMap::with_capacity(6);
    map.insert("10", "a");
    map.insert("11", "b");
    map.insert("12", "c");
    map.insert("13", "d");
    map.insert("14", "e");
    map.insert("15", "f");
    for char in num_str.chars() {
        if char == '(' {
            is_radix = true;
            continue;
        } else if char == ')' {
            break;
        }
        if !is_radix {
            if !char.is_numeric() {
                panic!("The input str is not valid");
            } else {
                num_chars.push(char);
            }
        } else {
            radix.push(char);
        }
    }
    let radix = match radix.parse::<u32>() {
        Ok(v) => v,
        Err(e) => panic!("The input str is not valid!"),
    };
    let mut sum: u32 = 0;
    num_chars.reverse();
    let num_lens = num_chars.len();
    for n in (0..num_lens).rev() {
        match num_chars[n].to_digit(radix) {
            Some(s) => sum += s * (radix.pow(n as u32)),
            None => {}
        }
    }

    let mut res_vec = Vec::new();
    while sum > 0 {
        let c = sum % to_base;
        res_vec.push(c.to_string());
        sum = sum / to_base;
    }
    res_vec.reverse();
    if to_base == 16 {
        for str in res_vec.iter_mut() {
            if map.contains_key(str.as_str()) {
                *str = map[str.as_str()].to_string();
            }
        }
    }
    let res = res_vec.join("");
    dbg!(&res);
    res
}
