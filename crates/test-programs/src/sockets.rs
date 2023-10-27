use crate::wasi::clocks::monotonic_clock;
use crate::wasi::io::poll::{self, Pollable};
use crate::wasi::io::streams::{InputStream, OutputStream, StreamError};
use crate::wasi::sockets::instance_network;
use crate::wasi::sockets::network::{
    ErrorCode, IpAddress, IpAddressFamily, IpSocketAddress, Ipv4SocketAddress, Ipv6SocketAddress,
    Network,
};
use crate::wasi::sockets::tcp::TcpSocket;
use crate::wasi::sockets::udp::{
    IncomingDatagram, IncomingDatagramStream, OutgoingDatagram, OutgoingDatagramStream, UdpSocket,
};
use crate::wasi::sockets::{tcp_create_socket, udp_create_socket};
use std::ops::Range;

const TIMEOUT_NS: u64 = 1_000_000_000;

impl Pollable {
    pub fn wait(&self) {
        poll::poll_one(self);
    }

    pub fn wait_until(&self, timeout: &Pollable) -> Result<(), ErrorCode> {
        let ready = poll::poll_list(&[self, timeout]);
        assert!(ready.len() > 0);
        match ready[0] {
            0 => Ok(()),
            1 => Err(ErrorCode::Timeout),
            _ => unreachable!(),
        }
    }
}

impl OutputStream {
    pub fn blocking_write_util(&self, mut bytes: &[u8]) -> Result<(), StreamError> {
        let pollable = self.subscribe();

        while !bytes.is_empty() {
            pollable.wait();

            let permit = self.check_write()?;

            let len = bytes.len().min(permit as usize);
            let (chunk, rest) = bytes.split_at(len);

            self.write(chunk)?;

            self.blocking_flush()?;

            bytes = rest;
        }
        Ok(())
    }
}

impl Network {
    pub fn default() -> Network {
        instance_network::instance_network()
    }
}

impl TcpSocket {
    pub fn new(address_family: IpAddressFamily) -> Result<TcpSocket, ErrorCode> {
        tcp_create_socket::create_tcp_socket(address_family)
    }

    pub fn blocking_bind(
        &self,
        network: &Network,
        local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        let sub = self.subscribe();

        self.start_bind(&network, local_address)?;

        loop {
            match self.finish_bind() {
                Err(ErrorCode::WouldBlock) => sub.wait(),
                result => return result,
            }
        }
    }

    pub fn blocking_listen(&self) -> Result<(), ErrorCode> {
        let sub = self.subscribe();

        self.start_listen()?;

        loop {
            match self.finish_listen() {
                Err(ErrorCode::WouldBlock) => sub.wait(),
                result => return result,
            }
        }
    }

    pub fn blocking_connect(
        &self,
        network: &Network,
        remote_address: IpSocketAddress,
    ) -> Result<(InputStream, OutputStream), ErrorCode> {
        let sub = self.subscribe();

        self.start_connect(&network, remote_address)?;

        loop {
            match self.finish_connect() {
                Err(ErrorCode::WouldBlock) => sub.wait(),
                result => return result,
            }
        }
    }

    pub fn blocking_accept(&self) -> Result<(TcpSocket, InputStream, OutputStream), ErrorCode> {
        let sub = self.subscribe();

        loop {
            match self.accept() {
                Err(ErrorCode::WouldBlock) => sub.wait(),
                result => return result,
            }
        }
    }
}

impl UdpSocket {
    pub fn new(address_family: IpAddressFamily) -> Result<UdpSocket, ErrorCode> {
        udp_create_socket::create_udp_socket(address_family)
    }

    pub fn blocking_bind(
        &self,
        network: &Network,
        local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        let sub = self.subscribe();

        self.start_bind(&network, local_address)?;

        loop {
            match self.finish_bind() {
                Err(ErrorCode::WouldBlock) => sub.wait(),
                result => return result,
            }
        }
    }
}

impl OutgoingDatagramStream {
    fn blocking_check_send(&self, timeout: &Pollable) -> Result<u64, ErrorCode> {
        let sub = self.subscribe();

        loop {
            match self.check_send() {
                Ok(0) => sub.wait_until(timeout)?,
                result => return result,
            }
        }
    }

