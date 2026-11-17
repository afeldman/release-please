//! Custom entry rendering with minijinja.

use crate::model::TemplateContext;
use anyhow::Context as _;

/// Render `source` (a minijinja template) with `ctx` as context.
pub fn render(source: &str, ctx: &TemplateContext) -> anyhow::Result<String> {
    let env = minijinja::Environment::new();
    env.render_str(source, ctx)
        .context("rendering changelog template")
}
