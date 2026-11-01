// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.93 B1, MU-ECONOMY §5.2 — ACT_WALLET: the wallet read, the stake and
// unstake, and the compute lease (sub-kinds 3 and 4), each signed.

//! The wallet and leases.
//!
//! A wallet is flat by design: per court, a party's balance, bond and holds,
//! each a [`Mu`] — an exact non-negative rational on each lane `S:G:T:H`,
//! never summed across lanes. A whole count `n` is the rational `n/1`
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

/// An amount of μ, exact per lane, and why one was refused.
pub use mycelium::economy::{Mu, MuRefusal};

/// A party's account at one court, and its wallet across courts.
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
