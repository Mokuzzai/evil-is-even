#![feature(specialization)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![recursion_limit="4294967295"]

fn even_odd<T: Eval + Print>(n: u32) {
	match n {
		_ if T::N == n => T::print(),
		_ => (),
	}
}

trait Print {
	fn print();
}

impl<T: Eval> Print for T
where
	Cond<{ T::N % 2 == 0 }>: True,
{
	 fn print() {
		println!("even")
	}
}

impl<T> Print for T {
	default fn print() {
		println!("odd")
	}
}

struct One;

struct Add<A, B>(A, B);

trait Eval {
	const N: u32;
}

impl Eval for One {
	const N: u32 = 1;
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

trait LtU8Max {}

struct Cond<const B: bool>;

trait True {}

impl True for Cond<true> {}

const MAX: u32 = 2u32.pow(11);

impl<T: Eval> LtU8Max for T
where
	Cond<{ T::N < MAX }>: True,
	{}

impl<T: Eval> EvenOdd for T {
	default fn even_odd(_: u32) {}
}

impl<T: Eval + LtU8Max + Print> EvenOdd for T {
	fn even_odd(n: u32) {
		even_odd::<Self>(n);

		Add::<Self, One>::even_odd(n);
	}
}

fn main() {
	println!("MAX: {MAX}");

	One::even_odd(12);
}
