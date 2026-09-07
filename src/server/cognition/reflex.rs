// 24.1 — THE REFLEX: the memory headroom the mind reads before it thinks.
//
//! Read from the machine rather than assumed, and not a clock (law 9).

/// The machine's live breath — MemAvailable from /proc/meminfo, in bytes.
/// A platform without the sensor reports half of a modest measured floor
/// rather than pretending to know (the capacity formula degrades gracefully).
#[allow(dead_code)]
pub(crate) fn mem_available_bytes() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|text| {
            text.lines().find(|l| l.starts_with("MemAvailable:")).and_then(|l| {
                l.split_whitespace().nth(1).and_then(|kb| kb.parse::<u64>().ok()).map(|kb| kb * 1024)
            })
        })
        .unwrap_or(64 * 1024 * 1024)
}
