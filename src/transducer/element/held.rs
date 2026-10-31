// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! L16, 18.3 — WHAT AN ORGAN HOLDS. A word an organ has taken in is either
//! a sentence, and then the FORM its tape decodes to is part of what is
//! held, or it is not, and then there is no form to hold.
//!
//! THE DECODE IS THE ADMISSION'S, NOT THE READ'S. `Element` carried a
//! `OnceLock<Option<Form>>` and answered `form()` by decoding the tape the
//! first time anybody asked. That put a mutable cache inside the value
//! every organ passes around by reference: a cloned element carries an
//! empty lock and re-parses, `substance()` and `set_description` take the
//! lock and the next reader parses again, and the same tape is parsed once
//! per copy rather than once per word. Measured at 79ba5e2: `on_class`
//! carries a comment recording that decoding a fresh clone "re-parsed the
//! tape every call, which made the cipher's carrier the working memory of
//! the organ" — the workaround was to read off the held element, which is
//! this type, arrived at one call site at a time.
//!
//! Here the decode happens ONCE, where the organ takes the word in, and
//! the result is a field. A `Held` that could not decode its tape is
//! `Held::Word`, which is exactly what `Element::form()` answered there:
//! a sentence whose tape does not read carries no form, and that is a
//! reading, not a cache miss to retry.

use super::Element;
use crate::transducer::sentence::Form;

/// ONE WORD AS AN ORGAN HOLDS IT.
#[derive(Debug, Clone)]
pub enum Held {
    /// A word with no sentence form: a connection, a rewrite, a
    /// translation, a pure residual chain — or a sentence whose tape does
    /// not decode, which carries no form either.
    Word(Element),
    /// A SENTENCE and the form its tape decodes to, decoded once when the
    /// organ took it in. Boxed: a `Form` is large beside an `Element`, and
    /// every `Held::Word` would carry that footprint.
    Sentence(Element, Box<Form>),
}

impl Held {
    /// Take a word in: decode its tape once and hold what came back.
    pub fn of(e: Element) -> Held {
        match crate::transducer::sentence::from_element_form(&e) {
            Some(f) => Held::Sentence(e, Box::new(f)),
            None => Held::Word(e),
        }
    }

    /// The word itself.
    pub fn element(&self) -> &Element {
        match self {
            Held::Word(e) | Held::Sentence(e, _) => e,
        }
    }

    /// The form this word carries, or none — the same answer
    /// `Element::form()` gave, read off a field rather than a lock.
    pub fn form(&self) -> Option<&Form> {
        match self {
            Held::Word(_) => None,
            Held::Sentence(_, f) => Some(f),
        }
    }

    /// Change the word and re-decode. The two cannot drift: there is no
    /// path that writes one without the other, which is what taking a
    /// `OnceLock` by hand at each mutation site was standing in for.
    pub fn set(&mut self, e: Element) {
        *self = Held::of(e);
    }

    /// Change the word THROUGH a reader that may mutate it, then re-decode.
    /// Every mutation of a held word goes through here, so a caller cannot
    /// leave a form describing a tape the element no longer has.
    pub fn update<R>(&mut self, f: impl FnOnce(&mut Element) -> R) -> R {
        let mut e = self.element().clone();
        let out = f(&mut e);
        self.set(e);
        out
    }

    /// 37.7, 45.9 — THE GENERATORS THIS WORD OPENED: its word vertices that
    /// cite nothing (read off the envelope, 54.5 Road (d)) and carry a rotor
    /// the seventeen do not spell. The ONE reader of adjunction (law 12): a
    /// court sums it over its ruled words (`Court::adjoined`), a mind over
    /// its genome (LC1), and the two cannot disagree about what counts.
    pub fn generators_opened(&self) -> u64 {
        let Some(f) = self.form() else { return 0 };
        let env = self.element().receipts();
        f.vertices
            .scoped()
            .filter(|(_, v)| match v {
                // An adjoined generator counts whatever its rotor: a motion
                // letter's rotor is the identity, which the seventeen spell,
                // so the spelling test alone never counted one (3 737 motion
                // letters on the development deployment, none opened, epoch 146).
                crate::transducer::sentence::Vertex::Word { address, derivation, .. } => {
                    env.cite_for(address.as_bytes()).is_none()
                        && (derivation.is_adjoined()
                            || !matches!(crate::transducer::alphabet::factor_rotor(&derivation.rotor()), Some(Ok(_))))
                }
                _ => false,
            })
            .count() as u64
    }
}

impl std::ops::Deref for Held {
    type Target = Element;
    fn deref(&self) -> &Element {
        self.element()
    }
}

impl From<Element> for Held {
    fn from(e: Element) -> Held {
        Held::of(e)
    }
}
