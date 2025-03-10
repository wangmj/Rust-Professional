use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let mut distinct_sets = HashSet::new();
    let chars = input_str.split(",").collect::<Vec<&str>>();
    for c in chars{
        distinct_sets.insert(c);
    }
    distinct_sets.len()
}
