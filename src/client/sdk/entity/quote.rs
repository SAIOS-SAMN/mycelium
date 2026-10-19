// 23.8 — THE QUOTE: what the market offers this entity, read across every
// hypha it knows. (The refinement bid it once made over chain words is
// gone: since MU-CLIENT §7.1's doctrine a chain is an open question and
// never a genome word, so no genome word was left for it to examine; the
// court consolidates its own language, `retire_redundant`.)

use crate::client::sdk::*;

impl Entity {
    /// ASK THE QUOTE (23.8): the charts this mind is thinking about, each
    /// with the standing it holds there, read on the field; routed to the
    /// hyphae that home them and asked on the carrier. One quote is in flight
    /// at a time, and it lands on the board (`quote_inflow` reads it).
    pub(crate) fn quote(&mut self) {
        if self.board.court.quoting {
            return;
        }
        let thought_charts: Vec<crate::transducer::stalk::Invariant> = {
            let mut charts = Vec::new();
            // THE BUBBLES THIS MIND HOLDS, all of them: the charts it asks
            // a quote on are the charts it is thinking about, and one left
            // out is a thought the book is never asked about.
            for (_, b) in self.contemplation.bubbles.scoped() {
                let inv = crate::transducer::stalk::invariant(&b.remainder);
                let mut cur = inv;
                loop {
                    if b.cover.contains(&cur.key()) {
                        charts.push(cur.clone());
                    }
                    if cur.0.is_empty() {
                        break;
                    }
                    cur = cur.peel();
                }
            }
            charts
        };
        // Each chart the ask may carry, with the standing this mind holds on
        // it: the charts it thinks about, the root (an empty atlas is
        // incident to the root prefix — the first quote reads the book at
        // depth zero, so a mind need not already hold a word to learn one),
        // and each held bubble's chart with the volume it routes by.
        let standing = |inv: &crate::transducer::stalk::Invariant| (inv.clone(), self.working_atlas().standing_on(inv).as_u8());
        let root = crate::transducer::stalk::Invariant(Vec::new());
        let empty = self.atlas_is_empty();
        let plan = QuotePlan {
            charts: thought_charts.iter().map(standing).collect(),
            root: vec![standing(&root)],
            bubbles: self
                .contemplation
                .bubbles
                .scoped()
                .map(|(_, b)| (standing(&crate::transducer::stalk::invariant(&b.remainder)), b.volume))
                .collect(),
            empty,
            voice: self.hypha.kernel().voice(),
        };
        let mut budget = self.physics.stroke();
        self.board.court.quoting = true;
        self.ask(move |s| plan.ask(s, &mut budget), |me, q| {
            me.board.court.quoting = false;
            me.board.court.quoted = q.ok();
        });
    }
}

/// A CHART AND THE STANDING THE MIND HOLDS ON IT.
type Standing8 = (crate::transducer::stalk::Invariant, u8);

/// WHAT A QUOTE ASKS, read on the field: the charts the mind thinks about,
/// the root, each held bubble's chart with its volume, whether the atlas is
/// empty, and the voice asking.
struct QuotePlan {
    charts: Vec<Standing8>,
    root: Vec<Standing8>,
    bubbles: Vec<(Standing8, [u64; 4])>,
    empty: bool,
    voice: [u8; 32],
}

