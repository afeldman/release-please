//! Page walker for list endpoints (1-based `page` parameter).

const MAX_PAGES: u32 = 200;

/// Fetches pages 1, 2, ... and passes every item to `visit`. Stops when
/// `visit` returns false, when a page is shorter than `per_page`, or after
/// `MAX_PAGES` pages.
pub(crate) fn walk<T>(
    per_page: usize,
    mut fetch: impl FnMut(u32) -> anyhow::Result<Vec<T>>,
    mut visit: impl FnMut(T) -> bool,
) -> anyhow::Result<()> {
    for page in 1..=MAX_PAGES {
        let items = fetch(page)?;
        let short = items.len() < per_page;
        for item in items {
            if !visit(item) {
                return Ok(());
            }
        }
        if short {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stops_on_short_page() {
        let mut requested = Vec::new();
        let mut seen = Vec::new();
        walk(
            2,
            |page| {
                requested.push(page);
                Ok(if page == 1 { vec![1, 2] } else { vec![3] })
            },
            |item| {
                seen.push(item);
                true
            },
        )
        .unwrap();
        assert_eq!(requested, vec![1, 2]);
        assert_eq!(seen, vec![1, 2, 3]);
    }

    #[test]
    fn stops_when_visitor_returns_false() {
        let mut seen = Vec::new();
        walk(
            2,
            |page| Ok(if page == 1 { vec![1, 2] } else { vec![3, 4] }),
            |item| {
                seen.push(item);
                item < 2
            },
        )
        .unwrap();
        assert_eq!(seen, vec![1, 2]);
    }

    #[test]
    fn propagates_fetch_error() {
        let res = walk(
            2,
            |_| -> anyhow::Result<Vec<u8>> { anyhow::bail!("boom") },
            |_| true,
        );
        assert!(res.is_err());
    }
}
