// 78.145 — ACT_SEAT: THE SEAT SURFACE. A game's house (its domain's sitting
// reserve) lists the seats it offers; a mind takes one by its own key; the
// house answers the take, perceives for the mind and answers each call the
// mind makes with its outcome. The hypha is a general signed relay between
// them: it holds and forwards, and judges nothing in-game.
//
//! THE WIRE, agreed with the kernels (binding), v1.2 (78.146, lexicon 4):
//! ARG tag 6 Bytes; a task is named by its 32-byte citation, so a percept's
//! kind-4 record is `[Key(citation)]`. Integers little-endian; a key is 32
//! bytes; a signature is ed25519, 64 bytes.
//!
//! A frame is `[0x53][sub u8][fields][sig 64]`. The signature is over
//! `b"mycelium seat v1" ‖ [sub] ‖ [fields]` — the act tag is not signed. A
//! CALL's reference is the 32-byte citation hash
//! (`chronometry::payload_hash`, blake3) of exactly those signed bytes: a
//! citation of the call, never its identity (law 2) — the call is its fields.
//!
//! ARG = `[tag u8]` then `0 Q [num i64][den u64>0]` | `1 Key [32]` | `2 Count
//! [u64]` | `3 Flag [u8 0|1]` | `4 Mv` (`Mv::to_bytes`) | `5 BigQ [num len
//! u16][num signed LE][den len u16][den LE]`, den > 0, reduced, each length the
//! minimal one | `6 Bytes [len u16][bytes]` (78.146: a run the house names
//! whole — a task's content address, a rule's tape). ARGS = `[count u8][ARG…]`.
//!
//! Sub-kinds: 0 SEATS, 1 TAKE, 2 TAKEN, 3 PERCEPT, 4 CALL, 5 OUTCOME (posts,
//! each answered `[1][seq u64]` or `[0][reason u8]`), 6 READ-LISTING
//! (unsigned, `[domain]`, answered the latest SEATS fields‖sig or nothing),
//! 7 READ-MAIL (signed by the reader, `[reader][after u64][nonce 32]`,
//! answered `[count u16]` then `(seq u64, sub u8, len u32, fields‖sig)…`, or
//! a two-byte refusal `[0][reason ≥ 1]` — a mail reply of two bytes is
//! `[0][0]`, none held).

use super::*;
use crate::transducer::cga::Mv;
use num_bigint::{BigInt, BigUint};

/// The signing domain of every seat signature.
pub const SIGNED: &[u8] = b"mycelium seat v1";

pub const SUB_SEATS: u8 = 0;
pub const SUB_TAKE: u8 = 1;
pub const SUB_TAKEN: u8 = 2;
pub const SUB_PERCEPT: u8 = 3;
pub const SUB_CALL: u8 = 4;
pub const SUB_OUTCOME: u8 = 5;
pub const SUB_READ_LISTING: u8 = 6;
pub const SUB_READ_MAIL: u8 = 7;

/// A record kind: the house's counsel to the mind — the call the in-game
/// mind would make, with its worth and cost.
pub const KIND_COUNSEL: u16 = 0xFFFF;

/// 78.148 — A HOUSE'S REASONS, as its TAKEN and OUTCOME carry them (the
/// house's own wire; zero is no reason). NOT THE HOLDER: the mind no longer
/// holds the seat it calls from (a house restarted, the seat passed).
pub const REASON_NOT_THE_HOLDER: u16 = 3;
/// The seat offers no such function (the listing moved).
pub const REASON_NO_SUCH_FUNCTION: u16 = 4;
/// The function exists and its arguments are outside its domain.
pub const REASON_BAD_ARGUMENTS: u16 = 5;
/// A valid call the house did not apply this beat.
pub const REASON_NOT_APPLIED: u16 = 6;

