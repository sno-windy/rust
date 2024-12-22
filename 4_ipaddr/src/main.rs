// oreore IP impl.
// @see https://github.com/rust-lang/rust/blob/master/library/core/src/net/ip_addr.rs

fn main() {
    let v4 = IpAddr::V4(Ipv4Addr::new((127, 0, 0, 1)));
    let v6 = IpAddr::V6(Ipv6Addr::new((127, 0, 0, 0, 0, 0, 0, 1)));
    println!("{:?}", v4.is_v6());
    println!("{:?}", v6);
    println!("{:?}", v4.octets());
    println!("{:?}", v6.octets());
    println!("{:?}", v4.is_v4());
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
struct Ipv4Addr (u8, u8, u8, u8);
#[derive(Debug)]
struct Ipv6Addr (u8, u8, u8, u8, u8, u8, u8, u8);

impl IpAddr {
    fn is_v4(&self) -> bool {
        match *self {
            Self::V4(_) => true,
            _ => false,
        }
    }

    fn is_v6(&self) -> bool {
        match *self {
            Self::V6(_) => true,
            _ => false,
        }
    }

    fn octets(&self) -> Vec<u8> {
        match self {
            Self::V4(v4) => Ipv4Addr::octets(&v4),
            Self::V6(v6) => Ipv6Addr::octets(&v6),
        }
    }
}

impl Ipv4Addr {
    fn new (octet: (u8, u8, u8, u8)) -> Self {
        Self(octet.0, octet.1, octet.2, octet.3)
    }

    fn octets(&self) -> Vec<u8> {
        vec![self.0, self.1, self.2, self.3]
    }
}

impl Ipv6Addr {
    fn new (octet: (u8, u8, u8, u8, u8, u8, u8, u8)) -> Self {
        Self(octet.0, octet.1, octet.2, octet.3, octet.4, octet.5, octet.6, octet.7)
    }

    fn octets(&self) -> Vec<u8> {
        vec![self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7]
    }
}
