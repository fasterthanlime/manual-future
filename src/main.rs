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

// TODO: return as soon as either future returns an error
async fn try_join<A, B, AR, BR, E>(a: A, b: B) -> Result<(AR, BR), E>
where
    A: Future<Output = Result<AR, E>>,
    B: Future<Output = Result<BR, E>>,
{
    let a = a.await?;
    let b = b.await?;

    Ok((a, b))
}
