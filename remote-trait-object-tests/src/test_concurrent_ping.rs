// Copyright 2020 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::connection::{create_connection, ConnectionEnd};
use cbasesandbox::ipc::intra::Intra;
use remote_trait_object::Context;
use remote_trait_object::{Packet};
use std::thread;
use std::time::Duration;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

/// There are thee entities: commander, main module and ping module.
/// Commander sends "start" message to the main module.
/// If the main module receives "start" message, it sends "ping" to the ping module.
/// If the ping module receives "ping" message, respond "pong".
/// If the main module received "pong" response, send "pong received" to the commander.
#[test]
fn ping() {
    init_logger();

    debug!("ping test start");
    let (cmd_to_ping, ping_to_cmd) = create_connection::<Intra>();

    let _ping_module = create_ping_module(ping_to_cmd);

    let ConnectionEnd {
        sender: to_ping,
        receiver: from_ping,
    } = cmd_to_ping;

    debug!("Send start cmd");

    let cmd_to_ping_rto = Context::new(to_ping, from_ping);
    let mut handles = Vec::new();
    for i in 0..2 {
        let port = cmd_to_ping_rto.get_port().upgrade().unwrap();

        let joiner = thread::Builder::new()
            .name(format!("ping sender {}", i))
            .spawn(move || {
                let request = Packet::new_request("ping".to_string(), 1, &[]);
                let response = port.call(request.view());
                assert_eq!(response.data(), b"pong");
            })
            .unwrap();
        handles.push(joiner);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn create_ping_module(connection: ConnectionEnd<Intra>) -> Context {
    let ConnectionEnd {
        sender: to_cmd,
        receiver: from_cmd,
    } = connection;

    let cmd_rto = Context::new(to_cmd, from_cmd);
    let port = cmd_rto.get_port().upgrade().unwrap();
    port.register(
        "ping".to_string(),
        Box::new(|_method: u32, _args: &[u8]| {
            thread::sleep(Duration::from_secs(1));
            b"pong".to_vec()
        }),
    );

    cmd_rto
}
