////////////////////
// 3rd party
////////////////////

use hyper;


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
