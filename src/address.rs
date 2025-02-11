//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use crate::error::{Error, Result};
use std::net::{IpAddr, Ipv4Addr};
use std::net::{SocketAddr, SocketAddrV4};

/// Helper trait to convert things into IPv4 addresses.
#[allow(clippy::wrong_self_convention)]
pub trait IntoAddress {
    /// Convert the type to an `Ipv4Addr`.
    fn into_address(&self) -> Result<IpAddr>;
}

impl IntoAddress for u32 {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::V4(Ipv4Addr::new(
            ((*self) & 0xff) as u8,
            ((*self >> 8) & 0xff) as u8,
            ((*self >> 16) & 0xff) as u8,
            ((*self >> 24) & 0xff) as u8,
        )))
    }
}

impl IntoAddress for i32 {
    fn into_address(&self) -> Result<IpAddr> {
        (*self as u32).into_address()
    }
}

impl IntoAddress for (u8, u8, u8, u8) {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::V4(Ipv4Addr::new(self.0, self.1, self.2, self.3)))
    }
}

impl IntoAddress for str {
    fn into_address(&self) -> Result<IpAddr> {
        self.parse().map_err(|_| Error::InvalidAddress)
    }
}

impl<'a> IntoAddress for &'a str {
    fn into_address(&self) -> Result<IpAddr> {
        (*self).into_address()
    }
}

impl IntoAddress for String {
    fn into_address(&self) -> Result<IpAddr> {
        self.as_str().into_address()
    }
}

impl<'a> IntoAddress for &'a String {
    fn into_address(&self) -> Result<IpAddr> {
        self.as_str().into_address()
    }
}

impl IntoAddress for Ipv4Addr {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::V4(*self))
    }
}

impl<'a> IntoAddress for &'a Ipv4Addr {
    fn into_address(&self) -> Result<IpAddr> {
        (*self).into_address()
    }
}

impl IntoAddress for IpAddr {
    fn into_address(&self) -> Result<IpAddr> {
        match self {
            IpAddr::V4(value) => Ok(IpAddr::V4(*value)),

            IpAddr::V6(_) => unimplemented!(),
        }
    }
}

impl<'a> IntoAddress for &'a IpAddr {
    fn into_address(&self) -> Result<IpAddr> {
        (*self).into_address()
    }
}

impl IntoAddress for SocketAddrV4 {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::V4(*self.ip()))
    }
}

impl<'a> IntoAddress for &'a SocketAddrV4 {
    fn into_address(&self) -> Result<IpAddr> {
        (*self).into_address()
    }
}

impl IntoAddress for SocketAddr {
    fn into_address(&self) -> Result<IpAddr> {
        match self {
            SocketAddr::V4(value) => Ok(IpAddr::V4(*value.ip())),

            SocketAddr::V6(_) => unimplemented!(),
        }
    }
}

impl<'a> IntoAddress for &'a SocketAddr {
    fn into_address(&self) -> Result<IpAddr> {
        (*self).into_address()
    }
}