const TAG_Q: u8 = 0;
const TAG_KEY: u8 = 1;
/// A Count's tag, as a listing's FN names its arguments (78.148: a
/// function whose every argument is a Count or a Flag is the mind's to
/// explore).
pub const TAG_COUNT: u8 = 2;
pub const TAG_FLAG: u8 = 3;
const TAG_MV: u8 = 4;
const TAG_BIGQ: u8 = 5;
const TAG_BYTES: u8 = 6;

/// ONE ARGUMENT, exact: a rational, a key, a count, a flag, a multivector of
/// the algebra, a rational past 64 bits, or a run of bytes named whole (at
/// most `u16::MAX` of them — the wall, never widened).
#[derive(Clone, Debug)]
pub enum Arg {
    Q { num: i64, den: u64 },
    Key([u8; 32]),
    Count(u64),
    Flag(bool),
    Mv(Mv),
    BigQ { num: BigInt, den: BigUint },
    Bytes(Vec<u8>),
}

impl PartialEq for Arg {
    /// Two arguments are one where they write one run of bytes.
    fn eq(&self, other: &Arg) -> bool {
        let (mut a, mut b) = (Vec::new(), Vec::new());
        self.put(&mut a) == Some(()) && other.put(&mut b) == Some(()) && a == b
    }
}

impl Arg {
    /// The argument's tag.
    pub fn tag(&self) -> u8 {
        match self {
            Arg::Q { .. } => TAG_Q,
            Arg::Key(_) => TAG_KEY,
            Arg::Count(_) => TAG_COUNT,
            Arg::Flag(_) => TAG_FLAG,
            Arg::Mv(_) => TAG_MV,
            Arg::BigQ { .. } => TAG_BIGQ,
            Arg::Bytes(_) => TAG_BYTES,
        }
    }

    /// A rational past 64 bits, reduced here: `None` where `den` is zero.
    pub fn big_q(num: BigInt, den: BigUint) -> Option<Arg> {
        if den == BigUint::from(0u8) {
            return None;
        }
        let r = num_rational::BigRational::new(num, BigInt::from(den));
        let den = r.denom().to_biguint()?;
        Some(Arg::BigQ { num: r.numer().clone(), den })
    }

    /// The argument written out; `None` where it cannot be (a zero or
    /// unreduced denominator, a length past its u16).
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.push(self.tag());
        match self {
            Arg::Q { num, den } => {
                if *den == 0 {
                    return None;
                }
                out.extend_from_slice(&num.to_le_bytes());
                out.extend_from_slice(&den.to_le_bytes());
            }
            Arg::Key(k) => out.extend_from_slice(k),
            Arg::Count(n) => out.extend_from_slice(&n.to_le_bytes()),
            Arg::Flag(f) => out.push(u8::from(*f)),
            Arg::Mv(m) => m.to_bytes(out),
            Arg::BigQ { num, den } => {
                if !reduced(num, den) {
                    return None;
                }
                let n = num.to_signed_bytes_le();
                let d = den.to_bytes_le();
                out.extend_from_slice(&u16::try_from(n.len()).ok()?.to_le_bytes());
                out.extend_from_slice(&n);
                out.extend_from_slice(&u16::try_from(d.len()).ok()?.to_le_bytes());
                out.extend_from_slice(&d);
            }
            Arg::Bytes(b) => {
                out.extend_from_slice(&u16::try_from(b.len()).ok()?.to_le_bytes());
                out.extend_from_slice(b);
            }
        }
        Some(())
    }

    /// The argument read whole, or none: a zero denominator, an unreduced or
    /// non-minimal BigQ, a flag other than 0 or 1, an unknown tag.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Arg> {
        let tag = u8_at(b, p)?;
        Some(match tag {
            TAG_Q => {
                let num = i64::from_le_bytes(bytes::<8>(b, p)?);
                let den = u64_at(b, p)?;
                if den == 0 {
                    return None;
                }
                Arg::Q { num, den }
            }
            TAG_KEY => Arg::Key(bytes::<32>(b, p)?),
            TAG_COUNT => Arg::Count(u64_at(b, p)?),
            TAG_FLAG => match u8_at(b, p)? {
                0 => Arg::Flag(false),
                1 => Arg::Flag(true),
                _ => return None,
            },
            TAG_MV => Arg::Mv(Mv::from_bytes(b, p)?),
            TAG_BIGQ => {
                let nl = usize::from(u16_at(b, p)?);
                let nb = b.get(*p..*p + nl)?;
                *p += nl;
                let dl = usize::from(u16_at(b, p)?);
                let db = b.get(*p..*p + dl)?;
                *p += dl;
                let num = BigInt::from_signed_bytes_le(nb);
                let den = BigUint::from_bytes_le(db);
                // Minimal lengths only: the value writes back as it was read.
                if num.to_signed_bytes_le() != nb || den.to_bytes_le() != db || !reduced(&num, &den) {
                    return None;
                }
                Arg::BigQ { num, den }
            }
            TAG_BYTES => {
                let n = usize::from(u16_at(b, p)?);
                let run = b.get(*p..*p + n)?.to_vec();
                *p += n;
                Arg::Bytes(run)
            }
            _ => return None,
        })
    }
}

