#![feature(option_filter)]
#![allow(unknown_lints)]
#[cfg(test)]
#[macro_use]
extern crate assert_matches;
extern crate capnp;
#[macro_use]
extern crate futures;
extern crate futures_timer;
#[macro_use]
extern crate log;
extern crate rand;
extern crate tokio_core;

mod algo;
mod state;
pub mod messages;
mod multipaxos;
mod net;
mod register;
mod config;
mod timer;

pub use multipaxos::{MultiPaxos, ReplicatedState};
pub use net::{UdpClient, UdpServer};
pub use register::Register;
pub use config::{Configuration, PeerIntoIter, PeerIter};
pub use timer::{FuturesScheduler, Scheduler};
pub use algo::NodeId;

/// An instance is a _round_ of the Paxos algorithm. Instances are chained to
/// form a sequence of values. Once an instance receives consensus, the next
/// instance is started.
///
/// In some implementations, this is also called a _slot_.
pub type Instance = u64;

#[allow(dead_code, clippy)]
mod messages_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/messages_capnp.rs"));
}
