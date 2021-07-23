use color_eyre::{eyre::eyre, Report};
use std::{future::Future, time::Duration};

#[tokio::main]
async fn main() {
    println!("Ok we're off!");
    let tup = try_join_correct(do_more_stuff(), do_stuff()).await;
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

// TODO: return as soon as either future returns Err
async fn try_join<AR, BR, E>(
    a: impl Future<Output = Result<AR, E>>,
    b: impl Future<Output = Result<BR, E>>,
) -> Result<(AR, BR), E> {
    let a = a.await?;
    let b = b.await?;

    Ok((a, b))
}

fn try_join_correct<AR, BR, E>(
    a: impl Future<Output = Result<AR, E>>,
    b: impl Future<Output = Result<BR, E>>,
) -> impl Future<Output = Result<(AR, BR), E>> {
    try_join(a, b)
}
