/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement the logic to rotate the matrix 90 degrees in place
    let x_origin_max = matrix.len();
    let y_origin_max = match matrix.first() {
        Some(v) => v.len(),
        None => 0,
    };
    //让matrix成为一个矩形矩阵
    if x_origin_max > y_origin_max {
        for _ in 0..(x_origin_max - y_origin_max) {
            matrix.iter_mut().for_each(|row| {
                row.push(0);
            });
        }
    } else if y_origin_max > x_origin_max {
        for _ in 0..(y_origin_max - x_origin_max) {
            matrix.push(vec![0; y_origin_max]);
        }
    }
    //  交换
    let mut x = 0;

    let max = x_origin_max.max(y_origin_max);
    while x < max {
        let mut y = x + 1;
        while y < max {
            let (above, down) = matrix.split_at_mut(x + 1);
            std::mem::swap(&mut above[x][y], &mut down[y - x - 1][x]);

            y += 1;
        }
        x += 1;
    }

    //反转
    matrix.iter_mut().for_each(|row| {
        row.reverse();
    });
    //收缩
    if x_origin_max > y_origin_max {
        for _ in 0..(x_origin_max - y_origin_max) {
            matrix.remove(matrix.len() - 1);
        }
        matrix.shrink_to(y_origin_max);
    } else if y_origin_max > x_origin_max {
        for _ in 0..(y_origin_max-x_origin_max){
            matrix.iter_mut().for_each(|row| {
                row.remove(row.len()-1);
            });
        }
        matrix.iter_mut().for_each(|row| {
            row.shrink_to_fit();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
