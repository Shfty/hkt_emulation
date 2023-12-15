mod applicative;
mod functor;
mod r#impl;
mod monad;

pub use applicative::*;
pub use functor::*;
pub use monad::*;
pub use r#impl::*;

/// Inner type representing an unapplied higher-kinded type (ex. Vec<_> or Option<_>)
#[derive(Debug)]
pub struct ForAll;

/// For some type T<U>, exposes T<ForAll> and U as associated types
pub trait Unplug {
    type Unapplied;
    type Unplugged;
}

pub type Unapplied<T> = <T as Unplug>::Unapplied;
pub type Unplugged<T> = <T as Unplug>::Unplugged;

/// For T<ForAll> and some type U, exposes T<U> as an associated type
pub trait Plug<T> {
    type Plugged;
}

pub type Plugged<T, U> = <T as Plug<U>>::Plugged;
