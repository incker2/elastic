//! A basic usage example.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.

#[macro_use]
extern crate json_str;
extern crate elastic;

use elastic::prelude::*;

fn main() {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let client = Client::new(RequestParams::default()).unwrap();

    // A freeform JSON request body.
    let get_query = json_fn!(|qry| {
        query: {
            query_string: {
                query: $qry
            }
        }
    });

    // A search request from the body.
    let req = SearchRequest::for_index("_all", get_query(r#""string value""#));

    // Send the request and process the response.
    let res: Response = client
        .request(req)
        .send()
        .and_then(|res| res.json())
        .unwrap();

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }
}