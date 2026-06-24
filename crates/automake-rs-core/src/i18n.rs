// automake-rs-core: Internationalization (i18n) — pure Rust message catalog
//
// Court: AM.I18N.1
//
// Provides translated diagnostic messages via pure-Rust message catalogs.
// No C FFI, no gettext dependency. Clean-room: message keys derived from
// black-box oracle interrogation (running GNU automake with LANG=xx_XX
// and capturing stderr output). No GNU .po files are copied.
//
// English is the built-in fallback. Additional languages load from
// locales/{lang}.json at startup. LC_MESSAGES/LANG/LC_ALL honored.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MessageCatalog {
    pub lang: String,
    pub messages: HashMap<String, String>,
}

impl MessageCatalog {
    pub fn new(lang: &str) -> Self {
        Self {
            lang: lang.to_string(),
            messages: HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(|s| s.as_str())
    }
    pub fn get_or_key(&self, key: &str) -> String {
        self.messages
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
    pub fn insert(&mut self, key: &str, value: &str) {
        self.messages.insert(key.to_string(), value.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct I18nManager {
    pub active: MessageCatalog,
    pub fallback: MessageCatalog,
    pub catalogs: HashMap<String, MessageCatalog>,
    pub enabled: bool,
}

impl I18nManager {
    pub fn new() -> Self {
        let english = build_english_catalog();
        let lang = detect_language();
        let mut manager = Self {
            active: english.clone(),
            fallback: english.clone(),
            catalogs: HashMap::new(),
            enabled: true,
        };
        manager.catalogs.insert("en".to_string(), english);
        if lang != "en" {
            if let Some(cat) = load_catalog(&lang) {
                manager.catalogs.insert(lang.clone(), cat.clone());
                manager.active = cat;
            }
        }
        if let Ok(lc) = std::env::var("LC_MESSAGES") {
            if !lc.is_empty() && lc != "C" && lc != "POSIX" {
                let code = lc.split('.').next().unwrap_or(&lc);
                if code != "en" {
                    if let Some(cat) = load_catalog(code) {
                        manager.catalogs.insert(code.to_string(), cat.clone());
                        manager.active = cat;
                    }
                }
            }
        }
        if let Ok(lc_all) = std::env::var("LC_ALL") {
            if lc_all == "C" || lc_all == "POSIX" {
                manager.enabled = false;
                manager.active = manager.fallback.clone();
            }
        }
        if let Ok(l) = std::env::var("LANG") {
            if l == "C" || l == "POSIX" {
                manager.enabled = false;
                manager.active = manager.fallback.clone();
            }
        }
        manager
    }
    pub fn translate(&self, key: &str) -> String {
        if !self.enabled {
            self.fallback.get_or_key(key)
        } else {
            self.active.get_or_key(key)
        }
    }
    pub fn translate_fmt(&self, key: &str, args: &[(&str, &str)]) -> String {
        let tpl = self.translate(key);
        let mut r = tpl;
        for (k, v) in args {
            r = r.replace(&format!("{{{}}}", k), v);
        }
        r
    }
    pub fn set_language(&mut self, lang: &str) {
        if let Some(cat) = self.catalogs.get(lang) {
            self.active = cat.clone();
        } else if let Some(cat) = load_catalog(lang) {
            self.catalogs.insert(lang.to_string(), cat.clone());
            self.active = cat;
        }
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}

fn detect_language() -> String {
    for var in &["LC_MESSAGES", "LANG", "LC_ALL"] {
        if let Ok(v) = std::env::var(var) {
            if !v.is_empty() && v != "C" && v != "POSIX" {
                return v.split('.').next().unwrap_or(&v).to_string();
            }
        }
    }
    "en".to_string()
}

fn load_catalog(lang: &str) -> Option<MessageCatalog> {
    for dir in &[
        format!("locales/{}.json", lang),
        format!("/usr/share/automake-rs/locales/{}.json", lang),
    ] {
        if let Ok(content) = std::fs::read_to_string(dir) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                let mut cat = MessageCatalog::new(lang);
                if let Some(msgs) = json.as_object() {
                    for (k, v) in msgs {
                        if let Some(s) = v.as_str() {
                            cat.insert(k, s);
                        }
                    }
                }
                return Some(cat);
            }
        }
    }
    None
}

fn build_english_catalog() -> MessageCatalog {
    let mut c = MessageCatalog::new("en");
    c.insert("gen.created", "generated automatically by automake-rs");
    c.insert("gen.error", "error");
    c.insert("gen.warning", "warning");
    c.insert("gen.note", "note");
    c.insert("missing.news", "required file 'NEWS' not found");
    c.insert("missing.readme", "required file 'README' not found");
    c.insert("missing.authors", "required file 'AUTHORS' not found");
    c.insert("missing.changelog", "required file 'ChangeLog' not found");
    c.insert("missing.copying", "required file 'COPYING' not found");
    c.insert("missing.install", "required file 'INSTALL' not found");
    c.insert(
        "primary.unknown",
        "unknown primary '{primary}' in variable '{var}'",
    );
    c.insert(
        "primary.unimplemented",
        "primary '{primary}' is not yet fully implemented in automake-rs",
    );
    c.insert("primary.duplicate", "duplicate primary '{var}'");
    c.insert(
        "variable.reserved",
        "variable '{var}' is reserved by Automake",
    );
    c.insert("syntax.bad", "bad syntax");
    c.insert("syntax.expected", "expected '{expected}', found '{found}'");
    c.insert(
        "portability.subst",
        "'{var}' contains configure substitution",
    );
    c.insert(
        "portability.recursive",
        "recursive variable '{var}' references itself",
    );
    c.insert(
        "cross.undefined",
        "'{var}' is undefined in cross-compilation mode",
    );
    c.insert("obsolete.macro", "macro '{macro}' is obsolete");
    c.insert(
        "obsolete.use",
        "use '{replacement}' instead of '{obsolete}'",
    );
    c.insert(
        "override.user",
        "user variable '{var}' overrides Automake variable",
    );
    c.insert("gnu.install", "installing '{file}'");
    c.insert(
        "gnu.missing_copyright",
        "missing copyright notice in '{file}'",
    );
    c.insert("gnits.strict", "'{var}' is required by GNITS standards");
    c
}

use std::sync::OnceLock;
static I18N: OnceLock<I18nManager> = OnceLock::new();
pub fn init_i18n() {
    I18N.get_or_init(I18nManager::new);
}
pub fn i18n() -> &'static I18nManager {
    I18N.get_or_init(I18nManager::new)
}
pub fn tr(key: &str) -> String {
    i18n().translate(key)
}
pub fn tr_fmt(key: &str, args: &[(&str, &str)]) -> String {
    i18n().translate_fmt(key, args)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_english_catalog() {
        let cat = build_english_catalog();
        assert_eq!(
            cat.get("missing.news"),
            Some("required file 'NEWS' not found")
        );
        assert_eq!(
            cat.get("primary.unknown"),
            Some("unknown primary '{primary}' in variable '{var}'")
        );
    }
    #[test]
    fn test_translate_fmt() {
        let m = I18nManager::new();
        assert_eq!(
            m.translate_fmt(
                "primary.unknown",
                &[("primary", "JAVA"), ("var", "bin_JAVA")]
            ),
            "unknown primary 'JAVA' in variable 'bin_JAVA'"
        );
    }
    #[test]
    fn test_init() {
        init_i18n();
        assert_eq!(tr("missing.news"), "required file 'NEWS' not found");
    }
}
