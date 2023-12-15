use crate::{Applicative, Plug, Plugged, Unplugged};

pub trait Monad: Applicative {
    fn bind<F, B>(self, f: F) -> Plugged<Self, B>
    where
        Self: Plug<F> + Plug<B>,
        F: FnMut(Unplugged<Self>) -> Plugged<Self, B> + Clone;
}
