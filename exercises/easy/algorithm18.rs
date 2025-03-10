/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end], 
    merge all overlapping intervals and return a list of non-overlapping intervals.
    
    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
    
    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement the logic to merge overlapping intervals
    intervals.sort_by(|a, b| a.first().cmp(&b.first()));

    let mut res = Vec::new();
    let mut res_insert_row_index = 0_usize;
    let mut interval_row_index = 1_usize;
    
    let interval_row_count = intervals.len();
    let first_row=&intervals[0];
    res.push(vec![intervals[0][0]]);
    let mut max_val=first_row[first_row.len()-1];
    while interval_row_index < interval_row_count {
        let cur_row = &intervals[interval_row_index];

        let cur_first_val = &cur_row[0];
        let cur_last_val = cur_row[cur_row.len() - 1];
        match max_val.cmp(cur_first_val){
            std::cmp::Ordering::Greater|std::cmp::Ordering::Equal=>{
                max_val=max_val.max(cur_last_val);
                interval_row_index+=1;
            },
            std::cmp::Ordering::Less=>{
                res[res_insert_row_index].push(max_val);
                res.push(vec![*cur_first_val]);
                res_insert_row_index+=1;
                max_val=cur_last_val;
                interval_row_index+=1;
            },
        }
    }
    res[res_insert_row_index].push(max_val);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18]
        ]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 5]
        ]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![0, 4]
        ]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![1, 10],
            vec![2, 6],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 10]
        ]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![4, 7],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 7],
            vec![8, 10]
        ]);
    }
}
