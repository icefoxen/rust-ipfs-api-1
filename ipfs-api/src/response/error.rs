// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use hyper;
use serde_json;
use serde_urlencoded;
use std::string::FromUtf8Error;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiError {
    pub message: String,
    pub code: u8,
}


error_chain! {
    foreign_links {
        Http(hyper::error::Error);
        Parse(serde_json::Error);
        ParseUtf8(FromUtf8Error);
        Url(hyper::error::UriError);
        Io(::std::io::Error);
        EncodeUrl(serde_urlencoded::ser::Error);
    }

    errors {
        /// An error returned by the Ipfs api.
        ///
        Api(err: ApiError) {
            description("api returned an error"),
            display("api returned '{}'", err.message)
        }

        /// A stream error indicated in the Trailer header.
        ///
        StreamError(err: String) {
            description("api returned a stream error"),
            display("api returned an error while streaming: '{}'", err)
        }

        Uncategorized(err: String) {
            description("api returned an unknown error"),
            display("api returned '{}'", err)
        }
    }
}
