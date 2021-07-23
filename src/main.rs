use color_eyre::Report;
use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    let tup = try_join(Box::pin(do_stuff()), Box::pin(do_more_stuff())).await;
    dbg!(&tup);
}

async fn do_stuff() -> Result<u64, Report> {
    Ok(27)
    // Err(eyre!("this don't work"))
}

async fn do_more_stuff() -> Result<String, Report> {
    Ok("nice number".into())
    // Err(eyre!("this don't work"))
}

// TODO: return as soon as either future returns an error
async fn try_join<A, B, E>(
    a: Pin<Box<dyn Future<Output = Result<A, E>>>>,
    b: Pin<Box<dyn Future<Output = Result<B, E>>>>,
) -> Result<(A, B), E> {
    let a = a.await?;
    let b = b.await?;

    Ok((a, b))
}
