use super::*;

/// Getter trait for type level binary tree node.
/// This requires that the target requirement exists in the tree.
pub trait Get<Target, Path>
where
    Self: Has<Target, Path>,
{
    fn get(&self) -> &Target;
    fn get_mut(&mut self) -> &mut Target;
}

// get: base case
impl<Target, SubtreeL, SubtreeR> Get<Target, Here> for Node<Target, SubtreeL, SubtreeR> {
    fn get(&self) -> &Target {
        &self.requirement
    }

    fn get_mut(&mut self) -> &mut Target {
        &mut self.requirement
    }
}

// get: left subtree case
impl<Target, Req, SubtreeL, SubtreeR, SubtreePath> Get<Target, Left<SubtreePath>>
    for Node<Req, SubtreeL, SubtreeR>
where
    SubtreeL: Get<Target, SubtreePath>,
{
    fn get(&self) -> &Target {
        self.prev_left.get()
    }

    fn get_mut(&mut self) -> &mut Target {
        self.prev_left.get_mut()
    }
}

// get: right subtree case
impl<Target, Req, SubtreeL, SubtreeR, SubtreePath> Get<Target, Right<SubtreePath>>
    for Node<Req, SubtreeL, SubtreeR>
where
    SubtreeR: Get<Target, SubtreePath>,
{
    fn get(&self) -> &Target {
        self.prev_right.get()
    }

    fn get_mut(&mut self) -> &mut Target {
        self.prev_right.get_mut()
    }
}
