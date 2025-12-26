use super::*;

pub struct Node<Req, SubtreeL, SubtreeR> {
    pub requirement: Req,
    pub prev_left: SubtreeL,
    pub prev_right: SubtreeR,
}

pub trait Depth
where
    Self::Output: IsNumber,
{
    type Output;
}

impl Depth for () {
    type Output = Zero;
}

impl<Req, SubtreeL, SubtreeR> Depth for Node<Req, SubtreeL, SubtreeR>
where
    SubtreeL: Depth,
    SubtreeR: Depth,
    <SubtreeL as Depth>::Output: IsNumber,
    <SubtreeR as Depth>::Output: IsNumber,
    (): Max<<SubtreeL as Depth>::Output, <SubtreeR as Depth>::Output>,
{
    type Output = Succ<<() as Max<SubtreeL::Output, SubtreeR::Output>>::Output>;
}

pub trait MinDepth
where
    Self::Output: IsNumber,
{
    type Output;
}

impl MinDepth for () {
    type Output = Zero;
}

impl<Req, SubtreeL, SubtreeR> MinDepth for Node<Req, SubtreeL, SubtreeR>
where
    SubtreeL: MinDepth,
    SubtreeR: MinDepth,
    <SubtreeL as MinDepth>::Output: IsNumber,
    <SubtreeR as MinDepth>::Output: IsNumber,
    (): Min<<SubtreeL as MinDepth>::Output, <SubtreeR as MinDepth>::Output>,
{
    type Output = Succ<<() as Min<SubtreeL::Output, SubtreeR::Output>>::Output>;
}
