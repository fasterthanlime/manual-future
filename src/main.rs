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

    println!("Ok we're off!");
    let tup = try_join_correct(Box::pin(do_stuff()), Box::pin(do_more_stuff())).await;
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
    a: impl Future<Output = Result<AR, E>> + Unpin,
    b: impl Future<Output = Result<BR, E>> + Unpin,
) -> impl Future<Output = Result<(AR, BR), E>>
where
    AR: Unpin,
    BR: Unpin,
{
    TryFuture {
        a: MaybeResult::Future(a),
        b: MaybeResult::Future(b),
    }
}

struct TryFuture<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>> + Unpin,
    B: Future<Output = Result<BR, E>> + Unpin,
    AR: Unpin,
    BR: Unpin,
{
    a: MaybeResult<A, AR>,
    b: MaybeResult<B, BR>,
}

enum MaybeResult<F, T> {
    Future(F),
    Result(T),
    Gone,
}

impl<F, T> MaybeResult<F, T> {
    fn as_mut(&mut self) -> MaybeResult<&mut F, &mut T> {
        match self {
            Self::Future(f) => MaybeResult::Future(f),
            Self::Result(t) => MaybeResult::Result(t),
            Self::Gone => MaybeResult::Gone,
        }
    }
}

impl<A, B, AR, BR, E> Future for TryFuture<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>> + Unpin,
    B: Future<Output = Result<BR, E>> + Unpin,
    AR: Unpin,
    BR: Unpin,
{
    type Output = Result<(AR, BR), E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Pin<&mut T> => &mut T
        let this: &mut Self = &mut self;

        match this.a.as_mut() {
            MaybeResult::Future(mut a) => {
                let a = Pin::new(&mut a);
                match a.poll(cx) {
                    Poll::Ready(a) => match a {
                        Ok(a) => this.a = MaybeResult::Result(a),
                        Err(e) => return Poll::Ready(Err(e)),
                    },
                    Poll::Pending => {}
                }
            }
            MaybeResult::Result(_) => {}
            MaybeResult::Gone => unreachable!(),
        };

        match this.b.as_mut() {
            MaybeResult::Future(mut b) => {
                let b = Pin::new(&mut b);
                match b.poll(cx) {
                    Poll::Ready(b) => match b {
                        Ok(b) => this.b = MaybeResult::Result(b),
                        Err(e) => return Poll::Ready(Err(e)),
                    },
                    Poll::Pending => {}
                }
            }
            MaybeResult::Result(_) => {}
            MaybeResult::Gone => unreachable!(),
        }

        if let (MaybeResult::Result(_), MaybeResult::Result(_)) = (this.a.as_mut(), this.b.as_mut())
        {
            let mut a = MaybeResult::Gone;
            let mut b = MaybeResult::Gone;

            std::mem::swap(&mut a, &mut this.a);
            std::mem::swap(&mut b, &mut this.b);

            let a = match a {
                MaybeResult::Result(a) => a,
                _ => unreachable!(),
            };
            let b = match b {
                MaybeResult::Result(b) => b,
                _ => unreachable!(),
            };

            Poll::Ready(Ok((a, b)))
        } else {
            Poll::Pending
        }
    }
}
