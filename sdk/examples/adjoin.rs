// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// A HYPHA ADJOINS ITSELF TO A RUNNING MESH, or retracts (78.185): the act
// signed by the hypha's own key (its home's `hypha.key`) and spoken to a
// hypha of the mesh. Each tether is to a genesis hypha by the identity
// motion, at rest on the identity plane: a hypha minted in the mesh's own
// frame, holding no relative phase yet.
//
//! `adjoin <hypha home> <hypha address> <node hex 64>…`
//! `adjoin --retract <hypha home> <hypha address>`

use mycelium_sdk::house::{decode_cover_reply, encode_adjunction, encode_retraction, Adjunction, Retraction, TetherStated};
use mycelium_sdk::prelude::*;
use std::process::ExitCode;

fn hex32(s: &str) -> Option<[u8; 32]> {
    let b: Option<Vec<u8>> = (0..s.len()).step_by(2).map(|i| s.get(i..i + 2).and_then(|x| u8::from_str_radix(x, 16).ok())).collect();
    b.and_then(|b| <[u8; 32]>::try_from(b).ok())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let retract = args.first().is_some_and(|a| a == "--retract");
    let rest: Vec<&String> = args.iter().filter(|a| *a != "--retract").collect();
    let (Some(home), Some(endpoint)) = (rest.first(), rest.get(1)) else {
        eprintln!("usage: adjoin [--retract] <hypha home> <hypha address> <node hex>…");
        return ExitCode::from(2);
    };
    let Ok(seed) = std::fs::read(format!("{home}/hypha.key")) else {
        eprintln!("{home}/hypha.key does not read");
        return ExitCode::from(2);
    };
    let Ok(seed) = <[u8; 32]>::try_from(seed.as_slice()) else {
        eprintln!("{home}/hypha.key is not 32 bytes");
        return ExitCode::from(2);
    };
    let key = mycelium_sdk::voice::SigningKey::from_bytes(&seed);
    let hypha = key.verifying_key().to_bytes();
    let (voice, _) = mycelium_sdk::voice::load_or_mint(&format!("{home}/voice.key"));
    let session = match mycelium_sdk::session::NoiseHome::open(std::path::Path::new(home.as_str())) {
        Ok(n) => Session::open(endpoint, voice).with_noise(n),
        Err(_) => Session::open(endpoint, voice),
    };
    // One adjunction per hypha is held at a time (`Held`), so a nonce of the
    // hypha's own key names each act without a clock.
    let nonce = hypha;
    let crossing = {
        use mycelium::chronometry::tuniversal::{Origin, TPoint};
        let at = TPoint { origin: Origin { genesis: hypha, epoch: 0 }, h: 0, lanes: [0, 0, 0, 0].into(), k: 0 };
        mycelium_sdk::forms::Crossing::still(mycelium_sdk::forms::Place { at, parents: Vec::new() }, vec![1])
    };
    let act = if retract {
        Retraction { hypha, nonce }.sign(crossing, &key).and_then(|s| encode_retraction(&s))
    } else {
        let zero = num_rational::BigRational::from_integer(0.into());
        let tethers: Option<Vec<TetherStated>> = rest
            .iter()
            .skip(2)
            .map(|h| hex32(h).map(|node| TetherStated { node, g: mycelium::transducer::element::identity_versor(), planes: vec![(zero.clone(), zero.clone())] }))
            .collect();
        let Some(tethers) = tethers else {
            eprintln!("a node is 64 hex digits");
            return ExitCode::from(2);
        };
        Adjunction { hypha, tethers, nonce }.sign(crossing, &key).and_then(|s| encode_adjunction(&s))
    };
    let Some(act) = act else {
        eprintln!("the act does not write");
        return ExitCode::from(1);
    };
    match session.speak(act).map(|r| decode_cover_reply(&r)) {
        Ok(Some(c)) => println!("{c:?}"),
        Ok(None) => {
            eprintln!("the reply does not decode");
            return ExitCode::from(1);
        }
        Err(e) => {
            eprintln!("{endpoint}: {e}");
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
