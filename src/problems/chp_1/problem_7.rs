pub fn rotate_image_90(image: &mut [Vec<u32>]) {
    let len = image.len();
    // Transponse the matrix on diagonal
    for x in 0..len {
        for y in 0..len - x - 1 {
            let tmp = image[x][y];
            image[x][y] = image[len - y - 1][len - x - 1];
            image[len - y - 1][len - x - 1] = tmp;
        }
    }

    // Flip on horizontal
    for y in 0..len {
        for x in 0..(len / 2) {
            let tmp = image[x][y];
            image[x][y] = image[len - x - 1][y];
            image[len - x - 1][y] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_image_90_1() {
        let mut image = vec![vec![1, 2], vec![3, 4]];
        rotate_image_90(&mut image);
        let expected = vec![vec![3, 1], vec![4, 2]];
        assert_eq!(expected, image);
    }

    #[test]
    fn test_rotate_image_90_2() {
        let mut image = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        rotate_image_90(&mut image);
        let expected = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];
        assert_eq!(expected, image);
    }

    #[test]
    fn test_rotate_image_90_3() {
        let mut image = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        rotate_image_90(&mut image);
        let expected = vec![
            vec![21, 16, 11, 6, 1],
            vec![22, 17, 12, 7, 2],
            vec![23, 18, 13, 8, 3],
            vec![24, 19, 14, 9, 4],
            vec![25, 20, 15, 10, 5],
        ];
        assert_eq!(expected, image);
    }
}
