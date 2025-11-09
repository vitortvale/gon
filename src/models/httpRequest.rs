use hyper::{Method, Request, StatusCode, Uri};

//let request = Request::builder()
//    .method("GET")
//    .uri("https://www.rust-lang.org/")
//    .header("X-Custom-Foo", "Bar")
//    .body(())
//    .unwrap();

struct httpRequest {
    method: Method
}
