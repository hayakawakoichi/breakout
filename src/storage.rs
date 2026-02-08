pub fn save_scores(scores: &[u32; 3]) {
    let csv = format!("{},{},{}", scores[0], scores[1], scores[2]);
    save_string(&csv);
}

pub fn load_scores() -> [u32; 3] {
    let csv = load_string();
    parse_scores(&csv)
}

fn parse_scores(csv: &str) -> [u32; 3] {
    let parts: Vec<u32> = csv
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    [
        parts.first().copied().unwrap_or(0),
        parts.get(1).copied().unwrap_or(0),
        parts.get(2).copied().unwrap_or(0),
    ]
}

// --- WASM implementation ---
#[cfg(target_arch = "wasm32")]
const STORAGE_KEY: &str = "breakout_scores";

#[cfg(target_arch = "wasm32")]
fn save_string(data: &str) {
    let _ = (|| -> Option<()> {
        let storage = web_sys::window()?.local_storage().ok()??;
        storage.set_item(STORAGE_KEY, data).ok()
    })();
}

#[cfg(target_arch = "wasm32")]
fn load_string() -> String {
    (|| -> Option<String> {
        let storage = web_sys::window()?.local_storage().ok()??;
        storage.get_item(STORAGE_KEY).ok()?
    })()
    .unwrap_or_default()
}

// --- Native implementation ---
#[cfg(not(target_arch = "wasm32"))]
fn scores_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".breakout").join("scores.txt"))
}

#[cfg(not(target_arch = "wasm32"))]
fn save_string(data: &str) {
    let _ = (|| -> Option<()> {
        let path = scores_path()?;
        std::fs::create_dir_all(path.parent()?).ok()?;
        std::fs::write(&path, data).ok()
    })();
}

#[cfg(not(target_arch = "wasm32"))]
fn load_string() -> String {
    scores_path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_csv() {
        assert_eq!(parse_scores("100,50,20"), [100, 50, 20]);
    }

    #[test]
    fn parse_empty_string() {
        assert_eq!(parse_scores(""), [0, 0, 0]);
    }

    #[test]
    fn parse_partial_csv() {
        assert_eq!(parse_scores("100"), [100, 0, 0]);
        assert_eq!(parse_scores("100,50"), [100, 50, 0]);
    }

    #[test]
    fn parse_invalid_csv() {
        assert_eq!(parse_scores("abc,def,ghi"), [0, 0, 0]);
    }
}
