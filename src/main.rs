#![feature(specialization)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![recursion_limit="4294967295"]

struct Num<const N: u32>;
struct Add<A, B>(A, B);

trait Eval {
    const N: u32;
}

impl<const N: u32> Eval for Num<N> {
    const N: u32 = N;
}

impl<A, B> Eval for Add<A, B> 
where
    A: Eval,
    B: Eval,
{
    const N: u32 = A::N + B::N;
}

trait EvenOdd {
    fn even_odd(n: u32);
}

const MAX: u32 = 1024;

struct Cond<const B: bool>;

trait True {}

impl True for Cond<true> {}

trait Even {}

impl<T> Even for T 
where
    T: Eval,
    Cond<{ T::N % 2 == 0 }>: True,
{}

impl<T> EvenOdd for T
where
    T: Eval,
{
    default fn even_odd(n: u32) {}
}

impl<T> EvenOdd for T
where
    T: Eval,
    Cond<{ T::N < MAX }>: True,
{
    default fn even_odd(n: u32) {
        match n {
            _ if n == T::N => println!("odd"),
            _ => Add::<Self, Num<1>>::even_odd(n), 
        }
    }
}

impl<T> EvenOdd for T
where
    T: Eval + Even,
    Cond<{ T::N < MAX }>: True,
{
    fn even_odd(n: u32) {
        match n {
            _ if n == T::N => println!("even"),
            _ => Add::<Self, Num<1>>::even_odd(n), 
        }
    }
}

pub fn even_odd(n: u32) {
    Num::<0>::even_odd(n)
}

fn main() {
    even_odd(7)
}
