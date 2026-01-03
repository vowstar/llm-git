//! HTML report generation for fixture test results

use std::{fs, path::Path};

use crate::error::Result;

use super::{CompareResult, Fixture, RunResult, TestSummary};

/// Generate an HTML report from test results
pub fn generate_html_report(
   results: &[RunResult],
   fixtures: &[Fixture],
   output_path: &Path,
) -> Result<()> {
   let summary = TestSummary::from_results(results);
   let html = render_report(results, fixtures, &summary);
   fs::write(output_path, html)?;
   Ok(())
}

fn render_report(results: &[RunResult], fixtures: &[Fixture], summary: &TestSummary) -> String {
   let mut html = String::new();

   // Header
   html.push_str(&format!(
      r#"<!DOCTYPE html>
<html lang="en">
<head>
   <meta charset="UTF-8">
   <meta name="viewport" content="width=device-width, initial-scale=1.0">
   <title>llm-git Fixture Test Report</title>
   <style>
      :root {{
         --bg: #0d1117;
         --fg: #c9d1d9;
         --fg-muted: #8b949e;
         --border: #30363d;
         --bg-card: #161b22;
         --green: #3fb950;
         --red: #f85149;
         --yellow: #d29922;
         --blue: #58a6ff;
         --purple: #a371f7;
      }}
      * {{ box-sizing: border-box; margin: 0; padding: 0; }}
      body {{
         font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
         background: var(--bg);
         color: var(--fg);
         line-height: 1.6;
         padding: 2rem;
      }}
      .container {{ max-width: 1400px; margin: 0 auto; }}
      h1 {{ margin-bottom: 1rem; font-weight: 600; }}
      .summary {{
         display: flex;
         gap: 1rem;
         margin-bottom: 2rem;
         flex-wrap: wrap;
      }}
      .stat {{
         background: var(--bg-card);
         border: 1px solid var(--border);
         border-radius: 6px;
         padding: 1rem 1.5rem;
         min-width: 120px;
      }}
      .stat-value {{ font-size: 2rem; font-weight: 600; }}
      .stat-label {{ color: var(--fg-muted); font-size: 0.875rem; }}
      .stat.passed .stat-value {{ color: var(--green); }}
      .stat.failed .stat-value {{ color: var(--red); }}
      .stat.no-golden .stat-value {{ color: var(--yellow); }}
      .stat.errors .stat-value {{ color: var(--red); }}

      .fixture {{
         background: var(--bg-card);
         border: 1px solid var(--border);
         border-radius: 6px;
         margin-bottom: 1.5rem;
         overflow: hidden;
      }}
      .fixture-header {{
         padding: 1rem 1.5rem;
         border-bottom: 1px solid var(--border);
         display: flex;
         justify-content: space-between;
         align-items: center;
         cursor: pointer;
      }}
      .fixture-header:hover {{ background: rgba(255,255,255,0.03); }}
      .fixture-name {{ font-weight: 600; font-size: 1.1rem; }}
      .fixture-status {{ padding: 0.25rem 0.75rem; border-radius: 20px; font-size: 0.875rem; }}
      .fixture-status.passed {{ background: rgba(63, 185, 80, 0.15); color: var(--green); }}
      .fixture-status.failed {{ background: rgba(248, 81, 73, 0.15); color: var(--red); }}
      .fixture-status.no-golden {{ background: rgba(210, 153, 34, 0.15); color: var(--yellow); }}
      .fixture-status.error {{ background: rgba(248, 81, 73, 0.15); color: var(--red); }}

      .fixture-content {{
         display: none;
         padding: 1.5rem;
      }}
      .fixture.expanded .fixture-content {{ display: block; }}

      .comparison {{
         display: grid;
         grid-template-columns: 1fr 1fr;
         gap: 1.5rem;
      }}
      @media (max-width: 1000px) {{
         .comparison {{ grid-template-columns: 1fr; }}
      }}
      .comparison-column {{ }}
      .comparison-column h3 {{
         font-size: 0.875rem;
         color: var(--fg-muted);
         text-transform: uppercase;
         letter-spacing: 0.05em;
         margin-bottom: 0.75rem;
      }}
      .comparison-column h3.golden {{ color: var(--purple); }}
      .comparison-column h3.actual {{ color: var(--blue); }}

      .message-box {{
         background: var(--bg);
         border: 1px solid var(--border);
         border-radius: 6px;
         padding: 1rem;
         font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
         font-size: 0.875rem;
         white-space: pre-wrap;
         word-break: break-word;
      }}

      .diff-row {{
         display: flex;
         gap: 1rem;
         margin-bottom: 0.5rem;
         align-items: baseline;
      }}
      .diff-label {{
         min-width: 80px;
         font-weight: 500;
         font-size: 0.875rem;
      }}
      .diff-value {{ flex: 1; }}
      .diff-match {{ color: var(--green); }}
      .diff-mismatch {{ color: var(--red); }}
      .diff-arrow {{ color: var(--fg-muted); margin: 0 0.5rem; }}

      .details-list {{
         list-style: none;
         font-size: 0.875rem;
      }}
      .details-list li {{
         padding: 0.25rem 0;
         padding-left: 1rem;
         position: relative;
      }}
      .details-list li::before {{
         content: "•";
         position: absolute;
         left: 0;
         color: var(--fg-muted);
      }}

      .error-message {{
         background: rgba(248, 81, 73, 0.1);
         border: 1px solid var(--red);
         color: var(--red);
         padding: 1rem;
         border-radius: 6px;
         font-family: monospace;
         font-size: 0.875rem;
      }}

      .timestamp {{
         color: var(--fg-muted);
         font-size: 0.875rem;
         margin-bottom: 1rem;
      }}
   </style>
</head>
<body>
   <div class="container">
      <h1>llm-git Fixture Test Report</h1>
      <p class="timestamp">Generated: {}</p>

      <div class="summary">
         <div class="stat">
            <div class="stat-value">{}</div>
            <div class="stat-label">Total</div>
         </div>
         <div class="stat passed">
            <div class="stat-value">{}</div>
            <div class="stat-label">Passed</div>
         </div>
         <div class="stat failed">
            <div class="stat-value">{}</div>
            <div class="stat-label">Failed</div>
         </div>
         <div class="stat no-golden">
            <div class="stat-value">{}</div>
            <div class="stat-label">No Golden</div>
         </div>
         <div class="stat errors">
            <div class="stat-value">{}</div>
            <div class="stat-label">Errors</div>
         </div>
      </div>
"#,
      chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
      summary.total,
      summary.passed,
      summary.failed,
      summary.no_golden,
      summary.errors
   ));

   // Render each fixture result
   for result in results {
      let fixture = fixtures.iter().find(|f| f.name == result.name);
      html.push_str(&render_fixture_result(result, fixture));
   }

   // Footer and JS
   html.push_str(
      r"
   </div>
   <script>
      document.querySelectorAll('.fixture-header').forEach(header => {
         header.addEventListener('click', () => {
            header.parentElement.classList.toggle('expanded');
         });
      });
      // Expand failed fixtures by default
      document.querySelectorAll('.fixture.failed, .fixture.error').forEach(f => {
         f.classList.add('expanded');
      });
   </script>
</body>
</html>
",
   );

   html
}

