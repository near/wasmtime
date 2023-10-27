use crate::preview2::poll::Subscribe;
use crate::preview2::with_ambient_tokio_runtime;
use async_trait::async_trait;
use cap_net_ext::{AddressFamily, Blocking, UdpSocketExt};
use io_lifetimes::raw::{FromRawSocketlike, IntoRawSocketlike};
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

/// The state of a UDP socket.
///
/// This represents the various states a socket can be in during the
/// activities of binding, and connecting.
pub(crate) enum UdpState {
    /// The initial state for a newly-created socket.
    Default,

    /// Binding started via `start_bind`.
    BindStarted,

    /// Binding finished via `finish_bind`. The socket has an address but
    /// is not yet listening for connections.
    Bound,

    /// The socket is "connected" to a peer address.
    Connected,
}

/// A host UDP socket, plus associated bookkeeping.
///
/// The inner state is wrapped in an Arc because the same underlying socket is
/// used for implementing the stream types.
pub struct UdpSocket {
    /// The part of a `UdpSocket` which is reference-counted so that we
    /// can pass it to async tasks.
    pub(crate) inner: Arc<tokio::net::UdpSocket>,

    /// The current state in the bind/connect progression.
    pub(crate) udp_state: UdpState,

    /// Socket address family.
    pub(crate) family: AddressFamily,
}

#[async_trait]
impl Subscribe for UdpSocket {
    async fn ready(&mut self) {
        // None of the socket-level operations block natively
    }
}

impl UdpSocket {
    /// Create a new socket in the given family.
    pub fn new(family: AddressFamily) -> io::Result<Self> {
        Ok(UdpSocket {
            inner: Arc::new(Self::new_tokio_socket(family)?),
            udp_state: UdpState::Default,
            family,
        })
    }

    fn new_tokio_socket(family: AddressFamily) -> io::Result<tokio::net::UdpSocket> {
        // Create a new host socket and set it to non-blocking, which is needed
        // by our async implementation.
        let cap_std_socket = cap_std::net::UdpSocket::new(family, Blocking::No)?;
        let fd = cap_std_socket.into_raw_socketlike();
        let std_socket = unsafe { std::net::UdpSocket::from_raw_socketlike(fd) };
        let tokio_socket =
            with_ambient_tokio_runtime(|| tokio::net::UdpSocket::try_from(std_socket))?;

        Ok(tokio_socket)
    }

    pub fn udp_socket(&self) -> &tokio::net::UdpSocket {
        &self.inner
    }
}

pub struct IncomingDatagramStream {
    pub(crate) inner: Arc<tokio::net::UdpSocket>,

    /// If this has a value, the stream is "connected".
    pub(crate) remote_address: Option<SocketAddr>,
}

pub struct OutgoingDatagramStream {
    pub(crate) inner: Arc<tokio::net::UdpSocket>,

    /// If this has a value, the stream is "connected".
    pub(crate) remote_address: Option<SocketAddr>,

    pub(crate) send_state: SendState,
}

pub(crate) enum SendState {
    /// Waiting for the API consumer to call `check-send`.
    Idle,

    /// Ready to send up to x datagrams.
    Permitted(usize),

    /// Waiting for the OS.
    Waiting,
}
