use color_eyre::Report;
use std::future::Future;

#[tokio::main]
async fn main() {
    let tup = try_join(&do_stuff(), &do_more_stuff()).await;
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

async fn try_join<A, B, E>(
    f1: &dyn Future<Output = Result<A, E>>,
    f2: &dyn Future<Output = Result<B, E>>,
) -> Result<(A, B), E> {
    todo!()
}
