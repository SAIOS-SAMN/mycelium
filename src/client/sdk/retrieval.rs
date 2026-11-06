// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.154 (4), 78.152 TU-6, issue #159 — A COORDINATE RETRIEVAL ON THE
// SESSION: any staker, a mind or a kernel, asks for the receipts at a final
// point's tick and pays the read price from its stake.

use std::io;

impl super::Session {
    /// #159 — A COORDINATE RETRIEVAL (`ACT_DOMAIN` sub-kind 19): the
    /// receipts at the point's tick with the fee claimed and the court's
    /// signed section over its head, or the refusal by name. The answer is
    /// returned as read; the caller checks it against the court's key with
    /// `retrieve::verify_retrieved`, and a served answer that does not stand
    /// is the court's own signed claim to present.
    pub fn retrieve(&self, s: &crate::anastome::dialogue::retrieve::SignedRetrieve) -> io::Result<crate::anastome::dialogue::retrieve::RetrieveReply> {
        use crate::anastome::dialogue::retrieve::{decode_retrieve_reply, encode_retrieve};
        let reply = self.speak(encode_retrieve(s))?;
        decode_retrieve_reply(&reply).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "the retrieval's reply did not decode"))
    }
}
