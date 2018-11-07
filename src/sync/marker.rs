use std::hash::{Hash, Hasher};

#[derive(Eq)]
pub enum Marker<T> {
    Left(T),
    Right(T),
}

impl<T> Marker<T> {
    fn value(&self) -> &T {
        match self {
            Marker::Left(x) | Marker::Right(x) => x,
        }
    }
}

impl<T: Eq> PartialEq for Marker<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(other.value())
    }
}

impl<T: Hash> Hash for Marker<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.value().hash(hasher)
    }
}