/// `den > 0` and `gcd(num, den) = 1`.
fn reduced(num: &BigInt, den: &BigUint) -> bool {
    use num_integer::Integer;
    *den != BigUint::from(0u8) && num.magnitude().gcd(den) == BigUint::from(1u8)
}

/// ARGS: `[count u8][ARG…]`.
pub fn put_args(out: &mut Vec<u8>, args: &[Arg]) -> Option<()> {
    out.push(u8::try_from(args.len()).ok()?);
    args.iter().try_for_each(|a| a.put(out))
}

pub fn take_args(b: &[u8], p: &mut usize) -> Option<Vec<Arg>> {
    let n = u8_at(b, p)?;
    (0..n).map(|_| Arg::take(b, p)).collect()
}

/// A SEAT'S ROLE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Progenitor,
    /// The house's own seat.
    Ancients,
    Steward,
    Sovereign,
    /// An office, 4 to 9.
    Office(u8),
}

impl Role {
    pub fn byte(self) -> u8 {
        match self {
            Role::Progenitor => 0,
            Role::Ancients => 1,
            Role::Steward => 2,
            Role::Sovereign => 3,
            Role::Office(o) => o,
        }
    }

    pub fn of(b: u8) -> Option<Role> {
        Some(match b {
            0 => Role::Progenitor,
            1 => Role::Ancients,
            2 => Role::Steward,
            3 => Role::Sovereign,
            4..=9 => Role::Office(b),
            _ => return None,
        })
    }
}

/// A FUNCTION A SEAT MAY CALL: its id and the tags of its arguments, in
/// order. `[fn u16][argtypes u8][tag u8…]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeatFn {
    pub id: u16,
    pub tags: Vec<u8>,
}

/// ONE SEAT the house offers. `[seat u32][role u8][on key][beat ARG][fns
/// u16] FN…` — `on` the key it acts on, `beat` its cadence in the game's own
/// time (a Q or a BigQ).
#[derive(Clone, Debug, PartialEq)]
pub struct Seat {
    pub seat: u32,
    pub role: Role,
    pub on: [u8; 32],
    pub beat: Arg,
    pub fns: Vec<SeatFn>,
}

/// THE SEATS A HOUSE OFFERS AT AN EPOCH. The latest epoch replaces.
#[derive(Clone, Debug, PartialEq)]
pub struct Seats {
    pub domain: [u8; 32],
    pub epoch: u64,
    pub seats: Vec<Seat>,
}

/// A MIND TAKES A SEAT, signed by the mind's own key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Take {
    pub domain: [u8; 32],
    pub seat: u32,
    pub mind: [u8; 32],
    pub nonce: [u8; 32],
}

/// THE HOUSE ANSWERS A TAKE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Taken {
    pub domain: [u8; 32],
    pub seat: u32,
    pub mind: [u8; 32],
    pub ok: bool,
    pub reason: u16,
}

