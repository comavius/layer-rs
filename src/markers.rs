pub struct Zero;
pub struct Succ<N>(std::marker::PhantomData<N>);

pub trait Equal<N1, N2> {}
impl Equal<Zero, Zero> for () {}
impl<N1, N2> Equal<Succ<N1>, Succ<N2>> for () where (): Equal<N1, N2> {}

pub trait IsNumber {}
impl IsNumber for Zero {}
impl<N> IsNumber for Succ<N> where N: IsNumber {}

pub trait GreaterThanOrEqual<N1, N2> {}
impl GreaterThanOrEqual<Zero, Zero> for () {}
impl<N> GreaterThanOrEqual<Succ<N>, Zero> for () {}
impl<N1, N2> GreaterThanOrEqual<Succ<N1>, Succ<N2>> for () where (): GreaterThanOrEqual<N1, N2> {}

pub trait LessThan<N1, N2> {}
impl<N> LessThan<Zero, Succ<N>> for () where N: IsNumber {}
impl<N1, N2> LessThan<Succ<N1>, Succ<N2>> for () where (): LessThan<N1, N2> {}

pub trait Max<N1, N2>
where
    N1: IsNumber,
    N2: IsNumber,
    Self::Output: IsNumber,
{
    type Output;
}

impl Max<Zero, Zero> for () {
    type Output = Zero;
}

impl<N> Max<Succ<N>, Zero> for ()
where
    N: IsNumber,
{
    type Output = Succ<N>;
}
impl<N> Max<Zero, Succ<N>> for ()
where
    N: IsNumber,
{
    type Output = Succ<N>;
}

impl<N1, N2> Max<Succ<N1>, Succ<N2>> for ()
where
    N1: IsNumber,
    N2: IsNumber,
    (): Max<N1, N2>,
{
    type Output = Succ<<() as Max<N1, N2>>::Output>;
}

pub trait Min<N1, N2>
where
    N1: IsNumber,
    N2: IsNumber,
    Self::Output: IsNumber,
{
    type Output;
}

impl Min<Zero, Zero> for () {
    type Output = Zero;
}

impl<N> Min<Succ<N>, Zero> for ()
where
    N: IsNumber,
{
    type Output = Zero;
}
impl<N> Min<Zero, Succ<N>> for ()
where
    N: IsNumber,
{
    type Output = Zero;
}

impl<N1, N2> Min<Succ<N1>, Succ<N2>> for ()
where
    N1: IsNumber,
    N2: IsNumber,
    (): Min<N1, N2>,
{
    type Output = Succ<<() as Min<N1, N2>>::Output>;
}

pub struct Here;
pub struct Left<PrevDirection>(std::marker::PhantomData<PrevDirection>);
pub struct Right<PrevDirection>(std::marker::PhantomData<PrevDirection>);
