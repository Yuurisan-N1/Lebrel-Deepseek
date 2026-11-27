use rand::Rng;

pub struct Device {
    pub user_agent: String,
    pub device_id: String,
    pub platform: String,
    pub locale: String,
    pub timezone: String,
    pub screen: String,
    pub model: String,
    pub browser: String,
}

const AGENTS: [&str; 8] = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
];

const PLATFORMS: [&str; 4] = ["Win32", "MacIntel", "Linux x86_64", "Win32"];
const LOCALES: [&str; 6] = ["en-US", "en-GB", "id-ID", "de-DE", "pt-BR", "ja-JP"];
const ZONES: [&str; 6] = [
    "Asia/Jakarta",
    "Asia/Singapore",
    "Europe/Berlin",
    "America/Sao_Paulo",
    "Asia/Tokyo",
    "America/New_York",
];
const SCREENS: [&str; 5] = ["1920x1080", "1536x864", "1440x900", "2560x1440", "1366x768"];
const MODELS: [&str; 4] = ["desktop", "laptop", "desktop", "laptop"];
const BROWSERS: [&str; 3] = ["chrome", "firefox", "safari"];

impl Device {
    pub fn generate() -> Device {
        let mut rng = rand::thread_rng();
        let agent = AGENTS[rng.gen_range(0..AGENTS.len())].to_string();
        Device {
            browser: pick_browser(&agent),
            user_agent: agent,
            device_id: random_hex(&mut rng, 16),
            platform: PLATFORMS[rng.gen_range(0..PLATFORMS.len())].to_string(),
            locale: LOCALES[rng.gen_range(0..LOCALES.len())].to_string(),
            timezone: ZONES[rng.gen_range(0..ZONES.len())].to_string(),
            screen: SCREENS[rng.gen_range(0..SCREENS.len())].to_string(),
            model: MODELS[rng.gen_range(0..MODELS.len())].to_string(),
        }
    }
}

fn pick_browser(agent: &str) -> String {
    if agent.contains("Firefox") {
        return BROWSERS[1].to_string();
    }
    if agent.contains("Safari") && !agent.contains("Chrome") {
        return BROWSERS[2].to_string();
    }
    BROWSERS[0].to_string()
}

fn random_hex<R: Rng>(rng: &mut R, len: usize) -> String {
    let mut out = String::with_capacity(len * 2);
    for _ in 0..len {
        out.push_str(&format!("{:02x}", rng.gen::<u8>()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_device_has_a_full_profile() {
        let device = Device::generate();
        assert!(!device.user_agent.is_empty());
        assert_eq!(device.device_id.len(), 32);
        assert!(!device.locale.is_empty());
        assert!(!device.timezone.is_empty());
        assert!(!device.screen.is_empty());
    }

    #[test]
    fn device_ids_differ_between_accounts() {
        let first = Device::generate();
        let second = Device::generate();
        assert_ne!(first.device_id, second.device_id);
    }

    #[test]
    fn the_browser_matches_the_agent() {
        let device = Device::generate();
        if device.user_agent.contains("Firefox") {
            assert_eq!(device.browser, "firefox");
        }
    }
}
