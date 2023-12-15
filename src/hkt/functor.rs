use crate::{Plug, Plugged, Unplug, Unplugged};

pub trait Functor: Unplug + Plug<Unplugged<Self>> {
    fn map<B, F>(self, f: F) -> Plugged<Self, B>
    where
        Self: Plug<B>,
        F: FnMut(Unplugged<Self>) -> B;
}

