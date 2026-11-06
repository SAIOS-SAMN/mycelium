// 24 — what speaking a word did.

/// What `speak_word` did with a closed mint. Only [`Expression::Spoken`]
/// is a fresh frame; the memo and the identity gate are not speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    /// Adopted up; the address is now on the memo.
    Spoken,
    /// This address was already asserted; the judge has the word.
    Already,
    /// Identity, or a mint with no group component: not a message.
    Silent,
}
