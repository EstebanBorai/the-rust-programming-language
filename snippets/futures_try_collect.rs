//! Futures Doc: https://docs.rs/futures/0.3.5/futures/stream/trait.TryStreamExt.html#method.try_collect
//! https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ea4036662a89ab52d1c892b3856241ae

use futures::channel::mpsc;
use futures::stream::TryStreamExt;
use std::thread;

#[tokio::main]
async fn main() {
  let (tx, rx) = mpsc::unbounded();

  thread::spawn(move || {
    for i in 1..=5 {
      tx.unbounded_send(Ok(i)).unwrap();
    }
    // tx.unbounded_send(Err(6)).unwrap();
  });

  let output: Result<Vec<i32>, i32> = rx.try_collect().await;

  println!("{:?}", output);
}
