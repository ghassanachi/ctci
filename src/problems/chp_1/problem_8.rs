use std::collections::HashSet;
pub fn zero_matrix(matrix: &mut [Vec<u32>]) {
    let mut col_set = HashSet::new();
    let mut row_set = HashSet::new();
    for (row_idx, row) in &mut matrix.iter().enumerate() {
        for (col_idx, col) in &mut row.iter().enumerate() {
            if *col == 0u32 {
                col_set.insert(col_idx);
                row_set.insert(row_idx);
            }
        }
    }

    for (row_idx, row) in &mut matrix.iter_mut().enumerate() {
        for (col_idx, col) in &mut row.iter_mut().enumerate() {
            if col_set.contains(&col_idx) || row_set.contains(&row_idx) {
                *col = 0u32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_matrix_1() {
        let mut image = vec![vec![1, 2], vec![3, 4]];
        zero_matrix(&mut image);
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(expected, image);
    }

    #[test]
    fn test_zero_matrix_2() {
        let mut image = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        zero_matrix(&mut image);
        let expected = vec![
            vec![1, 0, 3, 4],
            vec![0, 0, 0, 0],
            vec![9, 0, 11, 12],
            vec![13, 0, 15, 16],
        ];
        assert_eq!(expected, image);
    }

    #[test]
    fn test_zero_matrix_3() {
        let mut image = vec![
            vec![0, 2, 3, 4, 5],
            vec![6, 0, 8, 9, 10],
            vec![11, 0, 0, 14, 15],
            vec![16, 17, 18, 0, 20],
            vec![21, 22, 23, 24, 25],
        ];
        zero_matrix(&mut image);
        let expected = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 25],
        ];
        assert_eq!(expected, image);
    }
}
