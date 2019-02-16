use std::{
    io as stdio,
    marker::Unpin,
    pin::Pin,
    task::{Poll, Waker},
};

use embrio_core::io as embrio;

pub(crate) struct Std<T>(pub(crate) T);

impl<T: stdio::Read + Unpin> embrio::Read for Std<T> {
    type Error = stdio::Error;

    fn poll_read(
        self: Pin<&mut Self>,
        _waker: &Waker,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>> {
        Poll::Ready(Pin::get_mut(self).0.read(buf))
    }
}

impl<T: stdio::Write + Unpin> embrio::Write for Std<T> {
    type Error = stdio::Error;

    fn poll_write(
        self: Pin<&mut Self>,
        _waker: &Waker,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::Error>> {
        Poll::Ready(Pin::get_mut(self).0.write(buf))
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        _waker: &Waker,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Pin::get_mut(self).0.flush())
    }

    fn poll_close(
        self: Pin<&mut Self>,
        waker: &Waker,
    ) -> Poll<Result<(), Self::Error>> {
        self.poll_flush(waker)
    }
}