/// ONE RECORD OF A PERCEPT: `[kind u16][ARGS]`, or the house's COUNSEL
/// (kind 0xFFFF): `[fn u16][ARGS][worth ARG][cost ARG]`.
#[derive(Clone, Debug, PartialEq)]
pub enum Record {
    Plain { kind: u16, args: Vec<Arg> },
    Counsel { fn_id: u16, args: Vec<Arg>, worth: Arg, cost: Arg },
}

/// WHAT THE HOUSE LETS A SEATED MIND PERCEIVE, at the house's local time and
/// its own sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct Percept {
    pub domain: [u8; 32],
    pub seat: u32,
    pub mind: [u8; 32],
    pub local_time: Arg,
    pub seq: u64,
    pub records: Vec<Record>,
}

/// A SEATED MIND CALLS one of its seat's functions.
#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub domain: [u8; 32],
    pub seat: u32,
    pub mind: [u8; 32],
    pub nonce: [u8; 32],
    pub fn_id: u16,
    pub args: Vec<Arg>,
}

/// THE HOUSE ANSWERS A CALL: applied or refused by its reason, and the deed
/// it wrote in-game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub domain: [u8; 32],
    pub seat: u32,
    pub mind: [u8; 32],
    pub call: [u8; 32],
    pub applied: bool,
    pub reason: u16,
    pub deed: [u8; 32],
}

/// EVERY SIGNED POST.
#[derive(Clone, Debug, PartialEq)]
pub enum Post {
    Seats(Seats),
    Take(Take),
    Taken(Taken),
    Percept(Percept),
    Call(Call),
    Outcome(Outcome),
}

/// WHO MUST SIGN A POST.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signer {
    /// The domain's sitting reserve.
    House,
    /// The mind the post names.
    Mind([u8; 32]),
}

impl Post {
    pub fn sub(&self) -> u8 {
        match self {
            Post::Seats(_) => SUB_SEATS,
            Post::Take(_) => SUB_TAKE,
            Post::Taken(_) => SUB_TAKEN,
            Post::Percept(_) => SUB_PERCEPT,
            Post::Call(_) => SUB_CALL,
            Post::Outcome(_) => SUB_OUTCOME,
        }
    }

    /// The domain the post speaks in.
    pub fn domain(&self) -> [u8; 32] {
        match self {
            Post::Seats(s) => s.domain,
            Post::Take(t) => t.domain,
            Post::Taken(t) => t.domain,
            Post::Percept(t) => t.domain,
            Post::Call(t) => t.domain,
            Post::Outcome(t) => t.domain,
        }
    }

    /// The mind a post names, none for a listing.
    pub fn mind(&self) -> Option<[u8; 32]> {
        match self {
            Post::Seats(_) => None,
            Post::Take(t) => Some(t.mind),
            Post::Taken(t) => Some(t.mind),
            Post::Percept(t) => Some(t.mind),
            Post::Call(t) => Some(t.mind),
            Post::Outcome(t) => Some(t.mind),
        }
    }

    /// Who signs it: the mind for a take and a call, the house otherwise.
    pub fn signer(&self) -> Signer {
        match self {
            Post::Take(t) => Signer::Mind(t.mind),
            Post::Call(c) => Signer::Mind(c.mind),
            _ => Signer::House,
        }
    }

