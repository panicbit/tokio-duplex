use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;
use tokio::io::{self, AsyncRead, AsyncWrite, ReadBuf};

pin_project! {
    #[derive(Clone, Debug)]
    pub struct Duplex<R, W> {
        #[pin]
        reader: R,
        #[pin]
        writer: W,
    }
}

impl<R, W> Duplex<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
}

impl<R: AsyncRead, W> AsyncRead for Duplex<R, W> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        AsyncRead::poll_read(self.project().reader, cx, buf)
    }
}

impl<R, W: AsyncWrite> AsyncWrite for Duplex<R, W> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write(self.project().writer, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_flush(self.project().writer, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        AsyncWrite::poll_shutdown(self.project().writer, cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        AsyncWrite::poll_write_vectored(self.project().writer, cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        AsyncWrite::is_write_vectored(&self.writer)
    }
}
