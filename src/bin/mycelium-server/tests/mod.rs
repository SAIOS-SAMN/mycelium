// 39 — the hypha binary's own tests, one file per ORGAN they prove.

mod channel;
mod deferral;
mod genesis;
mod pacing;
mod patience;
mod standing;
mod strokes;
mod tick;
mod wire;
mod witness;

/// A FIXTURE PEER'S READ: one envelope off `s`, its expression read, the
/// act answered. `None` where the stream ends or the bytes are no
/// expression.
pub(crate) fn read_act(s: &mut impl std::io::Read) -> Option<Vec<u8>> {
    let mut head = [0u8; mycelium::network::wire::HEAD];
    s.read_exact(&mut head).ok()?;
    let mut body = vec![0u8; mycelium::network::wire::delimited_len(head)?];
    s.read_exact(&mut body).ok()?;
    Some(mycelium::anastome::dialogue::Expression::read(body)?.act)
}

/// A FIXTURE PEER'S REPLY: `act` written as an expression with no point,
/// framed whole.
pub(crate) fn reply_bytes(act: &[u8]) -> Vec<u8> {
    let body = mycelium::anastome::dialogue::write_act(act, None).expect("an expression");
    mycelium::network::wire::Delimited { body }.encode().expect("framed")
}

/// The act an expression carries, its point set aside.
pub(crate) fn act_of(expression: Vec<u8>) -> Vec<u8> {
    mycelium::anastome::dialogue::Expression::read(expression).expect("an expression").act
}
