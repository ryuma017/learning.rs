use rayon::prelude::*;
use std::time::Instant;

#[tokio::main]
async fn main() {
    // std::thread
    let before = Instant::now();
    for _ in 0..50 {
        std_thread_spawn();
    }
    let elapsed = before.elapsed();
    println!(
        "[std::thread] {}.{:03} total, 0.{} avg per iteration",
        elapsed.as_secs(),
        elapsed.subsec_millis(),
        elapsed.as_millis() / 50
    );

    // rayon
    let before = Instant::now();
    for _ in 0..50 {
        rayon_spawn();
    }
    let elapsed = before.elapsed();
    println!(
        "[rayon]       {}.{:03} total, 0.{} avg per iteration",
        elapsed.as_secs(),
        elapsed.subsec_millis(),
        elapsed.as_millis() / 50
    );

    // tokio
    let before = Instant::now();
    for _ in 0..50 {
        tokio_spawn_blocking().await;
    }
    let elapsed = before.elapsed();
    println!(
        "[tokio]       {}.{:03} total, 0.{} avg per iteration",
        elapsed.as_secs(),
        elapsed.subsec_millis(),
        elapsed.as_millis() / 50
    );
}

fn std_thread_spawn() {
    let handles = (0..10)
        .into_iter()
        .map(|_| {
            std::thread::spawn(|| {
                fib(40);
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        handle.join().unwrap();
    }
    // await other async tasks ...
}

fn rayon_spawn() {
    (0..10).into_par_iter().for_each(|_| {
        fib(40);
    });
    // await other async tasks ...
}

async fn tokio_spawn_blocking() {
    let handles = (0..10)
        .map(|_| {
            tokio::task::spawn_blocking(|| {
                fib(40);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.await.unwrap();
    }
    // await other async tasks ...
}

fn fib(i: i32) -> i32 {
    if i < 2 {
        return i;
    }
    fib(i - 3) + fib(i - 1)
}