fn render_fixture_result(result: &RunResult, fixture: Option<&Fixture>) -> String {
   let (status_class, status_text) = if result.error.is_some() {
      ("error", "Error")
   } else if let Some(ref cmp) = result.comparison {
      if cmp.passed {
         ("passed", "Passed")
      } else {
         ("failed", "Failed")
      }
   } else {
      ("no-golden", "No Golden")
   };

   let fixture_class = if result.error.is_some() || matches!(&result.comparison, Some(c) if !c.passed) {
      format!("fixture {status_class}")
   } else {
      format!("fixture {status_class}")
   };

   let mut html = format!(
      r#"
      <div class="{}">
         <div class="fixture-header">
            <span class="fixture-name">{}</span>
            <span class="fixture-status {}">{}</span>
         </div>
         <div class="fixture-content">
"#,
      fixture_class, result.name, status_class, status_text
   );

   // Error case
   if let Some(ref err) = result.error {
      html.push_str(&format!(
         r#"<div class="error-message">{}</div>"#,
         html_escape(err)
      ));
      html.push_str("</div></div>\n");
      return html;
   }

   // Comparison details
   if let Some(ref cmp) = result.comparison {
      html.push_str(&render_comparison(cmp, result, fixture));
   } else {
      // No golden - show actual output
      html.push_str(&render_actual_only(result));
   }

   html.push_str("</div></div>\n");
   html
}

