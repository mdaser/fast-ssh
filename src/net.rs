//
// Copyright (C) 2026 by Martin Daser
//

use std::net::ToSocketAddrs;
use std::time::Duration;

fn ping(host: &str) -> String {
    let mut socket_addr = match format!("{}:0", host).to_socket_addrs() {
        Ok(a) => a,
        _ => return format!("Invalid address '{}'", host),
    };

    match socket_addr.next() {
        Some(target) => {
            let custom_payload = [
                35, 68, 101, 114, 83, 99, 104, 119, 97, 114, 122, 101, 87, 101, 103, 101, 108, 97,
                103, 101, 114, 101, 114, 33,
            ];

            match ping::new(target.ip())
                .timeout(Duration::from_secs(2))
                .ttl(64)
                .payload(&custom_payload)
                .send()
            {
                Ok(_) => format!("{} is reachable.", host),
                Err(e) => format!("Ping failed: {}", e),
            }
        }
        _ => format!("no IP for {}", host),
    }
}

#[cfg(test)]
mod test {
    use crate::net::ping;

    #[test]
    fn ping_00_google() {
        assert_eq!(
            ping("www.google.com"),
            String::from("www.google.com is reachable.")
        );
    }

    #[test]
    fn ping_01_fritzbox() {
        assert_eq!(ping("fritz.box"), String::from("fritz.box is reachable."));
    }

    // site dependent test; skip
    // #[test]
    // fn ping_02_maibroker() {
    //     assert_eq!(
    //         ping("maibroker.fritz.box"),
    //         String::from("maibroker.fritz.box is reachable.")
    //     );
    // }

    #[test]
    fn ping_50_invalid_goggle() {
        assert_eq!(
            ping("www.goggle.com"),
            String::from("Invalid address 'www.goggle.com'")
        );
    }

    #[test]
    fn ping_51_invalid_iobroker() {
        assert_eq!(
            ping("iobroker.fritz.box"),
            String::from("Invalid address 'iobroker.fritz.box'")
        );
    }
}
