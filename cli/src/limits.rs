/// Current per-transaction mainnet resource limits.
///
/// **IMPORTANT**: These values are periodically adjusted by network vote.
/// Re-check https://developers.stellar.org/docs/networks/resource-limits-fees
/// before every release.
#[allow(dead_code)]
pub struct SorobanLimits {
    pub max_cpu_instructions: u64,
    pub max_memory_bytes: u64,
    pub max_disk_read_bytes: u64,
    pub max_disk_write_bytes: u64,
    pub max_tx_size_bytes: u64,
    pub max_events_return_bytes: u64,
}

impl Default for SorobanLimits {
    fn default() -> Self {
        Self {
            max_cpu_instructions: 100_000_000,
            max_memory_bytes: 40 * 1024 * 1024, // 40 MB
            max_disk_read_bytes: 200 * 1024,    // 200 KB
            max_disk_write_bytes: 132 * 1024,   // 132 KiB
            max_tx_size_bytes: 132 * 1024,      // 132 KB
            max_events_return_bytes: 16 * 1024, // 16 KB
        }
    }
}
