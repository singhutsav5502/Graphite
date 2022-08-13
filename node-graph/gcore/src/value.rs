use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::sync::atomic::AtomicBool;

use crate::ops::CloneNode;
use crate::structural::ComposeNode;
use crate::Node;

pub struct IntNode<const N: u32>;
impl<'n, const N: u32> Node<'n, ()> for IntNode<N> {
	type Output = u32;
	fn eval(&self, _: ()) -> u32 {
		N
	}
}

#[derive(Default)]
pub struct ValueNode<T>(pub T);
impl<'n, T: 'n> Node<'n, ()> for ValueNode<T> {
	type Output = &'n T;
	fn eval(&'n self, _: ()) -> Self::Output {
		&self.0
	}
}

impl<T> ValueNode<T> {
	pub const fn new(value: T) -> ValueNode<T> {
		ValueNode(value)
	}
}

impl<'n, T: Clone + 'n> ValueNode<T> {
	pub const fn clone(self) -> ComposeNode<'n, (), ValueNode<T>, CloneNode> {
		ComposeNode::new(self, CloneNode)
	}
}

impl<T> From<T> for ValueNode<T> {
	fn from(value: T) -> Self {
		ValueNode::new(value)
	}
}

#[derive(Default)]
pub struct DefaultNode<T>(PhantomData<T>);
impl<'n, T: Default + 'n> Node<'n, ()> for DefaultNode<T> {
	type Output = T;
	fn eval(&self, _: ()) -> T {
		T::default()
	}
}

#[repr(C)]
/// Return the unit value
pub struct UnitNode;
impl<'n> Node<'n, ()> for UnitNode {
	type Output = ();
	fn eval(&'n self, _: ()) -> Self::Output {}
}

pub struct InputNode<T>(MaybeUninit<T>, AtomicBool);
impl<'n, T: 'n> Node<'n, ()> for InputNode<T> {
	type Output = &'n T;
	fn eval(&'n self, _: ()) -> Self::Output {
		if self.1.load(core::sync::atomic::Ordering::SeqCst) {
			unsafe { self.0.assume_init_ref() }
		} else {
			panic!("tried to access an input before setting it")
		}
	}
}

impl<T> InputNode<T> {
	pub const fn new() -> InputNode<T> {
		InputNode(MaybeUninit::uninit(), AtomicBool::new(false))
	}
}
