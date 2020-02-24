extern crate iron;
#[macro_use] extern crate mime;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n" />
            <input type="text" name="n" />
            <button type="submit">Compute GCD</button>
        </form>
    "#);

    // 最後の式で、暗黙的にreturnされる返り値となる
    Ok(response)
}
