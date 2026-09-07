use owo_colors::OwoColorize;

use crate::limits::SorobanLimits;

#[derive(serde::Deserialize)]
pub struct CostRecord {
    pub label: String,
    pub cpu_instructions: u64,
    pub memory_bytes: u64,
    pub read_entries: u32,
    pub read_bytes: u32,
    pub write_entries: u32,
    pub write_bytes: u32,
    pub events_size_bytes: u32,
}

/// Compute the percentage of the mainnet limit for each dimension.
struct Pcts {
    cpu: f64,
    mem: f64,
    reads: f64,
    writes: f64,
    events: f64,
}

fn compute_pcts(r: &CostRecord, limits: &SorobanLimits) -> Pcts {
    Pcts {
        cpu: r.cpu_instructions as f64 / limits.max_cpu_instructions as f64 * 100.0,
        mem: r.memory_bytes as f64 / limits.max_memory_bytes as f64 * 100.0,
        reads: r.read_bytes as f64 / limits.max_disk_read_bytes as f64 * 100.0,
        writes: r.write_bytes as f64 / limits.max_disk_write_bytes as f64 * 100.0,
        events: r.events_size_bytes as f64 / limits.max_events_return_bytes as f64 * 100.0,
    }
}

pub fn print_report(records: &[CostRecord], limits: &SorobanLimits) {
    let mut entries: Vec<(&CostRecord, Pcts)> = records
        .iter()
        .map(|r| (r, compute_pcts(r, limits)))
        .collect();

    // Sort worst-first: highest max-dimension percentage first.
    entries.sort_by(|a, b| {
        let a_max = [a.1.cpu, a.1.mem, a.1.reads, a.1.writes, a.1.events]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let b_max = [b.1.cpu, b.1.mem, b.1.reads, b.1.writes, b.1.events]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        b_max.partial_cmp(&a_max).unwrap_or(std::cmp::Ordering::Equal)
    });

    let w = entries
        .iter()
        .map(|(r, _)| r.label.len())
        .max()
        .unwrap_or(6)
        .max(12);

    // ── Compute section (CPU + Memory) ──
    println!(
        "  {:<w$}  {:>12}  {:>7}  {:>12}  {:>7}",
        "label".underline(),
        "cpu".underline(),
        "cpu %".underline(),
        "mem".underline(),
        "mem %".underline(),
    );

    for (rec, pcts) in &entries {
        println!(
            "  {:<w$}  {:>12}  {:>7}  {:>12}  {:>7}",
            rec.label,
            rec.cpu_instructions,
            colorize_pct(pcts.cpu),
            rec.memory_bytes,
            colorize_pct(pcts.mem),
        );
    }

    // ── I/O section (reads, writes, events) ──
    println!();
    println!(
        "  {:<w$}  {:>12}  {:>7}  {:>12}  {:>7}  {:>12}  {:>7}",
        "label".underline(),
        "read bytes".underline(),
        "rd %".underline(),
        "wr bytes".underline(),
        "wr %".underline(),
        "events".underline(),
        "evt %".underline(),
    );

    for (rec, pcts) in &entries {
        println!(
            "  {:<w$}  {:>12}  {:>7}  {:>12}  {:>7}  {:>12}  {:>7}",
            rec.label,
            rec.read_bytes,
            colorize_pct(pcts.reads),
            rec.write_bytes,
            colorize_pct(pcts.writes),
            rec.events_size_bytes,
            colorize_pct(pcts.events),
        );
    }
}

fn colorize_pct(pct: f64) -> String {
    let s = format!("{pct:.1}%");
    if pct > 85.0 {
        s.red().to_string()
    } else if pct >= 60.0 {
        s.yellow().to_string()
    } else {
        s.green().to_string()
    }
}
