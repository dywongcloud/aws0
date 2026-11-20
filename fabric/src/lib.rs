// SPDX-License-Identifier: CC0-1.0

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! `aws0-fabric` -- the self-hosted substrate behind `dywongcloud/aws0`.
//!
//! A user runs one command and gets one AWS-shaped endpoint. Nodes discover each
//! other over iroh QUIC, replicate an ADT state with CRDT joins, detect and heal
//! failures, schedule workloads onto nodes, run those workloads in laboon
//! sandboxes, and recover state across restarts. No Kubernetes, and no P2P
//! concept is exposed by [`facade`].
//!
//! # Structure (strict DAG, no cycles)
//!
//! ```text
//! types --> crdt --> state --> membership --> scheduler --> routing
//!                     |                          |             |
//!                     +--> persistence          +--> smartops -+--> net
//!                                                              |
//!                          runtime ----------------------------+--> facade
//! ```
//!
//! Pure transitions live in each module's `step` functions: they take `&State`,
//! return a new `State` plus a `Vec<Effect>`. [`Effect`] is the only IO surface;
//! the executor in [`net`] is the single place effects become side effects.
//! Nothing in this crate uses `unsafe`, `unwrap`, `expect`, `panic!`, ambient
//! globals, or hidden interior mutation behind an immutable reference.

pub mod crdt;
pub mod facade;
pub mod membership;
pub mod net;
pub mod persistence;
pub mod routing;
pub mod runtime;
pub mod scheduler;
pub mod smartops;
pub mod state;
pub mod types;

pub use facade::{Fabric, FabricConfig, Health};
pub use smartops::{SmartOp, SmartReply};
pub use types::{AccountId, Error, NodeId, Region, Result, WorkloadId};
