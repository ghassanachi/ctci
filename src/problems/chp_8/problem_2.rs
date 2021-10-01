use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Very naive solution using normal dfs to find the path. Since shortest path is not required, dfs
/// works well and is simple. I'm prioritizing Right and Down moves as a simple heuristic since
/// they get us closer to the destination.
///
/// Alternative solution with similar complexity would be a 2 point BFS search, and this would also
/// give us the shortest path.
pub fn find_path_in_grid<const N: usize, const M: usize>(
    grid: &[[(); M]; N],
    blocked: &[(usize, usize)],
) -> Option<Vec<Direction>> {
    if grid.is_empty() || grid.first().is_none() {
        return None;
    }

    let mut blocked = HashSet::from_iter(blocked.iter().map(|n| *n));
    let mut path: Vec<Direction> = Vec::new();
    if dfs_grid(grid, &mut blocked, (0, 0), &mut path) {
        return Some(path);
    }
    None
}

fn dfs_grid<const N: usize, const M: usize>(
    grid: &[[(); M]; N],
    blocked: &mut HashSet<(usize, usize)>,
    position: (usize, usize),
    path: &mut Vec<Direction>,
) -> bool {
    assert!(grid.len() > 0, "grid should not be empty");
    let (x, y) = position;

    if x + 1 == M && y + 1 == N {
        return true;
    }

    // If the cell is blocked remove the last element from it;
    if blocked.contains(&position) {
        path.pop();
        return false;
    }

    blocked.insert(position);

    if x + 1 < M {
        path.push(Direction::Right);
        if dfs_grid(grid, blocked, (x + 1, y), path) {
            return true;
        }
    }

    if y + 1 < N {
        path.push(Direction::Down);
        if dfs_grid(grid, blocked, (x, y + 1), path) {
            return true;
        }
    }

    if x >= 1 {
        path.push(Direction::Left);
        if dfs_grid(grid, blocked, (x - 1, y), path) {
            return true;
        }
    }

    if y >= 1 {
        path.push(Direction::Up);
        if dfs_grid(grid, blocked, (x, y - 1), path) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::*;

    /// 0 1 0
    /// 0 1 0
    /// 0 0 0
    #[test]
    fn find_path_in_grid_1() {
        let grid = [[(); 3]; 3];
        let blocked = [(1, 0), (1, 1)];
        assert_eq!(
            find_path_in_grid(&grid, &blocked),
            Some(vec![Down, Down, Right, Right])
        )
    }

    #[test]
    fn find_path_in_grid_2() {
        let grid = [[(); 3]; 3];
        let blocked = [(1, 0), (1, 1), (1, 2)];
        assert_eq!(find_path_in_grid(&grid, &blocked), None);

        let grid = [[(); 3]; 0];
        let blocked = [];
        assert_eq!(find_path_in_grid(&grid, &blocked), None);

        let grid = [[(); 0]; 3];
        let blocked = [];
        assert_eq!(find_path_in_grid(&grid, &blocked), None)
    }

    #[test]
    /// 0 0 1 0
    /// 1 0 0 0
    /// 1 1 1 0
    /// 0 0 0 0
    /// 0 1 1 1
    /// 0 0 0 0
    fn find_path_in_grid_3() {
        let grid = [[(); 4]; 6];
        let blocked = [
            (2, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (1, 4),
            (2, 4),
            (3, 4),
        ];
        assert_eq!(
            find_path_in_grid(&grid, &blocked),
            Some(vec![
                Right, Down, Right, Right, Down, Down, Left, Left, Left, Down, Down, Right, Right,
                Right
            ])
        )
    }
}
