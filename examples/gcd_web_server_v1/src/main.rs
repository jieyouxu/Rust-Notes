use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

fn main() {
    const PORT: u16 = 8080;
    // Unfortunately `SocketAddr::new` and `Ipv4Addr::new` are not yet
    // `const fn`s so we can't make them compile-time constants, because
    // `match` is not yet compile time `const`.
    // See PR #67315: https://github.com/rust-lang/rust/pull/67315
    let address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT);

    let mut router = Router::new();
    router.get("/", get_form, "home");
    router.post("/gcd", post_gcd, "gcd");

    println!("Server listening on http://{}...", address);
    Iron::new(router).http(address).unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let content = r#"
		<title>GCD</title>
		<form action="/gcd" method="post">
			<input type="text" name="n" />
			<input type="text" name="n" />
			<button type="submit">Compute</button>
		</form>
	"#;

    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut("text/html; charset=utf-8".parse::<Mime>().unwrap());
    response.set_mut(content);

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("failed to parse {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };

    let raw_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut("invalid form");
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();

    for raw in raw_numbers {
        match u64::from_str(&raw) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("NaN: {:?} is not a number\n", raw));
                return Ok(response);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut("text/html; charset=utf-8".parse::<Mime>().unwrap());
    response.set_mut(format!("GCD of {:?} is <b>{}</b>\n", numbers, d));

    Ok(response)
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}
