use crate::{Applicative, ForAll, Functor, Monad, Plug, Plugged, Unplug, Unplugged};

impl<T> Unplug for Option<T> {
    type Unapplied = Option<ForAll>;
    type Unplugged = T;
}

impl<A, B> Plug<B> for Option<A> {
    type Plugged = Option<B>;
}

impl<A> Functor for Option<A> {
    fn map<B, F>(self, mut f: F) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> B,
    {
        match self {
            Some(v) => Some(f(v)),
            None => None,
        }
    }
}

impl<A> Applicative for Option<A>
where
    A: Clone,
{
    fn pure(a: A) -> Self {
        Some(a)
    }

    fn app<F, B>(self, fs: Plugged<Self, F>) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> B,
    {
        match fs {
            Some(f) => self.map(f),
            None => None,
        }
    }
}

impl<A> Monad for Option<A>
where
    A: Clone,
{
    fn bind<F, B>(self, mut f: F) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> Plugged<Self, B>,
    {
        match self {
            Some(v) => f(v),
            None => todo!(),
        }
    }
}
