extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;


mod utils;


fn main() {
    use std::env;
    let url = env::args().skip(1).take(1).next().unwrap();

    // pass a handler function for "hyper::Chunk" to http_get

    //  *hyper::Chunk ->  [u8]
    // &*hyper::Chunk -> &[u8]

    // support both "https://" and "http://"
    let data: String =
      utils::https_get(&url,
                       |body| Ok(String::from_utf8_lossy(&*body).to_ascii_lowercase()));
    println!("{}", data);

    // support only "http://"
    let data: String =
      utils::http_get(&url,
                      |body| Ok(String::from_utf8_lossy(&*body).to_ascii_lowercase()));
    println!("{}", data);

    // you can use serde to handle the result and return a custom type
    // let data: CustomStruct =
    //   http_get(&url, |body| Ok(serde_json::from_slice(&body).unwrap()));
}
