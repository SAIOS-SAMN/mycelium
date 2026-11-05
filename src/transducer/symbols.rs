// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SYMBOL SET Σ (MU-SENSING §3.3; ROADMAP track 26, S3): the distinct
//! symbols across a family's samples, compared by equality, with `|Σ|` read
//! from the data and never fixed in the code.
//!
//! - **An index into Σ** is a symbol's rank in canonical address order: the
//!   order of first appearance when the family's samples are walked in the
//!   order the family gives them, each sample's sites in canonical address
//!   order (axis 0 slowest). The index addresses a symbol and nothing more:
//!   it is never a role and never a choice.
//! - **The reservoir ray `e_∅`** (MU-RECURSION 78.58) stands at index `|Σ|`,
//!   the place after the family's own set ([`SymbolSet::reservoir`]).
//! - **A table over symbols** is keyed by the symbol and holds at most
//!   `|Σ|` entries, never a fixed array of slots.
//! - **A symbol's code is a label.** Nothing here reads it as a number, an
//!   order or a meaning; a colour word names a fibre relation across the
//!   family (78.53), never a code.
//! - **Σ is over all of a family's posted samples** (§3.3: "across all of
//!   its samples"), its test inputs included: a reader handed the train
//!   pairs reads them with the test inputs the task posts beside them
//!   ([`on_test_inputs`]), so a symbol seen only in a test input is in Σ.

use crate::network::nerve::Keyed;

use super::sample::Sample;

thread_local! {
    /// THE TEST INPUTS POSTED BESIDE THE FAMILY READ ON THIS THREAD, in the
    /// order the task gives them; empty where the reader holds the train
    /// pairs alone (a family of pairs with no test posted).
    static POSTED: std::cell::RefCell<Vec<Sample>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// READ `f` WITH THE FAMILY'S TEST INPUTS POSTED: every Σ read inside it
/// (`SymbolSet::of_family`) walks the train pairs, then these inputs, each
/// a sample of its own rank. The previous scope is restored as `f` returns.
pub fn on_test_inputs<'a, T>(tests: impl IntoIterator<Item = &'a Sample>, f: impl FnOnce() -> T) -> T {
    let before = POSTED.with(|c| c.replace(tests.into_iter().cloned().collect()));
    let out = f();
    POSTED.with(|c| *c.borrow_mut() = before);
    out
}

/// WHY A SYMBOL SET IS NOT READ, named with its cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolRefusal {
    /// The family holds no sample: no width to read a symbol at.
    NoSample,
    /// A sample's symbol width is not the width of the samples before it:
    /// two widths are two sources, and their symbols are not compared.
    Width { sample: usize, width: usize, found: usize },
}

/// A FAMILY'S SYMBOL SET Σ: each distinct symbol once, in index order, and
/// the index of each read by the symbol's own bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolSet {
    /// The source's symbol width in bytes.
    width: usize,
    /// The symbols in index order, `width` bytes each.
    symbols: Vec<u8>,
    /// Each symbol's index, at the symbol's bytes (A13: a keyed read, never
    /// a scan of the set).
    index: Keyed<usize>,
}

impl SymbolSet {
    /// THE EMPTY SET at a symbol width: no symbol read yet.
    fn empty(width: usize) -> SymbolSet {
        SymbolSet { width, symbols: Vec::new(), index: Keyed::new() }
    }

    /// Read one symbol: a symbol not yet held takes the next index.
    fn read(&mut self, symbol: &[u8]) {
        if self.index.contains(symbol) {
            return;
        }
        let at = self.len();
        self.index.insert(symbol, at);
        self.symbols.extend_from_slice(symbol);
    }

