// 16.10 — THE ROUTER: the XOR overlay that finds a node by identity.
//
//! This is the one place XOR distance is the right measure, because what is
//! being found is a KEY HOLDER and not a word's home. A word's home is the
//! Cover (16.11), and the two are never confused.


#[derive(Default)]
pub struct Router {
    children: [Option<Box<Router>>; 2],
    leaf: Option<(Vec<u8>, String)>,
}

const XOR_BITS: usize = 256;

fn xor_bit(id: &[u8], i: usize) -> usize {
    let byte = i / 8;
    if byte >= id.len() {
        return 0;
    }
    ((id[byte] >> (7 - (i % 8))) & 1) as usize
}

impl Router {
    pub fn new() -> Self {
        Router::default()
    }

    pub fn insert(&mut self, id: impl AsRef<[u8]>, addr: String) {
        let id = id.as_ref().to_vec();
        let mut node = self;
        for depth in 0..XOR_BITS {
            let b = xor_bit(&id, depth);
            node = node.children[b].get_or_insert_with(|| Box::new(Router::new()));
        }
        node.leaf = Some((id, addr));
    }

    /// The contact XOR-closest to `key` — O(bits), never O(N). A chart
    /// Address walks the same trie as a verifying key; bits past the key
    /// are 0, never a digest of the key.
    pub fn closest(&self, key: &[u8]) -> Option<(Vec<u8>, String)> {
        self.near(key, 0, &[])
    }

    /// The contact XOR-closest to `key` among those not excluded — the next
    /// hop on the nerve when the closest is the speaker itself or the one
    /// it heard from. Deterministic: the same key walks the same path from
    /// every node.
    pub fn closest_excluding(&self, key: &[u8], exclude: &[[u8; 32]]) -> Option<(Vec<u8>, String)> {
        self.near(key, 0, exclude)
    }

    /// Does the router hold this exact identity — a peer it was configured
    /// with, the only kind whose speech the nerve accepts?
    pub fn knows(&self, id: &[u8]) -> bool {
        self.closest(id).is_some_and(|(held, _)| held == id)
    }

    fn near(&self, key: &[u8], depth: usize, exclude: &[[u8; 32]]) -> Option<(Vec<u8>, String)> {
        if depth == XOR_BITS {
            return self.leaf.clone().filter(|(id, _)| {
                exclude.iter().all(|e| e.as_slice() != id.as_slice())
            });
        }
        let b = xor_bit(key, depth);
        if let Some(c) = &self.children[b] {
            if let Some(hit) = c.near(key, depth + 1, exclude) {
                return Some(hit);
            }
        }
        self.children[1 - b].as_ref().and_then(|c| c.near(key, depth + 1, exclude))
    }
}