impl QuotePlan {
    /// THE QUOTE ASKED ON THE CARRIER: each bubble's chart routed to the
    /// hypha that homes it and every hypha asked its charts; where nothing
    /// routes, or the atlas is empty, every known hypha is asked in turn.
    /// The listings merge by address.
    fn ask(self, s: &mut Session, budget: &mut crate::physics::Budget) -> io::Result<YieldQuote> {
        let mut dests: crate::network::nerve::Keyed<Vec<Standing8>> = crate::network::nerve::Keyed::new();
        let mut dest_order: Vec<String> = Vec::new();
        for ((inv, st), mag) in &self.bubbles {
            // A chart no hypha is known for is not routed to the empty
            // address; it is not routed. The whole ask goes to every hypha
            // below when nothing routes.
            let Some(dest) = s.route_chart(inv, mag) else {
                continue;
            };
            let dk = dest.as_bytes();
            match dests.get_mut(dk) {
                Some(held) => {
                    if !held.iter().any(|(h, _)| h == inv) {
                        held.push((inv.clone(), *st));
                    }
                }
                None => {
                    dest_order.push(dest.clone());
                    dests.insert(dk, vec![(inv.clone(), *st)]);
                }
            }
        }
        if dests.is_empty() || self.empty {
            let asked = if self.charts.is_empty() || self.empty { &self.root } else { &self.charts };
            return quote_all_hyphae(s, &self.voice, asked, budget);
        }
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
        let mut out: Option<YieldQuote> = None;
        for dest in dest_order {
            // A destination the routing named no charts for is asked the
            // whole ask; the empty list is that reading.
            let asked = match dests.get(dest.as_bytes()) {
                Some(i) if !i.is_empty() => i.clone(),
                _ => self.charts.clone(),
            };
            // An ask whose charts do not fit the count the frame writes is
            // not asked short (47.1).
            let Some(req) = dialogue::encode_quote_request_standing(&self.voice, &asked) else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "the ask does not fit the lengths its frame writes"));
            };
            let reply = match s.exchange_at(&dest, req.clone()) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => s.speak_nerve(req),
                other => other,
            };
            match reply {
                Ok(body) => merge_quote(&mut out, &body)?,
                Err(e) => last_err = e,
            }
        }
        settle_quote(out, last_err)
    }
}

/// Visit every known hypha by a keyed cursor under the stroke Budget.
/// Used while the genome is empty so the first syllabus/quote is a fan-out,
/// not a seed pin.
fn quote_all_hyphae(s: &mut Session, voice: &[u8; 32], asked: &[Standing8], budget: &mut crate::physics::Budget) -> io::Result<YieldQuote> {
    let peers = s.peers();
    let Some(start) = s.peer_start(peers.len()) else {
        let charts: Vec<crate::transducer::stalk::Invariant> = asked.iter().map(|(inv, _)| inv.clone()).collect();
        return s.quote(&charts);
    };
    s.bump_cursor();
    // An ask whose charts do not fit the count the frame writes is not
    // asked short (47.1).
    let Some(req) = dialogue::encode_quote_request_standing(voice, asked) else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "the ask does not fit the lengths its frame writes"));
    };
    let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
    let mut out: Option<YieldQuote> = None;
    for i in 0..peers.len() {
        if !budget.spend() {
            break;
        }
        match s.exchange_at(&peers[(start + i) % peers.len()], req.clone()) {
            Ok(body) => merge_quote(&mut out, &body)?,
            Err(e) => last_err = e,
        }
    }
    settle_quote(out, last_err)
}

/// ONE HYPHA'S QUOTE MERGED into what the others answered: its listings
/// joined by address, each once; the first answer's height, surface and K
/// stand.
fn merge_quote(out: &mut Option<YieldQuote>, body: &[u8]) -> io::Result<()> {
    let (height, surface, k, items) =
        dialogue::decode_quote_reply(body).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
    let clusters: Vec<Cluster> =
        items.into_iter().map(|i| Cluster { address: i.addr, mu: i.mu, element: i.element, open: i.open }).collect();
    match out {
        None => *out = Some(YieldQuote { height, surface, k, clusters, deficit: None }),
        Some(q) => {
            let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
            for h in &q.clusters {
                have.insert(&h.address, ());
            }
            for c in clusters {
                if have.contains(&c.address) {
                    continue;
                }
                have.insert(&c.address, ());
                q.clusters.push(c);
            }
        }
    }
    Ok(())
}

/// The merged quote, or — where no hypha answered — the quote a deferral
/// reads as, else the last refusal. MU-CLIENT §4.13 — A DEFERRAL'S LANES ARE
/// THE PRESSURE THEY ARE: no height, no K, no clusters, and the door's
/// deficit as the surface, so the mind dwells on the lanes the court named
/// (23.8, S13.4), as `Session::quote` reads it — never four zero lanes,
/// which open the gate a deferral holds.
fn settle_quote(out: Option<YieldQuote>, last_err: io::Error) -> io::Result<YieldQuote> {
    match out {
        Some(q) => Ok(q),
        None if last_err.kind() == io::ErrorKind::WouldBlock => match last_err.get_ref().and_then(|inner| inner.downcast_ref::<Deferral>()) {
            Some(d) => Ok(YieldQuote { height: 0, surface: d.lanes, k: 0, clusters: Vec::new(), deficit: Some(d.lanes) }),
            // A would-block that carries no deferral names no lanes: it is the
            // refusal it is, never a surface read off nothing.
            None => Err(last_err),
        },
        None => Err(last_err),
    }
}