    /// The post's fields, written out.
    pub fn put_fields(&self, out: &mut Vec<u8>) -> Option<()> {
        let head = |out: &mut Vec<u8>, domain: &[u8; 32], seat: u32, mind: &[u8; 32]| {
            out.extend_from_slice(domain);
            out.extend_from_slice(&seat.to_le_bytes());
            out.extend_from_slice(mind);
        };
        match self {
            Post::Seats(s) => {
                out.extend_from_slice(&s.domain);
                out.extend_from_slice(&s.epoch.to_le_bytes());
                out.extend_from_slice(&u16::try_from(s.seats.len()).ok()?.to_le_bytes());
                for seat in &s.seats {
                    if !matches!(seat.beat, Arg::Q { .. } | Arg::BigQ { .. }) {
                        return None;
                    }
                    out.extend_from_slice(&seat.seat.to_le_bytes());
                    out.push(seat.role.byte());
                    out.extend_from_slice(&seat.on);
                    seat.beat.put(out)?;
                    out.extend_from_slice(&u16::try_from(seat.fns.len()).ok()?.to_le_bytes());
                    for f in &seat.fns {
                        out.extend_from_slice(&f.id.to_le_bytes());
                        out.push(u8::try_from(f.tags.len()).ok()?);
                        out.extend_from_slice(&f.tags);
                    }
                }
            }
            Post::Take(t) => {
                head(out, &t.domain, t.seat, &t.mind);
                out.extend_from_slice(&t.nonce);
            }
            Post::Taken(t) => {
                head(out, &t.domain, t.seat, &t.mind);
                out.push(u8::from(t.ok));
                out.extend_from_slice(&t.reason.to_le_bytes());
            }
            Post::Percept(t) => {
                head(out, &t.domain, t.seat, &t.mind);
                t.local_time.put(out)?;
                out.extend_from_slice(&t.seq.to_le_bytes());
                out.extend_from_slice(&u16::try_from(t.records.len()).ok()?.to_le_bytes());
                for r in &t.records {
                    match r {
                        Record::Plain { kind, args } => {
                            if *kind == KIND_COUNSEL {
                                return None;
                            }
                            out.extend_from_slice(&kind.to_le_bytes());
                            put_args(out, args)?;
                        }
                        Record::Counsel { fn_id, args, worth, cost } => {
                            out.extend_from_slice(&KIND_COUNSEL.to_le_bytes());
                            out.extend_from_slice(&fn_id.to_le_bytes());
                            put_args(out, args)?;
                            worth.put(out)?;
                            cost.put(out)?;
                        }
                    }
                }
            }
            Post::Call(c) => {
                head(out, &c.domain, c.seat, &c.mind);
                out.extend_from_slice(&c.nonce);
                out.extend_from_slice(&c.fn_id.to_le_bytes());
                put_args(out, &c.args)?;
            }
            Post::Outcome(o) => {
                head(out, &o.domain, o.seat, &o.mind);
                out.extend_from_slice(&o.call);
                out.push(u8::from(o.applied));
                out.extend_from_slice(&o.reason.to_le_bytes());
                out.extend_from_slice(&o.deed);
            }
        }
        Some(())
    }

