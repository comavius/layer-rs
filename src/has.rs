use super::*;

pub trait Has<Target, Path> {}

impl<Req, SubtreeL, SubtreeR> Has<Req, Here> for Node<Req, SubtreeL, SubtreeR> {}
impl<Target, Req, SubtreeL, SubtreeR, SubtreePath> Has<Target, Left<SubtreePath>>
    for Node<Req, SubtreeL, SubtreeR>
where
    SubtreeL: Has<Target, SubtreePath>,
{
}
impl<Target, Req, SubtreeL, SubtreeR, SubtreePath> Has<Target, Right<SubtreePath>>
    for Node<Req, SubtreeL, SubtreeR>
where
    SubtreeR: Has<Target, SubtreePath>,
{
}

pub struct Base;
pub struct Specialized;

pub trait NotHas<Target, Kind> {}
impl<Target, Req, SubtreeL, SubtreeR> NotHas<Target, Base> for Node<Req, SubtreeL, SubtreeR> {}

impl<Target, Req, SubtreeL, SubtreeR, Path> NotHas<Target, (Specialized, Path)>
    for Node<Req, SubtreeL, SubtreeR>
where
    Node<Req, SubtreeL, SubtreeR>: Has<Target, Path>,
{
}
