use color_eyre::Report;
use std::future::Future;

#[tokio::main]
async fn main() {
    let tup = try_join(do_stuff(), do_more_stuff()).await;
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

// TODO: return as soon as either future returns Err
async fn try_join<AR, BR, E>(
    a: impl Future<Output = Result<AR, E>>,
    b: impl Future<Output = Result<BR, E>>,
) -> Result<(AR, BR), E> {
    let a = a.await?;
    let b = b.await?;
    Ok((a, b))
}
