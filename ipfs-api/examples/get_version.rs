// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

// Creates an Ipfs client, and gets the version of the Ipfs server.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());
    let req = client.version();
    let version = core.run(req).expect("expected a valid response");

    println!("version: {:?}", version.version);
}