    /// The post's fields read at `p` for sub-kind `sub`.
    pub fn take_fields(sub: u8, b: &[u8], p: &mut usize) -> Option<Post> {
        let head = |b: &[u8], p: &mut usize| -> Option<([u8; 32], u32, [u8; 32])> {
            Some((bytes::<32>(b, p)?, u32_at(b, p)?, bytes::<32>(b, p)?))
        };
        Some(match sub {
            SUB_SEATS => {
                let domain = bytes::<32>(b, p)?;
                let epoch = u64_at(b, p)?;
                let n = u16_at(b, p)?;
                let mut seats = Vec::new();
                for _ in 0..n {
                    let seat = u32_at(b, p)?;
                    let role = Role::of(u8_at(b, p)?)?;
                    let on = bytes::<32>(b, p)?;
                    let beat = Arg::take(b, p)?;
                    if !matches!(beat, Arg::Q { .. } | Arg::BigQ { .. }) {
                        return None;
                    }
                    let nf = u16_at(b, p)?;
                    let mut fns = Vec::new();
                    for _ in 0..nf {
                        let id = u16_at(b, p)?;
                        let nt = usize::from(u8_at(b, p)?);
                        let tags = b.get(*p..*p + nt)?.to_vec();
                        *p += nt;
                        fns.push(SeatFn { id, tags });
                    }
                    seats.push(Seat { seat, role, on, beat, fns });
                }
                Post::Seats(Seats { domain, epoch, seats })
            }
            SUB_TAKE => {
                let (domain, seat, mind) = head(b, p)?;
                Post::Take(Take { domain, seat, mind, nonce: bytes::<32>(b, p)? })
            }
            SUB_TAKEN => {
                let (domain, seat, mind) = head(b, p)?;
                let ok = flag(u8_at(b, p)?)?;
                Post::Taken(Taken { domain, seat, mind, ok, reason: u16_at(b, p)? })
            }
            SUB_PERCEPT => {
                let (domain, seat, mind) = head(b, p)?;
                let local_time = Arg::take(b, p)?;
                let seq = u64_at(b, p)?;
                let n = u16_at(b, p)?;
                let mut records = Vec::new();
                for _ in 0..n {
                    let kind = u16_at(b, p)?;
                    records.push(if kind == KIND_COUNSEL {
                        let fn_id = u16_at(b, p)?;
                        let args = take_args(b, p)?;
                        let worth = Arg::take(b, p)?;
                        Record::Counsel { fn_id, args, worth, cost: Arg::take(b, p)? }
                    } else {
                        Record::Plain { kind, args: take_args(b, p)? }
                    });
                }
                Post::Percept(Percept { domain, seat, mind, local_time, seq, records })
            }
            SUB_CALL => {
                let (domain, seat, mind) = head(b, p)?;
                let nonce = bytes::<32>(b, p)?;
                let fn_id = u16_at(b, p)?;
                Post::Call(Call { domain, seat, mind, nonce, fn_id, args: take_args(b, p)? })
            }
            SUB_OUTCOME => {
                let (domain, seat, mind) = head(b, p)?;
                let call = bytes::<32>(b, p)?;
                let applied = flag(u8_at(b, p)?)?;
                let reason = u16_at(b, p)?;
                Post::Outcome(Outcome { domain, seat, mind, call, applied, reason, deed: bytes::<32>(b, p)? })
            }
            _ => return None,
        })
    }

    /// The bytes the signer signs: `b"mycelium seat v1" ‖ [sub] ‖ [fields]`.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        let mut out = SIGNED.to_vec();
        out.push(self.sub());
        self.put_fields(&mut out)?;
        Some(out)
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> Option<SignedPost> {
        use ed25519_dalek::Signer as _;
        let sig = key.sign(&self.signed_bytes()?).to_bytes();
        Some(SignedPost { post: self, sig })
    }
}

/// A POST AND ITS SIGNATURE.
#[derive(Clone, Debug, PartialEq)]
pub struct SignedPost {
    pub post: Post,
    pub sig: [u8; 64],
}

impl SignedPost {
    /// Whether `vk` signed it.
    pub fn signed_by(&self, vk: &[u8; 32]) -> bool {
        self.post.signed_bytes().is_some_and(|m| crate::court::identity::verify(vk, &m, &self.sig))
    }

    /// The post's fields and signature — what the relay holds and serves
    /// (the mail's and the listing's body).
    pub fn body(&self) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        self.post.put_fields(&mut out)?;
        out.extend_from_slice(&self.sig);
        Some(out)
    }

    /// A body of sub-kind `sub` read whole, or none.
    pub fn of_body(sub: u8, b: &[u8]) -> Option<SignedPost> {
        let mut p = 0usize;
        let post = Post::take_fields(sub, b, &mut p)?;
        let sig = bytes::<64>(b, &mut p)?;
        (p == b.len()).then_some(SignedPost { post, sig })
    }

    /// 78.145 — A CALL'S REFERENCE: the citation hash of exactly the bytes
    /// its signature covers. Any post has one; an OUTCOME names its call by
    /// it.
    pub fn reference(&self) -> Option<[u8; 32]> {
        Some(crate::chronometry::payload_hash(&self.post.signed_bytes()?))
    }
}

/// `[0x53][sub][fields][sig]`.
pub fn encode_post(s: &SignedPost) -> Option<Vec<u8>> {
    let mut out = vec![ACT_SEAT, s.post.sub()];
    out.extend_from_slice(&s.body()?);
    Some(out)
}

