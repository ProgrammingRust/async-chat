//! Asynchronous chat server.

//# server-main
use async_std::prelude::*;
use async_std::{net, task};
use async_chat::utils::ChatResult;
use std::sync::Arc;

mod connection;
mod group;
mod group_table;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");

    let groups = Arc::new(group_table::GroupTable::new());

    task::block_on(async {
        let listener = net::TcpListener::bind(address).await?;

        let mut new_connections = listener.incoming();
        loop {
            let socket = new_connections.next().await.unwrap()?;
            let groups = groups.clone();
            task::spawn(async {
                if let Err(error) = connection::serve(socket, groups).await {
                    eprintln!("Error: {}", error);
                }
            });
        }
    })
}
//# end
