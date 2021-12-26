// Copyright 2021 Developers of Pyroscope.

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0>. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate pyroscope;

use pyroscope::{PyroscopeAgent, Result};

use std::thread;
use std::time::Duration;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let handle_1 = tokio::task::spawn(async {
        let mut agent = PyroscopeAgent::builder("http://localhost:4040", "MultiThread")
            .frequency(100)
            .tags(
                &[("Thread", "Thread 1")]
            )
            .build()
            .unwrap();

        agent.start().unwrap();

        let n = fibonacci(48);
        println!("Thread 1: {}", n);

        agent.stop().await.unwrap();
    }).await.unwrap();

    let handle_2 = tokio::task::spawn(async {
        let mut agent = PyroscopeAgent::builder("http://localhost:4040", "MultiThread")
            .frequency(100)
            .tags(
                &[("Thread", "Thread 2")]
            )
            .build()
            .unwrap();

        agent.start().unwrap();

        let n = fibonacci(39);
        println!("Thread 2: {}", n);

        agent.stop().await.unwrap();
    }).await.unwrap();

    let handle_3 = tokio::task::spawn(async {
        let n = fibonacci(50);
        println!("Thread 3: {}", n);
    }).await.unwrap();

    Ok(())
}