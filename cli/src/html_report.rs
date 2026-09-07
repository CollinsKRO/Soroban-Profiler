use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, Result};

use crate::limits::SorobanLimits;
use crate::report::CostRecord;

/// Write a self-contained HTML report to `path`.
///
/// The file uses only inline styles, no external assets, no JavaScript —
/// it opens correctly with zero network access.
pub fn write_html_report(records: &[CostRecord], limits: &SorobanLimits, path: &Path) -> Result<()> {
    let mut cards = String::new();

    for rec in records {
        let cpu_pct = rec.cpu_instructions as f64 / limits.max_cpu_instructions as f64 * 100.0;
        let mem_pct = rec.memory_bytes as f64 / limits.max_memory_bytes as f64 * 100.0;

        let cpu_color = bar_color(cpu_pct);
        let mem_color = bar_color(mem_pct);

        let cpu_bar_width = cpu_pct.min(100.0);
        let mem_bar_width = mem_pct.min(100.0);

        let _ = writeln!(
            cards,
            r#"    <div class="card">
      <h2>{label}</h2>
      <div class="stat">
        <span class="stat-label">CPU Instructions</span>
        <span class="stat-value">{cpu}</span>
        <span class="stat-pct" style="color:{cpu_color}">{cpu_pct:.1}%</span>
      </div>
      <div class="bar-track"><div class="bar-fill" style="width:{cpu_bar_width:.1}%;background:{cpu_color}"></div></div>
      <div class="stat">
        <span class="stat-label">Memory</span>
        <span class="stat-value">{mem}</span>
        <span class="stat-pct" style="color:{mem_color}">{mem_pct:.1}%</span>
      </div>
      <div class="bar-track"><div class="bar-fill" style="width:{mem_bar_width:.1}%;background:{mem_color}"></div></div>
    </div>"#,
            label = escape_html(&rec.label),
            cpu = rec.cpu_instructions,
            cpu_color = cpu_color,
            cpu_pct = cpu_pct,
            cpu_bar_width = cpu_bar_width,
            mem = rec.memory_bytes,
            mem_color = mem_color,
            mem_pct = mem_pct,
            mem_bar_width = mem_bar_width,
        );
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Soroban Cost Report</title>
<style>
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{
    font-family: system-ui, -apple-system, sans-serif;
    background: #0f1117;
    color: #e1e4e8;
    padding: 2rem;
  }}
  h1 {{
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: #f0f6fc;
  }}
  .card {{
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 8px;
    padding: 1.25rem 1.5rem;
    margin-bottom: 1rem;
  }}
  .card h2 {{
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: 0.75rem;
    color: #f0f6fc;
  }}
  .stat {{
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
    margin-bottom: 0.35rem;
  }}
  .stat-label {{
    font-size: 0.8rem;
    color: #8b949e;
    min-width: 10rem;
  }}
  .stat-value {{
    font-size: 0.85rem;
    font-variant-numeric: tabular-nums;
    color: #c9d1d9;
  }}
  .stat-pct {{
    font-size: 0.8rem;
    font-weight: 600;
    margin-left: auto;
  }}
  .bar-track {{
    height: 6px;
    background: #21262d;
    border-radius: 3px;
    margin-bottom: 0.75rem;
    overflow: hidden;
  }}
  .bar-fill {{
    height: 100%;
    border-radius: 3px;
    transition: width 0.3s ease;
  }}
</style>
</head>
<body>
<h1>Soroban Cost Report</h1>
{cards}
</body>
</html>"#
    );

    std::fs::write(path, html).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

/// Return a CSS color string based on the percentage thresholds.
fn bar_color(pct: f64) -> &'static str {
    if pct > 85.0 {
        "#f85149" // red
    } else if pct >= 60.0 {
        "#d29922" // yellow
    } else {
        "#3fb950" // green
    }
}

/// Minimal HTML-escape for safe interpolation into HTML text content.
fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}
