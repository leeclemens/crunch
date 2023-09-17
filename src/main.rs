// The MIT License (MIT)
// Copyright © 2021 Aukbit Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod config;
mod crunch;
mod errors;
mod matrix;
mod pools;
mod report;
mod runtimes;
mod stats;
use tokio::select;
use tokio::signal;
use tokio_util::sync::CancellationToken;

use crate::config::CONFIG;
use crate::crunch::Crunch;
use log::info;
use std::env;

async fn main_interruptable() {
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    // ... spawn application as separate task ...
    let join_handle = tokio::spawn(async move {
        // Wait for either cancellation or a very long time
        tokio::select! {
            _ = cloned_token.cancelled() => {
                // The token was cancelled
                5
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(9999)) => {
                99
            }
        }
    });

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        token.cancel();
    });
    if config.is_mode_era {
        return Crunch::subscribe();
    }
    Crunch::flakes();

    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }
    // send shutdown signal to application and wait
}

async fn main() {
    let config = CONFIG.clone();
    if config.is_debug {
        env::set_var("RUST_LOG", "crunch=debug,subxt=debug");
    } else {
        env::set_var("RUST_LOG", "crunch=info");
    }
    env_logger::try_init().unwrap_or_default();

    info!(
        "{} v{} * {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );

    if config.only_view {
        return Crunch::view();
    }
    main_interruptable().await?;
}
