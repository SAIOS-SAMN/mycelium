// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-DERIVATIONS D2.5 (a) — THE PROCESS'S MEMORY, `M_proc`: the cgroup v2
//! `memory.max` this process runs under, or `MemAvailable` where no limit is set.

use super::calibration::Uncalibrated;

/// THE SIZE OF ONE `kB` OF `/proc/meminfo`, in bytes.
/// EXTERNAL: proc(5), `/proc/meminfo` — every size is written in kB, the kibibyte of 1024 bytes
const KIB: u64 = 1024;

/// WHICH READING `M_proc` IS: the process's own limit, or the host's
/// available memory where the process has none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemorySource {
    /// The least `memory.max` on the process's cgroup v2 path, in bytes.
    CgroupMax,
    /// `1024 · MemAvailable_kB` of `/proc/meminfo`: no cgroup limit is set.
    MemAvailable,
}

impl MemorySource {
    /// The file the reading was taken from, as the boot line names it.
    pub fn file(&self) -> &'static str {
        match self {
            MemorySource::CgroupMax => "cgroup memory.max",
            MemorySource::MemAvailable => "MemAvailable",
        }
    }
}

/// `M_proc`: the bytes this process may hold, and which reading said so.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Memory {
    bytes: std::num::NonZeroU64,
    source: MemorySource,
}

impl Memory {
    /// A memory of `bytes`, read from `source`.
    pub fn of(bytes: std::num::NonZeroU64, source: MemorySource) -> Memory {
        Memory { bytes, source }
    }

    /// `M_proc` in bytes.
    pub fn bytes(&self) -> std::num::NonZeroU64 {
        self.bytes
    }

    /// The reading `M_proc` was taken from.
    pub fn source(&self) -> MemorySource {
        self.source
    }

    /// Whether `held` bytes fit `M_proc`; a count past the carrier does not.
    pub fn holds(&self, held: Option<u64>) -> bool {
        held.is_some_and(|h| h <= self.bytes.get())
    }
}

/// THE LEAST `memory.max` ON A CGROUP V2 PATH: `membership` is
/// `/proc/self/cgroup`, whose unified-hierarchy line is `0::<path>`; the
/// limit of `<path>` and of each ancestor is read through `max_at` (the
/// contents of `<root><dir>/memory.max`), and the least numeric one binds the
/// process. `None` where the process is in no unified hierarchy or every
/// level reads `max` (no limit).
pub fn cgroup_limit_of(membership: Option<&str>, max_at: impl Fn(&str) -> Option<String>) -> Option<u64> {
    let path = membership?.lines().find_map(|l| l.strip_prefix("0::"))?.trim();
    let mut dir = path.trim_end_matches('/').to_string();
    let mut least: Option<u64> = None;
    loop {
        if let Some(limit) = max_at(&dir).and_then(|s| s.trim().parse::<u64>().ok()) {
            least = Some(least.map_or(limit, |l| l.min(limit)));
        }
        match dir.rfind('/') {
            Some(cut) => dir.truncate(cut),
            None => return least,
        }
    }
}

/// `M_proc` FROM WHAT THE PROCESS AND THE HOST SAY: the cgroup limit where
/// one is set, else `1024 · MemAvailable_kB` exactly; a process that says
/// neither, says none, or says past the carrier is `Uncalibrated::Memory`,
/// never run on a guess.
pub fn memory_of(limit: Option<u64>, available_kib: Option<u64>) -> Result<Memory, Uncalibrated> {
    if let Some(bytes) = limit {
        return std::num::NonZeroU64::new(bytes).map(|b| Memory::of(b, MemorySource::CgroupMax)).ok_or(Uncalibrated::Memory);
    }
    available_kib
        .and_then(|k| k.checked_mul(KIB))
        .and_then(std::num::NonZeroU64::new)
        .map(|b| Memory::of(b, MemorySource::MemAvailable))
        .ok_or(Uncalibrated::Memory)
}

/// The cgroup v2 limit this process runs under, read from
/// `/proc/self/cgroup` and the unified hierarchy at `/sys/fs/cgroup`.
pub(super) fn read_cgroup_limit() -> Option<u64> {
    let membership = std::fs::read_to_string("/proc/self/cgroup").ok();
    cgroup_limit_of(membership.as_deref(), |dir| std::fs::read_to_string(format!("/sys/fs/cgroup{dir}/memory.max")).ok())
}

/// `MemAvailable_kB`, or the host's silence: `None` where `/proc/meminfo`
/// does not read or carries no `MemAvailable:` line. The field is the
/// second word of that line, which is the line's format and not a member
/// picked by position (rule 15).
pub(super) fn read_mem_available_kib() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/meminfo").ok()?;
    let line = s.lines().find(|l| l.starts_with("MemAvailable:"))?;
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — wire tag offset: the value is the second word of the `MemAvailable:` line, which is that line's format"
    )]
    let kb: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb)
}