/// A READER ASKS ITS MAIL past `after`, which acknowledges everything at or
/// below it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadMail {
    pub reader: [u8; 32],
    pub after: u64,
    pub nonce: [u8; 32],
    pub sig: [u8; 64],
}

impl ReadMail {
    fn fields(reader: &[u8; 32], after: u64, nonce: &[u8; 32]) -> Vec<u8> {
        let mut out = SIGNED.to_vec();
        out.push(SUB_READ_MAIL);
        out.extend_from_slice(reader);
        out.extend_from_slice(&after.to_le_bytes());
        out.extend_from_slice(nonce);
        out
    }

    pub fn sign(key: &ed25519_dalek::SigningKey, after: u64, nonce: [u8; 32]) -> ReadMail {
        use ed25519_dalek::Signer as _;
        let reader = key.verifying_key().to_bytes();
        let sig = key.sign(&ReadMail::fields(&reader, after, &nonce)).to_bytes();
        ReadMail { reader, after, nonce, sig }
    }

    /// Whether the reader signed it.
    pub fn verified(&self) -> bool {
        crate::court::identity::verify(&self.reader, &ReadMail::fields(&self.reader, self.after, &self.nonce), &self.sig)
    }
}

/// `[0x53][6][domain]`.
pub fn encode_read_listing(domain: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_SEAT, SUB_READ_LISTING];
    out.extend_from_slice(domain);
    out
}

/// `[0x53][7][reader][after][nonce][sig]`.
pub fn encode_read_mail(r: &ReadMail) -> Vec<u8> {
    let mut out = vec![ACT_SEAT, SUB_READ_MAIL];
    out.extend_from_slice(&r.reader);
    out.extend_from_slice(&r.after.to_le_bytes());
    out.extend_from_slice(&r.nonce);
    out.extend_from_slice(&r.sig);
    out
}

/// WHAT A SEAT FRAME ASKS.
#[derive(Clone, Debug, PartialEq)]
pub enum SeatRequest {
    Post(SignedPost),
    ReadListing([u8; 32]),
    ReadMail(ReadMail),
}

/// WHY THE RELAY REFUSED A FRAME.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The signature does not verify under the key the post names.
    Unsigned,
    /// A house post not signed by the domain's sitting reserve.
    NotTheHouse,
    /// The frame does not read whole.
    Malformed,
    /// The domain has no sitting reserve on this court.
    UnknownDomain,
}

impl Refusal {
    pub fn byte(self) -> u8 {
        match self {
            Refusal::Unsigned => 1,
            Refusal::NotTheHouse => 2,
            Refusal::Malformed => 3,
            Refusal::UnknownDomain => 4,
        }
    }

    pub fn of(b: u8) -> Option<Refusal> {
        [Refusal::Unsigned, Refusal::NotTheHouse, Refusal::Malformed, Refusal::UnknownDomain].into_iter().find(|r| r.byte() == b)
    }
}

/// A seat frame read: `None` where it is not `ACT_SEAT` at all, `Err` where
/// it is and does not read whole (a byte left over included).
pub fn decode_seat_request(b: &[u8]) -> Option<Result<SeatRequest, Refusal>> {
    if !act_is(b, ACT_SEAT) {
        return None;
    }
    Some(read_request(b).ok_or(Refusal::Malformed))
}

fn read_request(b: &[u8]) -> Option<SeatRequest> {
    let sub = *b.get(1)?;
    let rest = b.get(2..)?;
    match sub {
        SUB_READ_LISTING => {
            let domain: [u8; 32] = rest.try_into().ok()?;
            Some(SeatRequest::ReadListing(domain))
        }
        SUB_READ_MAIL => {
            let mut p = 0usize;
            let reader = bytes::<32>(rest, &mut p)?;
            let after = u64_at(rest, &mut p)?;
            let nonce = bytes::<32>(rest, &mut p)?;
            let sig = bytes::<64>(rest, &mut p)?;
            (p == rest.len()).then_some(SeatRequest::ReadMail(ReadMail { reader, after, nonce, sig }))
        }
        _ => SignedPost::of_body(sub, rest).map(SeatRequest::Post),
    }
}

