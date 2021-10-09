use std::cmp::Ordering;

pub struct Package {
    height: u32,
    width: u32,
    depth: u32,
}

impl Package {
    pub fn new(height: u32, width: u32, depth: u32) -> Self {
        Self {
            height,
            width,
            depth,
        }
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        return self.height == other.height
            && self.width == other.width
            && self.depth == other.depth;
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (
            self.height.cmp(&other.height),
            self.width.cmp(&other.width),
            self.depth.cmp(&other.depth),
        ) {
            (Ordering::Less, Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Greater, Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Equal, Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

pub fn package_stack_height(packages: &mut [Package]) -> u32 {
    packages.sort_by(|a, b| b.height.cmp(&a.height));
    let mut memo = vec![0u32; packages.len()];
    package_stack_height_helper(packages, None, 0, 0, &mut memo)
}

fn package_stack_height_helper(
    packages: &[Package],
    top_idx: Option<usize>,
    cur_idx: usize,
    stack_height: u32,
    memo: &mut [u32],
) -> u32 {
    if cur_idx == packages.len() {
        return stack_height;
    }

    if memo[cur_idx] != 0 {
        return memo[cur_idx];
    }

    let current_package = &packages[cur_idx];

    let can_stack = top_idx.map_or(true, |idx| {
        current_package.partial_cmp(&packages[idx]) == Some(Ordering::Less)
    });

    let with_me = if can_stack {
        let height = package_stack_height_helper(
            packages,
            Some(cur_idx),
            cur_idx + 1,
            stack_height + current_package.height,
            memo,
        );
        memo[cur_idx] = height - stack_height;
        height
    } else {
        0
    };

    let without_me =
        package_stack_height_helper(packages, top_idx, cur_idx + 1, stack_height, memo);
    with_me.max(without_me)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_stack_height_1() {
        assert_eq!(package_stack_height(&mut []), 0);
        assert_eq!(package_stack_height(&mut [Package::new(1, 1, 1)]), 1);

        let mut packages = [
            Package::new(1, 4, 1),
            Package::new(3, 3, 3),
            Package::new(1, 2, 2),
        ];
        assert_eq!(package_stack_height(&mut packages), 4);
    }

    #[test]
    fn package_stack_height_2() {
        let mut packages = [
            Package::new(2, 2, 2),
            Package::new(3, 3, 3),
            Package::new(3, 3, 4),
        ];
        assert_eq!(package_stack_height(&mut packages), 5);
    }

    #[test]
    fn package_stack_height_3() {
        let mut packages = [
            Package::new(4, 4, 4),
            Package::new(3, 3, 3),
            Package::new(2, 2, 2),
            Package::new(1, 1, 1),
        ];
        assert_eq!(package_stack_height(&mut packages), 10);
    }
}
