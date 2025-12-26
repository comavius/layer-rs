use super::*;

pub trait Replace<Target, Path>
where
    Self: Get<Target, Path>,
{
    fn replace(&mut self, replacement: Target) -> Target;
}

impl<Target, Path, Gettable> Replace<Target, Path> for Gettable
where
    Gettable: Get<Target, Path>,
{
    fn replace(&mut self, replacement: Target) -> Target {
        std::mem::replace(self.get_mut(), replacement)
    }
}
