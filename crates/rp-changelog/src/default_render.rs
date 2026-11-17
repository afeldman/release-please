//! Built-in markdown layout (no template engine).

use crate::model::{CommitView, TemplateContext};

/// Entry body without the trailing newline handling (caller trims).
pub fn render(ctx: &TemplateContext) -> String {
    let mut out = format!("{}\n", heading(ctx));
    if !ctx.breaking.is_empty() {
        out.push_str("\n### ⚠ BREAKING CHANGES\n\n");
        for b in &ctx.breaking {
            out.push_str(&format!(
                "* {}{}\n",
                scope_prefix(b.scope.as_deref()),
                b.description
            ));
        }
    }
    for section in &ctx.sections {
        out.push_str(&format!("\n### {}\n\n", section.title));
        for commit in &section.commits {
            out.push_str(&commit_line(commit));
            out.push('\n');
        }
    }
    out
}

fn heading(ctx: &TemplateContext) -> String {
    match &ctx.compare_url {
        Some(url) => format!("## [{}]({}) ({})", ctx.version, url, ctx.date),
        None => format!("## {} ({})", ctx.version, ctx.date),
    }
}

fn scope_prefix(scope: Option<&str>) -> String {
    match scope {
        Some(s) if !s.is_empty() => format!("**{s}:** "),
        _ => String::new(),
    }
}

fn commit_line(commit: &CommitView) -> String {
    let reference = match &commit.link {
        Some(link) => format!(" ([{}]({}))", commit.short_sha, link),
        None => format!(" ({})", commit.short_sha),
    };
    format!(
        "* {}{}{}",
        scope_prefix(commit.scope.as_deref()),
        commit.description,
        reference
    )
}
