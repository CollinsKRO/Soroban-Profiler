use owo_colors::OwoColorize;

use crate::limits::SorobanLimits;

#[derive(serde::Deserialize)]
pub struct CostRecord {
    pub label: String,
    pub cpu_instructions: u64,
    pub memory_bytes: u64,
}

pub fn print_report(records: &[CostRecord], limits: &SorobanLimits) {
    let mut entries: Vec<(&CostRecord, f64, f64)> = records
        .iter()
        .map(|r| {
            let cpu_pct = r.cpu_instructions as f64 / limits.max_cpu_instructions as f64 * 100.0;
            let mem_pct = r.memory_bytes as f64 / limits.max_memory_bytes as f64 * 100.0;
            (r, cpu_pct, mem_pct)
        })
        .collect();

    // Sort worst-first (highest single-dimension percentage first).
    entries.sort_by(|a, b| {
        let a_max = a.1.max(a.2);
        let b_max = b.1.max(b.2);
        b_max.partial_cmp(&a_max).unwrap_or(std::cmp::Ordering::Equal)
    });

    let max_label = entries
        .iter()
        .map(|(r, _, _)| r.label.len())
        .max()
        .unwrap_or(6)
        .max(6);

    println!(
        "  {:<w$}  {:>14}  {:>10}  {:>14}  {:>10}",
        "label".underline(),
        "cpu".underline(),
        "cpu %".underline(),
        "mem".underline(),
        "mem %".underline(),
        w = max_label,
    );

    for (rec, cpu_pct, mem_pct) in &entries {
        let cpu_str = format!("{cpu_pct:.1}%");
        let mem_str = format!("{mem_pct:.1}%");

        println!(
            "  {:<w$}  {:>14}  {:>10}  {:>14}  {:>10}",
            rec.label,
            rec.cpu_instructions,
            colorize_pct(*cpu_pct, &cpu_str),
            rec.memory_bytes,
            colorize_pct(*mem_pct, &mem_str),
            w = max_label,
        );
    }
}

fn colorize_pct(pct: f64, text: &str) -> String {
    if pct > 85.0 {
        text.red().to_string()
    } else if pct >= 60.0 {
        text.yellow().to_string()
    } else {
        text.green().to_string()
    }
}
