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
        a,
        b,
        a_result: None,
        b_result: None,
    }
}

struct TryFuture<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>> + Unpin,
    B: Future<Output = Result<BR, E>> + Unpin,
    AR: Unpin,
    BR: Unpin,
{
    a: A,
    b: B,
    a_result: Option<AR>,
    b_result: Option<BR>,
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

        if this.a_result.is_none() {
            let a = Pin::new(&mut this.a);
            match a.poll(cx) {
                Poll::Ready(a) => match a {
                    Ok(a) => this.a_result = Some(a),
                    Err(e) => return Poll::Ready(Err(e)),
                },
                Poll::Pending => {}
            }
        }

        if this.b_result.is_none() {
            let b = Pin::new(&mut this.b);
            match b.poll(cx) {
                Poll::Ready(b) => match b {
                    Ok(b) => this.b_result = Some(b),
                    Err(e) => return Poll::Ready(Err(e)),
                },
                Poll::Pending => {}
            }
        }

        if let (Some(_), Some(_)) = (&this.a_result, &this.b_result) {
            // unwrap rationale: we just checked both `a_result` and `b_result` were `Some`
            let (a, b) = (this.a_result.take().unwrap(), this.b_result.take().unwrap());
            Poll::Ready(Ok((a, b)))
        } else {
            Poll::Pending
        }
    }
}
