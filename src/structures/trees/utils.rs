#[derive(Debug)]
pub enum NodeType {
    Left,
    Right,
    Root,
}

pub trait BinaryTreeUtil
where
    Self: Sized,
{
    fn left(&self) -> Option<Self>;

    fn right(&self) -> Option<Self>;

    fn depth(&self) -> usize {
        let mut left_depth = 0;
        if let Some(left) = &self.left() {
            left_depth = Self::depth(left);
        }
        let mut right_depth = 0;
        if let Some(right) = &self.right() {
            right_depth = Self::depth(right);
        }
        return std::cmp::max(left_depth, right_depth) + 1;
    }

    fn inorder(&self, f: &mut impl FnMut(&Self)) {
        if let Some(left) = self.left() {
            left.inorder(f);
        }
        f(&self);
        if let Some(right) = self.right() {
            right.inorder(f)
        }
    }

    fn preorder(&self, f: &mut impl FnMut(&Self)) {
        f(&self);
        if let Some(left) = self.left() {
            left.inorder(f);
        }
        if let Some(right) = self.right() {
            right.inorder(f)
        }
    }

    fn postorder(&self, f: &mut impl FnMut(&Self)) {
        if let Some(left) = self.left() {
            left.inorder(f);
        }
        if let Some(right) = self.right() {
            right.inorder(f)
        }
        f(&self);
    }
}

pub trait BinaryTreePrint<T>
where
    Self: BinaryTreeUtil,
{
    fn print_node(&self) -> String;

    fn pprint_helper(&self, prefix: String, node_type: NodeType) {
        let prefix_current = "|- ";

        println!(
            "{}{} {:?}({})",
            prefix,
            prefix_current,
            node_type,
            self.print_node()
        );

        let prefix_child = "|  ";
        let prefix = prefix + prefix_child;

        if let Some(left) = &self.left() {
            Self::pprint_helper(left, prefix.to_string(), NodeType::Left);
        }
        if let Some(right) = &self.right() {
            Self::pprint_helper(right, prefix.to_string(), NodeType::Right);
        }
    }

    fn pprint(&self) {
        Self::pprint_helper(&self, "".to_string(), NodeType::Root);
        return;
    }
}

pub trait BinaryTreeValidator<T>
where
    Self: BinaryTreeUtil,
    T: PartialOrd + Copy,
{
    fn val(&self) -> T;

    fn is_valid_helper(&self, min: Option<T>, max: Option<T>) -> bool {
        let val = self.val();

        let left_valid = match (self.left(), min) {
            (Some(left), None) => left.val() <= val,
            (Some(left), Some(min)) => left.val() <= val && left.val() >= min,
            _ => true,
        };
        let right_valid = match (self.right(), max) {
            (Some(right), None) => right.val() >= val,
            (Some(right), Some(max)) => right.val() >= val && right.val() < max,
            _ => true,
        };

        if !left_valid || !right_valid {
            return false;
        }
        let mut left_valid = true;
        if let Some(left) = &self.left() {
            left_valid = Self::is_valid_helper(left, min, Some(val));
        }

        let mut right_valid = true;
        if let Some(right) = &self.right() {
            right_valid = Self::is_valid_helper(right, Some(val), max);
        }
        return left_valid && right_valid;
    }

    fn is_valid_bst(&self) -> bool {
        Self::is_valid_helper(&self, None, None)
    }
}
