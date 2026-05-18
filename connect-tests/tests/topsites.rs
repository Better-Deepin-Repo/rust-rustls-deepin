// These tests check we can handshake with a selection of
// common hosts.
//
// Rules: only hosts that can really handle the traffic.

#[allow(dead_code)]
mod common;

mod online {
    use super::common::TlsClient;

    fn check(hostname: &str) {
        TlsClient::new(hostname)
            .expect("HTTP/1.[01] ")
            .go()
            .unwrap()
    }

    #[test]
    #[ignore]
    fn joe() {
        check("jbp.io")
    }

    #[test]
    #[ignore]
    fn google() {
        check("google.com")
    }

    #[test]
    #[ignore]
    fn github() {
        check("github.com")
    }

    #[test]
    #[ignore]
    fn aws() {
        check("aws.amazon.com")
    }

    #[test]
    #[ignore]
    fn microsoft() {
        check("www.microsoft.com")
    }

    #[test]
    #[ignore]
    fn wikipedia() {
        check("www.wikipedia.org")
    }

    #[test]
    #[ignore]
    fn twitter() {
        check("twitter.com")
    }

    #[test]
    #[ignore]
    fn facebook() {
        check("www.facebook.com")
    }

    #[test]
    #[ignore]
    fn baidu() {
        check("www.baidu.com")
    }

    #[test]
    #[ignore]
    fn netflix() {
        check("www.netflix.com")
    }

    #[test]
    #[ignore]
    fn stackoverflow() {
        check("stackoverflow.com")
    }

    #[test]
    #[ignore]
    fn apple() {
        check("www.apple.com")
    }

    #[test]
    fn cloudflare_1_1_1_1_dns() {
        check("1.1.1.1")
    }
}