fn render_comparison(cmp: &CompareResult, result: &RunResult, fixture: Option<&Fixture>) -> String {
   let mut html = String::new();

   // Type/Scope comparison row
   html.push_str(r#"<div style="margin-bottom: 1.5rem;">"#);

   // Type
   let type_class = if cmp.type_match { "diff-match" } else { "diff-mismatch" };
   if let Some(f) = fixture
      && let Some(ref golden) = f.golden {
         html.push_str(&format!(
            r#"<div class="diff-row">
               <span class="diff-label">Type:</span>
               <span class="diff-value {}">
                  {}<span class="diff-arrow">→</span>{}
               </span>
            </div>"#,
            type_class,
            golden.analysis.commit_type.as_str(),
            result.analysis.commit_type.as_str()
         ));
      }

   // Scope
   let scope_class = if cmp.scope_match { "diff-match" } else { "diff-mismatch" };
   if let Some(ref diff) = cmp.scope_diff {
      html.push_str(&format!(
         r#"<div class="diff-row">
            <span class="diff-label">Scope:</span>
            <span class="diff-value {}">{}</span>
         </div>"#,
         scope_class,
         html_escape(diff)
      ));
   } else {
      html.push_str(&format!(
         r#"<div class="diff-row">
            <span class="diff-label">Scope:</span>
            <span class="diff-value {}">{}</span>
         </div>"#,
         scope_class,
         result.analysis.scope.as_ref().map_or("(none)", |s| s.as_str())
      ));
   }

   // Detail counts
   html.push_str(&format!(
      r#"<div class="diff-row">
         <span class="diff-label">Details:</span>
         <span class="diff-value">{} golden → {} actual</span>
      </div>"#,
      cmp.golden_detail_count, cmp.actual_detail_count
   ));

   html.push_str("</div>");

   // Side-by-side comparison
   html.push_str(r#"<div class="comparison">"#);

   // Golden column
   if let Some(f) = fixture
      && let Some(ref golden) = f.golden {
         html.push_str(&format!(
            r#"<div class="comparison-column">
               <h3 class="golden">Golden (Expected)</h3>
               <div class="message-box">{}</div>
            </div>"#,
            html_escape(&golden.final_message)
         ));
      }

   // Actual column
   html.push_str(&format!(
      r#"<div class="comparison-column">
         <h3 class="actual">Actual (Current)</h3>
         <div class="message-box">{}</div>
      </div>"#,
      html_escape(&result.final_message)
   ));

   html.push_str("</div>");

   html
}

fn render_actual_only(result: &RunResult) -> String {
   format!(
      r#"<div>
         <div class="diff-row">
            <span class="diff-label">Type:</span>
            <span class="diff-value">{}</span>
         </div>
         <div class="diff-row">
            <span class="diff-label">Scope:</span>
            <span class="diff-value">{}</span>
         </div>
         <div class="diff-row">
            <span class="diff-label">Details:</span>
            <span class="diff-value">{} points</span>
         </div>
         <h3 style="margin: 1rem 0 0.5rem; color: var(--blue); font-size: 0.875rem;">Generated Message</h3>
         <div class="message-box">{}</div>
      </div>"#,
      result.analysis.commit_type.as_str(),
      result.analysis.scope.as_ref().map_or("(none)", |s| s.as_str()),
      result.analysis.details.len(),
      html_escape(&result.final_message)
   )
}

fn html_escape(s: &str) -> String {
   s.replace('&', "&amp;")
      .replace('<', "&lt;")
      .replace('>', "&gt;")
      .replace('"', "&quot;")
      .replace('\'', "&#39;")
}
