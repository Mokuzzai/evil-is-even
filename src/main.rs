#![feature(specialization)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![recursion_limit="4294967295"]

enum Num<const N: u32> {}
enum Add<const A: u32, const B: u32> {}

trait Eval {
    const N: u32;
}

impl<const N: u32> Eval for Num<N> {
    const N: u32 = N;
}

impl<const A: u32, const B: u32> Eval for Add<A, B> {
    const N: u32 = A + B;
}

trait EvenOdd {
    fn even_odd(n: u32);
}

const MAX: u32 = 2048;

enum Cond<const B: bool> {}

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
            _ => Add::<{ Self::N }, 1>::even_odd(n), 
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
            _ => Add::<{ Self::N }, 1>::even_odd(n), 
        }
    }
}

fn even_odd(n: u32) {
    Num::<0>::even_odd(n)
}

fn main() {
    even_odd(7)
}
