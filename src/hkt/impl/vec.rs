use crate::{Applicative, ForAll, Functor, Monad, Plug, Plugged, Unplug, Unplugged};

impl<T> Unplug for Vec<T> {
    type Unapplied = Vec<ForAll>;
    type Unplugged = T;
}

impl<A, B> Plug<B> for Vec<A> {
    type Plugged = Vec<B>;
}

impl<A> Functor for Vec<A> {
    fn map<B, F>(self, f: F) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A> Applicative for Vec<A>
where
    A: Clone,
{
    fn pure(a: A) -> Self {
        vec![a]
    }

    fn app<F, B>(self, fs: Plugged<Self, F>) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> B,
        Plugged<Self, F>: Clone,
    {
        self.map(|x| fs.clone().map(|mut f| f(x.clone())))
            .into_iter()
            .flatten()
            .collect()
    }
}

impl<A: Clone> Monad for Vec<A> {
    fn bind<F, B>(self, mut f: F) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> Plugged<Self, B>,
    {
        self.into_iter().map(|x| f(x)).flatten().collect()
    }
}
