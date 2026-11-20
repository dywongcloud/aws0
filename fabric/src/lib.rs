// SPDX-License-Identifier: CC0-1.0

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! aws0 -- a true 1:1 AWS. Local-first. Horizontally scalable.
//!
//! Built to give you the democracy to run your own cloud. One command gives
//! you an AWS-shaped endpoint on your own machine: real databases and
//! containers, isolated regions, state that survives restarts, and capacity
//! that heals itself. Add machines to add capacity; no control plane to
//! operate, no account, no subscription, no vendor.
//!
//! Every transition is a pure morphism: it takes a state value and returns a
//! new state value plus an explicit list of effects. Effects are executed at
//! exactly one boundary. This crate uses no `unsafe`, no unchecked panics, no
//! ambient globals and no hidden interior mutation behind an immutable
//! reference.

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
