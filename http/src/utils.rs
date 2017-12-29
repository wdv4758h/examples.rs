////////////////////
// 3rd party
////////////////////

use hyper;
use hyper_tls;


////////////////////////////////////////
// utils
////////////////////////////////////////

pub fn http_get<T, F>(url: &str, func: F) -> T
  where F: FnOnce(hyper::Chunk) -> Result<T, hyper::Error> {
    use futures::{Future, Stream};
    use hyper::Client;
    use tokio_core::reactor::Core;

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = url.parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(func)
    });
    core.run(work).unwrap()
}

pub fn https_get<T, F>(url: &str, func: F) -> T
  where F: FnOnce(hyper::Chunk) -> Result<T, hyper::Error> {
    use futures::{Future, Stream};
    use hyper::Client;
    use tokio_core::reactor::Core;

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let uri = url.parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(func)
    });
    core.run(work).unwrap()
}
