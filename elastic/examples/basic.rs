//! A basic usage example.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.

#[macro_use]
extern crate serde_json;
extern crate elastic;

use serde_json::Value;
use elastic::prelude::*;

fn main() {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let client = Client::new(RequestParams::default()).unwrap();

    // A search request with a freeform body.
    let req = {
        let query = json!({
            "query": {
                "query_string": {
                    "query": "*"
                }
            }
        });

        SearchRequest::for_index("_all", query.to_string())
    };

    // Send the request and process the response.
    let res = client.search::<Value>()
                    .request(req)
                    .send()
                    .unwrap();

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }

    println!("{:?}", res);
}