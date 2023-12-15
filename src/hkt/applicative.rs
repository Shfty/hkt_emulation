use crate::{Functor, Plug, Plugged, Unapplied, Unplug, Unplugged};

pub trait Applicative: Functor {
    fn pure(s: Unplugged<Self>) -> Self;
    fn app<F, B>(self, f: Plugged<Self, F>) -> Plugged<Self, B>
    where
        F: FnMut(Unplugged<Self>) -> B,
        Self: Plug<F> + Plug<B> + Unplug,
        Plugged<Self, F>: Unplug<Unapplied = Unapplied<Self>, Unplugged = F> + Plug<F> + Clone,
        Unapplied<Self>: Plug<F>;
}
