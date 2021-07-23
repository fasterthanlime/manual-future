use color_eyre::{eyre::eyre, Report};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[tokio::main]
async fn main() {
    println!("Ok we're off!");
    let tup = try_join_correct(Box::pin(do_more_stuff()), Box::pin(do_stuff())).await;
    println!("And we're done");
    dbg!(&tup);
}

async fn do_stuff() -> Result<u64, Report> {
    tokio::time::sleep(Duration::from_millis(20)).await;
    Err(eyre!("this don't work"))
}

async fn do_more_stuff() -> Result<String, Report> {
    tokio::time::sleep(Duration::from_secs(3)).await;
    Ok("nice number".into())
}

fn try_join_correct<AR, BR, E>(
    a: impl Future<Output = Result<AR, E>> + Unpin,
    b: impl Future<Output = Result<BR, E>> + Unpin,
) -> impl Future<Output = Result<(AR, BR), E>> {
    TryFuture { a, b }
}

struct TryFuture<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>> + Unpin,
    B: Future<Output = Result<BR, E>> + Unpin,
{
    a: A,
    b: B,
}

impl<A, B, AR, BR, E> Future for TryFuture<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>> + Unpin,
    B: Future<Output = Result<BR, E>> + Unpin,
{
    type Output = Result<(AR, BR), E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Pin<&mut T> => &mut T
        let this: &mut Self = &mut self;

        let a = Pin::new(&mut this.a);
        let b = Pin::new(&mut this.b);

        match a.poll(cx) {
            Poll::Pending => match b.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(b) => match b {
                    Err(e) => Poll::Ready(Err(e)),
                    Ok(_) => Poll::Pending,
                },
            },
            Poll::Ready(a) => match a {
                Err(e) => Poll::Ready(Err(e)),
                Ok(a) => match b.poll(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(b) => match b {
                        Err(e) => Poll::Ready(Err(e)),
                        Ok(b) => Poll::Ready(Ok((a, b))),
                    },
                },
            },
        }
    }
}
