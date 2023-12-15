mod hkt;

pub use hkt::*;

fn functor_test<F, F2, F3, A, B, C>(functor: F, fun: impl Fn(A) -> B, fun2: impl Fn(B) -> C) -> F3
where
    F: Functor + Plug<A> + Plug<B, Plugged = F2> + Unplug<Unplugged = A>,
    Unapplied<F>: Plug<A> + Plug<B> + Plug<C>,
    F2: Functor + Plug<B> + Plug<C, Plugged = F3> + Unplug<Unplugged = B>,
    Unapplied<F2>: Plug<B> + Plug<C>,
    F3: Functor + Plug<C>,
{
    functor.map(fun).map(fun2)
}

fn option_monad() {
    let v = Some(1).bind(|x| Some(x * 2)).app(Some(|x| x + 1));
    println!("{:?}", v);
}

fn result_monad() {
    let v: Result<_, ()> = Ok(7).bind(|x| Ok(x / 2)).app(Ok(|x| x - 1));
    println!("{:?}", v);
}

fn vec_monad() {
    fn x2(x: i32) -> i32 {
        x * 2
    }

    fn x20(x: i32) -> i32 {
        x * 20
    }

    let v = vec![1, 2, 3]
        .app(vec![x2 as fn(i32) -> i32, x20])
        .bind(|x| vec![x + 100]);

    println!("{:?}", v);
}

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("Vec: {:#?}", v);
    let r = functor_test(v, |i| (i + 1) as f32, |i| i * 2.0);
    println!("Result: {:#?}", r);

    option_monad();
    result_monad();
    vec_monad();

    Option::<u8>::test();
    Result::<u16, u32>::test();
    Vec::<f32>::test();
}

trait Test<T> {
    fn test() {}
}

impl<T, U> Test<T> for U
where
    U: Unplug<Unplugged = T>,
{
    fn test() {
        println!("Test: {}", std::any::type_name::<T>());
    }
}
