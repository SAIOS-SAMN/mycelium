// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 24 — what asserting a word did (MU-RECURSION 1.9; MU-ANASTOME §5.1, §6).

/// What asserting a word did with a closed mint. Only [`Assertion::Admitted`]
/// is a fresh frame; the memo and the identity gate are not fresh speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assertion {
    /// Adopted up; the address is now on the memo.
    Admitted,
    /// This address was already asserted; the judge has the word.
    Already,
    /// Identity, or a mint with no group component: not a message.
    Silent,
}
