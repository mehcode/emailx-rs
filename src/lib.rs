extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate failure;
extern crate futures;
extern crate trust_dns;

use failure::Error;
use futures::{future, prelude::*};
use regex::Regex;
use trust_dns::client::ClientHandle;
use trust_dns::client::{ClientFuture};
use trust_dns::rr::{DNSClass, Name, RecordType};
use trust_dns::udp::UdpClientStream;

lazy_static! {
    // Use -very- simple regex
    // https://davidcel.is/posts/stop-validating-email-addresses-with-regex/
    static ref EMAIL_RE: Regex = Regex::new(r".+@.+\..+").unwrap();
}

pub fn validate(email: &str) -> Box<Future<Item = bool, Error = Error>> {
    if !validate_re(email) {
        // Failed regex validation
        return Box::new(future::ok(false));
    }

    // Forward to MX record validation
    Box::new(validate_mx(email))
}

pub fn validate_re(email: &str) -> bool {
    EMAIL_RE.is_match(email)
}

pub fn validate_mx(email: &str) -> impl Future<Item = bool, Error = Error> {
    // TODO: Fail rather than panic for a badly formatted email
    let domain = format!("{}.", email.split('@').nth(1).unwrap());

    // TODO: Less concrete way to pick the DNS server
    let address = "1.1.1.1:53".parse().unwrap();
    let (stream, sender) = UdpClientStream::new(address);

    ClientFuture::new(stream, sender, None)
        .from_err()
        .and_then(move |mut client| {
            let name: Name = domain.parse().unwrap();

            client
                .query(name, DNSClass::IN, RecordType::MX)
                .from_err()
                .map(|response| {
                    // TODO: Do we want to do something _with_ the answers ?
                    // println!("{:#?}", response.answers());

                    // As long as we have at least 1 record
                    response.answers().len() > 0
                })
        })
}

#[cfg(test)]
mod tests {
    extern crate tokio;

    use self::tokio::runtime::Runtime;
    use super::{validate_re, validate_mx};

    #[test]
    fn test_validate_re() {
        assert!(validate_re("user@example.com"));
        assert!(!validate_re("user"));
        assert!(!validate_re("user@example"));
    }

    #[test]
    fn test_validate_mx() {
        let mut rt = Runtime::new().unwrap();

        assert!(!rt.block_on(validate_mx("user@example.com")).unwrap());

        assert!(rt.block_on(validate_mx("user@gmail.com")).unwrap());
        assert!(rt.block_on(validate_mx("user@github.com")).unwrap());
        assert!(rt.block_on(validate_mx("user@nwytg.net")).unwrap());
    }
}
