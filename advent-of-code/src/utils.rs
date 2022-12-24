use derive_more::Add;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default, Add)]
pub struct Vector<T>(pub T, pub T);

impl<T> From<(T, T)> for Vector<T> {
    fn from(loc: (T, T)) -> Self {
        Vector(loc.0, loc.1)
    }
}
