use color_eyre::{eyre::eyre, Report};
use std::{future::Future, pin::Pin, time::Duration};

#[tokio::main]
async fn main() {
    println!("Ok we're off!");
    let tup = try_join(do_stuff(), do_more_stuff()).await;
    println!("And we're done");
    dbg!(&tup);
}

async fn do_stuff() -> Result<u64, Report> {
    tokio::time::sleep(Duration::from_millis(20)).await;
    Err(eyre!("this don't work"))
}

async fn do_more_stuff() -> Result<String, Report> {
    tokio::time::sleep(Duration::from_secs(10)).await;
    Ok("nice number".into())
}

// TODO: return as soon as either future returns Err
async fn try_join<AR, BR, E>(
    mut a: impl Future<Output = Result<AR, E>>,
    mut b: impl Future<Output = Result<BR, E>>,
) -> Result<(AR, BR), E> {
    let mut a = unsafe { Pin::new_unchecked(&mut a) };
    let mut b = unsafe { Pin::new_unchecked(&mut b) };

    tokio::select! {
        a = a.as_mut() => {
            match a {
                Ok(a) => {
                    let b = b.await?;
                    Ok((a, b))
                },
                Err(e) => Err(e),
            }
        },
        b = b.as_mut() => {
            match b {
                Ok(b) => {
                    let a = a.await?;
                    Ok((a, b))
                },
                Err(e) => Err(e),
            }
        },
    }
}