/// A post's answer: `[1][seq u64]` or `[0][reason u8]`.
pub fn encode_post_reply(r: Result<u64, Refusal>) -> Vec<u8> {
    match r {
        Ok(seq) => {
            let mut out = vec![1];
            out.extend_from_slice(&seq.to_le_bytes());
            out
        }
        Err(why) => vec![0, why.byte()],
    }
}

pub fn decode_post_reply(b: &[u8]) -> Option<Result<u64, Refusal>> {
    match b {
        [1, rest @ ..] => Some(Ok(u64::from_le_bytes(rest.try_into().ok()?))),
        [0, why] => Some(Err(Refusal::of(*why)?)),
        _ => None,
    }
}

/// The listing as served: the latest SEATS body, or nothing.
pub fn decode_listing_reply(b: &[u8]) -> Option<Option<SignedPost>> {
    if b.is_empty() {
        return Some(None);
    }
    SignedPost::of_body(SUB_SEATS, b).filter(|s| matches!(s.post, Post::Seats(_))).map(Some)
}

/// ONE PIECE OF MAIL as the relay holds it: its sequence, its sub-kind and
/// the post's body (fields ‖ sig).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mail {
    pub seq: u64,
    pub sub: u8,
    pub body: Vec<u8>,
}

impl Mail {
    /// The bytes one piece takes in a reply.
    pub fn wire_len(&self) -> usize {
        8 + 1 + 4 + self.body.len()
    }

    /// The post it carries, read whole and typed.
    pub fn post(&self) -> Option<SignedPost> {
        SignedPost::of_body(self.sub, &self.body)
    }
}

/// `[count u16]` then `(seq u64, sub u8, len u32, body)…`; or the refusal.
pub fn encode_mail_reply(r: Result<&[Mail], Refusal>) -> Option<Vec<u8>> {
    let mail = match r {
        Ok(m) => m,
        Err(why) => return Some(vec![0, why.byte()]),
    };
    let mut out = u16::try_from(mail.len()).ok()?.to_le_bytes().to_vec();
    for m in mail {
        out.extend_from_slice(&m.seq.to_le_bytes());
        out.push(m.sub);
        out.extend_from_slice(&u32::try_from(m.body.len()).ok()?.to_le_bytes());
        out.extend_from_slice(&m.body);
    }
    Some(out)
}

pub fn decode_mail_reply(b: &[u8]) -> Option<Result<Vec<Mail>, Refusal>> {
    match b {
        [0, why] if *why != 0 => return Some(Err(Refusal::of(*why)?)),
        _ => {}
    }
    let mut p = 0usize;
    let n = u16_at(b, &mut p)?;
    let mut mail = Vec::new();
    for _ in 0..n {
        let seq = u64_at(b, &mut p)?;
        let sub = u8_at(b, &mut p)?;
        let len = usize::try_from(u32_at(b, &mut p)?).ok()?;
        let body = b.get(p..p + len)?.to_vec();
        p += len;
        mail.push(Mail { seq, sub, body });
    }
    (p == b.len()).then_some(Ok(mail))
}

fn flag(b: u8) -> Option<bool> {
    match b {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

fn bytes<const N: usize>(b: &[u8], p: &mut usize) -> Option<[u8; N]> {
    let v: [u8; N] = b.get(*p..*p + N)?.try_into().ok()?;
    *p += N;
    Some(v)
}

fn u8_at(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

fn u16_at(b: &[u8], p: &mut usize) -> Option<u16> {
    bytes::<2>(b, p).map(u16::from_le_bytes)
}

fn u32_at(b: &[u8], p: &mut usize) -> Option<u32> {
    bytes::<4>(b, p).map(u32::from_le_bytes)
}

fn u64_at(b: &[u8], p: &mut usize) -> Option<u64> {
    bytes::<8>(b, p).map(u64::from_le_bytes)
}
