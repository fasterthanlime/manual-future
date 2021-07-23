mod maybe_result;

use color_eyre::{eyre::eyre, Report};
use maybe_result::MaybeResult;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[tokio::main]
async fn main() {
    println!("Ok we're off!");
    let tup = try_join_correct(do_more_stuff(), do_stuff()).await;
    println!("And we're done");
    dbg!(&tup);

    println!("Ok we're off!");
    let tup = try_join_correct(do_stuff(), do_more_stuff()).await;
    println!("And we're done");
    dbg!(&tup);
}

async fn do_more_stuff() -> Result<String, Report> {
    tokio::time::sleep(Duration::from_secs(3)).await;
    Ok("nice number".into())
}

async fn do_stuff() -> Result<u64, Report> {
    tokio::time::sleep(Duration::from_millis(20)).await;
    Err(eyre!("this don't work"))
}

fn try_join_correct<AR, BR, E>(
    a: impl Future<Output = Result<AR, E>>,
    b: impl Future<Output = Result<BR, E>>,
) -> impl Future<Output = Result<(AR, BR), E>> {
    TryFuture {
        a: MaybeResult::Future(a),
        b: MaybeResult::Future(b),
    }
}

#[pin_project::pin_project]
struct TryFuture<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>>,
    B: Future<Output = Result<BR, E>>,
{
    #[pin]
    a: MaybeResult<A, AR>,

    #[pin]
    b: MaybeResult<B, BR>,
}

impl<A, B, AR, BR, E> Future for TryFuture<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>>,
    B: Future<Output = Result<BR, E>>,
{
    type Output = Result<(AR, BR), E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();

        match this.a.as_mut().project() {
            MaybeResult::Future(a) => match a.poll(cx) {
                Poll::Ready(a) => match a {
                    Ok(a) => this.a.as_mut().set_result(a),
                    Err(e) => return Poll::Ready(Err(e)),
                },
                Poll::Pending => {}
            },
            MaybeResult::Result(_) => {}
            MaybeResult::Gone => unreachable!(),
        }

        match this.b.as_mut().project() {
            MaybeResult::Future(b) => match b.poll(cx) {
                Poll::Ready(b) => match b {
                    Ok(b) => this.b.as_mut().set_result(b),
                    Err(e) => return Poll::Ready(Err(e)),
                },
                Poll::Pending => {}
            },
            MaybeResult::Result(_) => {}
            MaybeResult::Gone => unreachable!(),
        }

        if let (true, true) = (this.a.as_mut().is_result(), this.b.as_mut().is_result()) {
            let a = this.a.take_result();
            let b = this.b.take_result();
            Poll::Ready(Ok((a, b)))
        } else {
            Poll::Pending
        }
    }
}