    /// THE SYMBOL SET OF A FAMILY OF SAMPLES, walked in the order the family
    /// gives them. Refused `Width` where a sample's symbol width is not the
    /// first sample's, and `NoSample` for a family of none.
    pub fn of_samples<'a>(samples: impl IntoIterator<Item = &'a Sample>) -> Result<SymbolSet, SymbolRefusal> {
        let mut set: Option<SymbolSet> = None;
        for (at, sample) in samples.into_iter().enumerate() {
            let held = set.get_or_insert_with(|| SymbolSet::empty(sample.width()));
            if sample.width() != held.width {
                return Err(SymbolRefusal::Width { sample: at, width: held.width, found: sample.width() });
            }
            for symbol in sample.symbols() {
                held.read(symbol.bytes());
            }
        }
        set.ok_or(SymbolRefusal::NoSample)
    }

    /// THE SYMBOL SET OF RASTERS AT INTAKE (one byte per site), walked in
    /// the order given, each raster's sites in canonical address order.
    #[cfg(test)]
    pub fn of_rasters<'a>(rasters: impl IntoIterator<Item = &'a [Vec<u8>]>) -> SymbolSet {
        let mut set = SymbolSet::empty(1);
        for raster in rasters {
            for symbol in raster.iter().flatten() {
                set.read(std::slice::from_ref(symbol));
            }
        }
        set
    }

    /// THE SYMBOL SET OF A FAMILY OF PAIRS: each pair's two members in the
    /// order the pair gives them, read as samples. A family of no pair
    /// holds no symbol.
    #[cfg(test)]
    pub fn of_pairs(family: &[super::family::TrainPair]) -> SymbolSet {
        SymbolSet::of_section(family.iter().flat_map(|(x, y)| [x.sample(), y.sample()]))
    }

    /// THE SYMBOL SET OF SAMPLES ON THE ALPHABET'S SECTION, one byte each:
    /// every sample there is at the section's one width, so the set is
    /// read whole; samples of no width hold no symbol.
    #[cfg(test)]
    fn of_section<'a>(samples: impl IntoIterator<Item = &'a Sample>) -> SymbolSet {
        match SymbolSet::of_samples(samples) {
            Ok(set) => set,
            Err(_) => SymbolSet::empty(1),
        }
    }

    /// THE SYMBOL SET OF THE WHOLE FAMILY: its train pairs in the family's
    /// order, then the test inputs posted beside them (`on_test_inputs`), so
    /// a symbol seen only in a test input is in Σ (MU-SENSING §3.3).
    /// Refused by name, never read empty: `Width` where a posted test input's
    /// symbol width is not the train pairs' (two widths are two sources), and
    /// `NoSample` for a family of no pair and no posted input.
    pub fn of_family(train: &[super::family::TrainPair]) -> Result<SymbolSet, SymbolRefusal> {
        POSTED.with(|c| {
            let tests = c.borrow();
            let pairs = train.iter().flat_map(|(x, y)| [x.sample(), y.sample()]);
            SymbolSet::of_samples(pairs.chain(tests.iter()))
        })
    }

    /// `|Σ|`: the count of distinct symbols, read from the data.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Whether the family showed no symbol at all.
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// The source's symbol width in bytes.
    pub fn width(&self) -> usize {
        self.width
    }

    /// THE INDEX OF A SYMBOL: its rank of first appearance in canonical
    /// address order, or `None` for a symbol the family never showed.
    pub fn index(&self, symbol: &[u8]) -> Option<usize> {
        self.index.get(symbol).copied()
    }

    /// The symbol at an index, or `None` at or past `|Σ|` (the reservoir is
    /// no symbol).
    pub fn symbol(&self, index: usize) -> Option<&[u8]> {
        let from = index.checked_mul(self.width)?;
        self.symbols.get(from..from.checked_add(self.width)?)
    }

    /// THE INDEX OF THE RESERVOIR RAY `e_∅` (78.58): `|Σ|`, the place after
    /// the family's own set.
    pub fn reservoir(&self) -> usize {
        self.len()
    }

    /// The symbols in index order.
    pub fn symbols(&self) -> impl Iterator<Item = &[u8]> + '_ {
        self.symbols.chunks(self.width.max(1))
    }
}
