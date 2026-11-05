// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.93 B1, MU-ECONOMY §5.2 — ACT_WALLET: the wallet read, the stake and
// unstake, and the compute lease (sub-kinds 3 and 4), each signed.

//! The wallet and leases.
//!
//! A wallet is a reading of the holding's coordinate: a party's holding
//! stands at a coordinate of the one multi-dimensional ledger (lane × layer
//! × grade, MU-ECONOMY §2), and the wallet reads it per court as the
//! balance, bond and holds, each a [`Mu`] — an exact non-negative rational
//! read on each lane `S:G:T:H`, never summed across lanes, retiring in
//! v0.5.0 for [`Modes`](crate::task::Modes). A whole count `n` is the rational `n/1`
//! ([`Mu::counts`]); a draw past a holding is refused naming the lane
//! ([`MuRefusal::Short`]); every μ field on the wire is the lane codec
//! [`Mu::put`], refused where a lane is negative or not in lowest terms.
//! Each act is encoded here and spoken with
//! [`Session::speak`](crate::session::Session::speak):
//!
//! - [`encode_wallet`] reads a party's [`Account`] ([`decode_wallet_reply`]);
//! - a [`StakeAct`] (stake, or unstake) is signed and sent with
//!   [`encode_stake`], answered by [`decode_stake_reply`];
//! - a [`LeaseAct`] opens a lease ([`encode_lease`]), answered by
//!   [`decode_lease_reply`]; a [`CloseLease`] closes it ([`encode_close`]).

/// An amount of μ read exactly per lane (retiring in v0.5.0 for `Modes`),
/// and why one was refused.
pub use mycelium::economy::{Mu, MuRefusal};

/// A party's account at one court, and its wallet across courts: readings
/// of its holding, never a stored balance.
pub use mycelium::anastome::dialogue::wallet::{Account, Wallet};

/// The wallet read and its reply.
pub use mycelium::anastome::dialogue::wallet::{decode_wallet_reply, encode_wallet};

/// Stake and unstake: the act, its signed form, its codec and its refusal.
pub use mycelium::anastome::dialogue::wallet::{decode_stake_reply, encode_stake, SignedStake, StakeAct, StakeRefusal};

/// The compute lease: open and close, signed, with the reply and the
/// refusal by name.
pub use mycelium::anastome::dialogue::lease::{
    decode_lease_reply, encode_close, encode_lease, CloseLease, LeaseAct, LeaseRefusal, LeaseReply, SignedClose, SignedLease,
};
