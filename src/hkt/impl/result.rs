use crate::{Applicative, ForAll, Functor, Monad, Plug, Plugged, Unplug, Unplugged};

impl<T, E> Unplug for Result<T, E> {
    type Unapplied = Result<ForAll, ForAll>;
    type Unplugged = T;
}

impl<A, E, B> Plug<B> for Result<A, E> {
    type Plugged = Result<B, E>;
}

impl<A, E> Functor for Result<A, E> {
    fn map<B, F>(self, mut f: F) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> B,
    {
        match self {
            Ok(v) => Ok(f(v)),
            Err(e) => Err(e),
        }
    }
}

impl<A, E> Applicative for Result<A, E>
where
    A: Clone,
{
    fn pure(a: A) -> Self {
        Ok(a)
    }

    fn app<F, B>(self, fs: Plugged<Self, F>) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> B,
    {
        match fs {
            Ok(f) => self.map(f),
            Err(e) => Err(e),
        }
    }
}

impl<A, E> Monad for Result<A, E>
where
    A: Clone,
{
    fn bind<F, B>(self, mut f: F) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> Plugged<Self, B>,
    {
        match self {
            Ok(v) => f(v),
            Err(e) => Err(e),
        }
    }
}
