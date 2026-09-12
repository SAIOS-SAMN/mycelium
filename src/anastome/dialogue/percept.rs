// ACT_TASK and ACT_SOLVE — a task envelope out of the registry, and the
// solve that answers it.

use super::*;

/// task request: `[ACT_TASK][voice 32]`.
pub fn encode_task_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_TASK];
    out.extend_from_slice(voice);
    out
}

pub fn decode_task_request(body: &[u8]) -> Option<[u8; 32]> {
    if !act_is(body, ACT_TASK) || body.len() != 33 {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

/// work envelope: `[task_id address][nt u8][(input grid, output grid)·nt]
/// [nx u8][test-input grid·nx]` — train pairs to STUDY, test inputs to
/// SOLVE; the held answers never ride the wire. An empty body = no work.
pub fn encode_task_envelope(
    id: impl AsRef<[u8]>,
    train: &crate::transducer::family::Family,
    tests: &[Vec<Vec<u8>>],
) -> Vec<u8> {
    let mut out = Vec::new();
    let _ = put_addr(&mut out, id);
    out.push(train.len().min(255) as u8);
    for (a, b) in train.iter().take(255) {
        push_grid(&mut out, a);
        push_grid(&mut out, b);
    }
    out.push(tests.len().min(255) as u8);
    for x in tests.iter().take(255) {
        push_grid(&mut out, x);
    }
    out
}

/// A REGISTERED TASK as the registry hands it over: the family to learn
/// from and the inputs to answer, never the answers.
pub struct TaskEnvelope {
    /// The task's address — what a solve is filed under.
    pub id: Address,
    /// The TRAIN pairs: the family the rule is extracted from.
    pub train: Vec<crate::transducer::family::TrainPair>,
    /// The TEST inputs. Their outputs are what a solve claims; the registry
    /// never sends them.
    pub tests: Vec<Vec<Vec<u8>>>,
}

pub fn decode_task_envelope(body: &[u8]) -> Option<TaskEnvelope> {
    let mut p = 0usize;
    let id = take_addr(body, &mut p)?;
    let nt = *body.get(p)? as usize;
    p += 1;
    let mut train = Vec::with_capacity(nt.min(64));
    for _ in 0..nt {
        let a = read_grid(body, &mut p)?;
        let b = read_grid(body, &mut p)?;
        train.push((a, b));
    }
    let nx = *body.get(p)? as usize;
    p += 1;
    let mut tests = Vec::with_capacity(nx.min(64));
    for _ in 0..nx {
        tests.push(read_grid(body, &mut p)?);
    }
    Some(TaskEnvelope { id, train, tests })
}

/// solve submission: `[ACT_SOLVE][voice 32][task_id address][n u8][grid·n]`
/// and optionally `[RULE][len u32][element]` so the home speaks the rule.
pub fn encode_solve(voice: &[u8; 32], id: impl AsRef<[u8]>, outputs: &[Vec<Vec<u8>>]) -> Vec<u8> {
    encode_solve_with_rule(voice, id, outputs, None)
}

pub fn encode_solve_with_rule(
    voice: &[u8; 32],
    id: impl AsRef<[u8]>,
    outputs: &[Vec<Vec<u8>>],
    rule: Option<&[u8]>,
) -> Vec<u8> {
    let mut out = vec![ACT_SOLVE];
    out.extend_from_slice(voice);
    let _ = put_addr(&mut out, id);
    out.push(outputs.len().min(255) as u8);
    for g in outputs.iter().take(255) {
        push_grid(&mut out, g);
    }
    if let Some(r) = rule {
        out.extend_from_slice(b"RULE");
        out.extend_from_slice(&(r.len() as u32).to_le_bytes());
        out.extend_from_slice(r);
    }
    out
}

pub fn decode_solve(body: &[u8]) -> Option<SolveAct> {
    if !act_is(body, ACT_SOLVE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let id = take_addr(body, &mut p)?;
    let n = *body.get(p)? as usize;
    p += 1;
    let mut outputs = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        outputs.push(read_grid(body, &mut p)?);
    }
    let rule = if body.get(p..p + 4) == Some(&b"RULE"[..]) {
        p += 4;
        let len = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        Some(body.get(p..p + len)?.to_vec())
    } else {
        None
    };
    Some(SolveAct { voice, task: id, outputs, rule })
}


pub fn encode_solve_receipt(code: u8, mu: &[u64; 4]) -> Vec<u8> {
    let mut out = vec![code];
    push_mu(&mut out, mu);
    out
}

pub fn decode_solve_receipt(body: &[u8]) -> Option<(u8, [u64; 4])> {
    let code = act_tag(body)?;
    let mut p = 1usize;
    let mu = read_mu(body, &mut p)?;
    Some((code, mu))
}