    pub fn blocking_send(&self, mut datagrams: &[OutgoingDatagram]) -> Result<(), ErrorCode> {
        let timeout = monotonic_clock::subscribe(TIMEOUT_NS, false);

        while !datagrams.is_empty() {
            let permit = self.blocking_check_send(&timeout)?;
            let chunk_len = datagrams.len().min(permit as usize);
            match self.send(&datagrams[..chunk_len]) {
                Ok(0) => {}
                Ok(packets_sent) => {
                    let packets_sent = packets_sent as usize;
                    datagrams = &datagrams[packets_sent..];
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }
}

impl IncomingDatagramStream {
    pub fn blocking_receive(&self, count: Range<u64>) -> Result<Vec<IncomingDatagram>, ErrorCode> {
        let timeout = monotonic_clock::subscribe(TIMEOUT_NS, false);
        let pollable = self.subscribe();
        let mut datagrams = vec![];

        loop {
            match self.receive(count.end - datagrams.len() as u64) {
                Ok(mut chunk) => {
                    datagrams.append(&mut chunk);

                    if datagrams.len() >= count.start as usize {
                        return Ok(datagrams);
                    } else {
                        pollable.wait_until(&timeout)?;
                    }
                }
                Err(err) => return Err(err),
            }
        }
    }
}

impl IpAddress {
    pub const IPV4_BROADCAST: IpAddress = IpAddress::Ipv4((255, 255, 255, 255));

    pub const IPV4_LOOPBACK: IpAddress = IpAddress::Ipv4((127, 0, 0, 1));
    pub const IPV6_LOOPBACK: IpAddress = IpAddress::Ipv6((0, 0, 0, 0, 0, 0, 0, 1));

    pub const IPV4_UNSPECIFIED: IpAddress = IpAddress::Ipv4((0, 0, 0, 0));
    pub const IPV6_UNSPECIFIED: IpAddress = IpAddress::Ipv6((0, 0, 0, 0, 0, 0, 0, 0));

    pub const IPV4_MAPPED_LOOPBACK: IpAddress =
        IpAddress::Ipv6((0, 0, 0, 0, 0, 0xFFFF, 0x7F00, 0x0001));

    pub const fn new_loopback(family: IpAddressFamily) -> IpAddress {
        match family {
            IpAddressFamily::Ipv4 => Self::IPV4_LOOPBACK,
            IpAddressFamily::Ipv6 => Self::IPV6_LOOPBACK,
        }
    }

    pub const fn new_unspecified(family: IpAddressFamily) -> IpAddress {
        match family {
            IpAddressFamily::Ipv4 => Self::IPV4_UNSPECIFIED,
            IpAddressFamily::Ipv6 => Self::IPV6_UNSPECIFIED,
        }
    }

    pub const fn family(&self) -> IpAddressFamily {
        match self {
            IpAddress::Ipv4(_) => IpAddressFamily::Ipv4,
            IpAddress::Ipv6(_) => IpAddressFamily::Ipv6,
        }
    }
}

impl PartialEq for IpAddress {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ipv4(left), Self::Ipv4(right)) => left == right,
            (Self::Ipv6(left), Self::Ipv6(right)) => left == right,
            _ => false,
        }
    }
}

impl IpSocketAddress {
    pub const fn new(ip: IpAddress, port: u16) -> IpSocketAddress {
        match ip {
            IpAddress::Ipv4(addr) => IpSocketAddress::Ipv4(Ipv4SocketAddress {
                port: port,
                address: addr,
            }),
            IpAddress::Ipv6(addr) => IpSocketAddress::Ipv6(Ipv6SocketAddress {
                port: port,
                address: addr,
                flow_info: 0,
                scope_id: 0,
            }),
        }
    }

    pub const fn ip(&self) -> IpAddress {
        match self {
            IpSocketAddress::Ipv4(addr) => IpAddress::Ipv4(addr.address),
            IpSocketAddress::Ipv6(addr) => IpAddress::Ipv6(addr.address),
        }
    }

    pub const fn port(&self) -> u16 {
        match self {
            IpSocketAddress::Ipv4(addr) => addr.port,
            IpSocketAddress::Ipv6(addr) => addr.port,
        }
    }

    pub const fn family(&self) -> IpAddressFamily {
        match self {
            IpSocketAddress::Ipv4(_) => IpAddressFamily::Ipv4,
            IpSocketAddress::Ipv6(_) => IpAddressFamily::Ipv6,
        }
    }
}

impl PartialEq for Ipv4SocketAddress {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port && self.address == other.address
    }
}

impl PartialEq for Ipv6SocketAddress {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port
            && self.flow_info == other.flow_info
            && self.address == other.address
            && self.scope_id == other.scope_id
    }
}

impl PartialEq for IpSocketAddress {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ipv4(l0), Self::Ipv4(r0)) => l0 == r0,
            (Self::Ipv6(l0), Self::Ipv6(r0)) => l0 == r0,
            _ => false,
        }
    }
}
