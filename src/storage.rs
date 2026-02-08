pub fn save_scores(scores: &[u32; 3]) {
    let csv = format!("{},{},{}", scores[0], scores[1], scores[2]);
    save_string(SCORES_KEY, &csv);
}

pub fn load_scores() -> [u32; 3] {
    let csv = load_string(SCORES_KEY);
    parse_scores(&csv)
}

pub fn save_audio_settings(bgm_volume: f32, sfx_volume: f32) {
    let data = format!("{},{}", bgm_volume, sfx_volume);
    save_string(AUDIO_KEY, &data);
}

pub fn load_audio_settings() -> (f32, f32) {
    let data = load_string(AUDIO_KEY);
    parse_audio_settings(&data)
}

fn parse_audio_settings(csv: &str) -> (f32, f32) {
    let parts: Vec<f32> = csv
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    let bgm = parts.first().copied().unwrap_or(0.4).clamp(0.0, 1.0);
    let sfx = parts.get(1).copied().unwrap_or(1.0).clamp(0.0, 1.0);
    (bgm, sfx)
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

const SCORES_KEY: &str = "breakout_scores";
const AUDIO_KEY: &str = "breakout_audio";

// --- WASM implementation ---
#[cfg(target_arch = "wasm32")]
fn save_string(key: &str, data: &str) {
    let _ = (|| -> Option<()> {
        let storage = web_sys::window()?.local_storage().ok()??;
        storage.set_item(key, data).ok()
    })();
}

#[cfg(target_arch = "wasm32")]
fn load_string(key: &str) -> String {
    (|| -> Option<String> {
        let storage = web_sys::window()?.local_storage().ok()??;
        storage.get_item(key).ok()?
    })()
    .unwrap_or_default()
}

// --- Native implementation ---
#[cfg(not(target_arch = "wasm32"))]
fn storage_path(key: &str) -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".breakout").join(format!("{}.txt", key)))
}

#[cfg(not(target_arch = "wasm32"))]
fn save_string(key: &str, data: &str) {
    let _ = (|| -> Option<()> {
        let path = storage_path(key)?;
        std::fs::create_dir_all(path.parent()?).ok()?;
        std::fs::write(&path, data).ok()
    })();
}

#[cfg(not(target_arch = "wasm32"))]
fn load_string(key: &str) -> String {
    storage_path(key)
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

    #[test]
    fn parse_audio_valid() {
        let (bgm, sfx) = parse_audio_settings("0.5,0.75");
        assert!((bgm - 0.5).abs() < f32::EPSILON);
        assert!((sfx - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn parse_audio_empty() {
        let (bgm, sfx) = parse_audio_settings("");
        assert!((bgm - 0.4).abs() < f32::EPSILON);
        assert!((sfx - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn parse_audio_clamped() {
        let (bgm, sfx) = parse_audio_settings("2.0,-1.0");
        assert!((bgm - 1.0).abs() < f32::EPSILON);
        assert!((sfx - 0.0).abs() < f32::EPSILON);
    }
}
