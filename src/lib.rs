#![feature(arbitrary_self_types)]

extern crate pin_api;
extern crate futures_core;
extern crate futures_executor;

mod executor;
mod unsafe_pin;

use pin_api::{PinMut, PinBox, Unpin};
use futures_core::{Future, Stream, Poll, task};

pub use executor::{StableExecutor, block_on_stable};
use unsafe_pin::UnsafePin;

pub type PinnedFuture<'a, T, E> = PinBox<Future<Item = T, Error = E> + 'a>;
pub type PinnedFutureSend<'a, T, E> = PinBox<Future<Item = T, Error = E> + Send + 'a>;
pub type PinnedStream<'a, T, E> = PinBox<Stream<Item = T, Error = E> + 'a>;
pub type PinnedStreamSend<'a, T, E> = PinBox<Stream<Item = T, Error = E> + Send + 'a>;

pub trait StableFuture {
    type Item;
    type Error;

    fn poll(self: PinMut<Self>, ctx: &mut task::Context) -> Poll<Self::Item, Self::Error>;

    fn pin<'a>(self) -> PinnedFuture<'a, Self::Item, Self::Error>
        where Self: Sized + 'a
    {
        PinBox::new(unsafe { UnsafePin::new(self) })
    }

    fn pin_send<'a>(self) -> PinnedFutureSend<'a, Self::Item, Self::Error>
        where Self: Send + Sized + 'a
    {
        PinBox::new(unsafe { UnsafePin::new(self) })
    }
}

impl<F: Future + Unpin> StableFuture for F {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(mut self: PinMut<Self>, ctx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        F::poll(&mut *self, ctx)
    }
}

pub trait StableStream {
    type Item;
    type Error;

    fn poll_next(self: PinMut<Self>, ctx: &mut task::Context) -> Poll<Option<Self::Item>, Self::Error>;

    fn pin<'a>(self) -> PinnedStream<'a, Self::Item, Self::Error>
        where Self: Sized + 'a
    {
        PinBox::new(unsafe { UnsafePin::new(self) })
    }

    fn pin_send<'a>(self) -> PinnedStreamSend<'a, Self::Item, Self::Error>
        where Self: Send + Sized + 'a
    {
        PinBox::new(unsafe { UnsafePin::new(self) })
    }
}

impl<S: Stream + Unpin> StableStream for S {
    type Item = S::Item;
    type Error = S::Error;

    fn poll_next(mut self: PinMut<Self>, ctx: &mut task::Context) -> Poll<Option<Self::Item>, Self::Error> {
        S::poll_next(&mut *self, ctx)
    }
}
