pub mod string_ext;

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;
use dashmap::DashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json;

pub static LOCALIZATION: LazyLock<RwLock<Localization>> = LazyLock::new(|| Localization::from(LocLang::English).into());

static DE: &'static str = include_str!("../localization/de.json");
static EN: &'static str = include_str!("../localization/en.json");
static ES: &'static str = include_str!("../localization/es.json");
static FR: &'static str = include_str!("../localization/fr.json");
static IT: &'static str = include_str!("../localization/it.json");
static JA: &'static str = include_str!("../localization/ja.json");
static KO: &'static str = include_str!("../localization/ko.json");
static RU: &'static str = include_str!("../localization/ru.json");
static NL: &'static str = include_str!("../localization/nl.json");
static ZH: &'static str = include_str!("../localization/zh.json");
static AR: &'static str = include_str!("../localization/ar.json");
static VI: &'static str = include_str!("../localization/vi.json");

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum LocLang {
    English,
    Dutch,
    French,
    German,
    Italian,
    Japanese,
    Korean,
    Russian,
    SimpleChinese,
    Spanish,
    Arabic,
    Vietnamese,
}

impl Default for LocLang {
    /// 根据系统语言选择界面默认语言，不支持或无法读取时使用英语。
    fn default() -> Self {
        Self::from_locale(sys_locale::get_locale().as_deref())
    }
}

impl From<LocLang> for &str {
    fn from(value: LocLang) -> Self {
        match value {
            LocLang::English => "English",
            LocLang::Dutch => "Nederlands",
            LocLang::French => "Français",
            LocLang::German => "Deutsch",
            LocLang::Italian => "Italiano",
            LocLang::Japanese => "日本語",
            LocLang::Korean => "한국어 [韓國語]",
            LocLang::Russian => "Русский язык",
            LocLang::SimpleChinese => "中文",
            LocLang::Spanish => "Español",
            LocLang::Arabic => "العربية",
            LocLang::Vietnamese => "Tiếng Việt",
        }
    }
}

impl std::fmt::Display for LocLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl LocLang {
    /// 将系统语言标识映射到现有翻译，兼容区域、脚本及 POSIX 后缀。
    fn from_locale(locale: Option<&str>) -> Self {
        let Some(locale) = locale else {
            log::warn!("System UI language unavailable; falling back to English");
            return Self::English;
        };
        let code = locale.split(['-', '_', '.', '@']).next().unwrap_or("");
        let language = match code.to_ascii_lowercase().as_str() {
            "en" => Self::English,
            "nl" => Self::Dutch,
            "fr" => Self::French,
            "de" => Self::German,
            "it" => Self::Italian,
            "ja" => Self::Japanese,
            "ko" => Self::Korean,
            "ru" => Self::Russian,
            // 当前中文翻译仅有简体，所有中文区域共用现有翻译。
            "zh" => Self::SimpleChinese,
            "es" => Self::Spanish,
            "ar" => Self::Arabic,
            "vi" => Self::Vietnamese,
            _ => {
                log::warn!(
                    "Unsupported system UI language {:?}; falling back to English",
                    locale.chars().take(64).collect::<String>()
                );
                return Self::English;
            }
        };
        log::debug!("Detected system UI language: {language}");
        language
    }

    pub fn iter() -> std::slice::Iter<'static, Self> {
        [
            Self::English,
            Self::Dutch,
            Self::French,
            Self::German,
            Self::Italian,
            Self::Japanese,
            Self::Korean,
            Self::Russian,
            Self::SimpleChinese,
            Self::Spanish,
            Self::Arabic,
            Self::Vietnamese,
        ]
            .iter()
    }

    #[inline(always)]
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}

pub struct Localization {
    pub language: LocLang,
    strings: DashMap<&'static str, Cow<'static, str>>,
    strings_default: DashMap<&'static str, Cow<'static, str>>,
}

impl<'a> From<LocLang> for Localization {
    fn from(value: LocLang) -> Self {
        Self {
            strings: match value {
                LocLang::English => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&EN)
                    .expect("Invalid English localization")
                    .into_iter()
                    .collect(),
                LocLang::Dutch => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&NL)
                    .expect("Invalid Dutch localization")
                    .into_iter()
                    .collect(),
                LocLang::French => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&FR)
                    .expect("Invalid French localization")
                    .into_iter()
                    .collect(),
                LocLang::German => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&DE)
                    .expect("Invalid German localization")
                    .into_iter()
                    .collect(),
                LocLang::Italian => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&IT)
                    .expect("Invalid Italian localization")
                    .into_iter()
                    .collect(),
                LocLang::Japanese => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&JA)
                    .expect("Invalid Japanese localization")
                    .into_iter()
                    .collect(),
                LocLang::Korean => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&KO)
                    .expect("Invalid Korean localization")
                    .into_iter()
                    .collect(),
                LocLang::Russian => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&RU)
                    .expect("Invalid Russian localization")
                    .into_iter()
                    .collect(),
                LocLang::SimpleChinese => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&ZH)
                    .expect("Invalid SimpleChinese localization")
                    .into_iter()
                    .collect(),
                LocLang::Spanish => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&ES)
                    .expect("Invalid Spanish localization")
                    .into_iter()
                    .collect(),
                LocLang::Arabic => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&AR)
                    .expect("Invalid Arabic localization")
                    .into_iter()
                    .collect(),
                LocLang::Vietnamese => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&VI)
                    .expect("Invalid Vietnamese localization")
                    .into_iter()
                    .collect(),
            },
            language: value,
            strings_default: serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&EN)
                .expect("Invalid English localization")
                .into_iter()
                .collect()
        }
    }
}

impl Localization {
    pub fn get(&self, key: &'static str) -> Cow<'static, str> {
        self.strings.get(&key)
            .map(|v| v.clone())
            .unwrap_or_else(|| self.strings_default.get(&key)
                .map(|v| v.clone())
                .unwrap_or(key.into()))
    }

    pub fn update_language(&mut self, lang: &LocLang) {
        self.strings = match lang {
            LocLang::English => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&EN)
                .expect("Invalid English localization")
                .into_iter()
                .collect(),
            LocLang::Dutch => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&NL)
                .expect("Invalid Dutch localization")
                .into_iter()
                .collect(),
            LocLang::French => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&FR)
                .expect("Invalid French localization")
                .into_iter()
                .collect(),
            LocLang::German => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&DE)
                .expect("Invalid German localization")
                .into_iter()
                .collect(),
            LocLang::Italian => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&IT)
                .expect("Invalid Italian localization")
                .into_iter()
                .collect(),
            LocLang::Japanese => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&JA)
                .expect("Invalid Japanese localization")
                .into_iter()
                .collect(),
            LocLang::Korean => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&KO)
                .expect("Invalid Korean localization")
                .into_iter()
                .collect(),
            LocLang::Russian => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&RU)
                .expect("Invalid Russian localization")
                .into_iter()
                .collect(),
            LocLang::SimpleChinese => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&ZH)
                .expect("Invalid SimpleChinese localization")
                .into_iter()
                .collect(),
            LocLang::Spanish => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&ES)
                .expect("Invalid Spanish localization")
                .into_iter()
                .collect(),
            LocLang::Arabic => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&AR)
                .expect("Invalid Arabic localization")
                .into_iter()
                .collect(),
            LocLang::Vietnamese => serde_json::from_str::<HashMap<&'static str, Cow<'static, str>>>(&VI)
                .expect("Invalid Vietnamese localization")
                .into_iter()
                .collect(),
        };
        self.language = *lang;
    }
}

#[cfg(test)]
mod tests {
    use super::LocLang;

    struct WarningLogger(std::sync::Mutex<Vec<String>>);

    impl log::Log for WarningLogger {
        /// 仅采集语言回退产生的警告，避免并行测试的调试日志干扰。
        fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
            metadata.level() == log::Level::Warn
        }

        /// 在内存中保存本次测试的有限条警告，供断言检查原始原因。
        fn log(&self, record: &log::Record<'_>) {
            if self.enabled(record.metadata()) {
                self.0.lock().unwrap().push(record.args().to_string());
            }
        }

        /// 测试日志直接保存于内存，无需刷新外部输出。
        fn flush(&self) {}
    }

    static WARNINGS: WarningLogger = WarningLogger(std::sync::Mutex::new(Vec::new()));

    /// 验证全部现有翻译均可由系统语言及区域标识匹配。
    #[test]
    fn system_locales_match_supported_languages() {
        let cases = [
            ("en-US", LocLang::English),
            ("nl_NL.UTF-8", LocLang::Dutch),
            ("fr-CA", LocLang::French),
            ("de_DE@euro", LocLang::German),
            ("it-IT", LocLang::Italian),
            ("ja-JP", LocLang::Japanese),
            ("ko-KR", LocLang::Korean),
            ("ru-RU", LocLang::Russian),
            ("zh-Hans-CN", LocLang::SimpleChinese),
            ("zh_TW.UTF-8", LocLang::SimpleChinese),
            ("ZH-hant-HK", LocLang::SimpleChinese),
            ("es-MX", LocLang::Spanish),
            ("ar-SA", LocLang::Arabic),
            ("vi-VN", LocLang::Vietnamese),
            ("zh", LocLang::SimpleChinese),
        ];
        for (locale, expected) in cases {
            assert_eq!(LocLang::from_locale(Some(locale)), expected, "{locale}");
        }
    }

    /// 验证缺失、不支持及无效语言标识使用英语回退。
    #[test]
    fn unavailable_or_unsupported_locales_fall_back_to_english() {
        log::set_logger(&WARNINGS).unwrap();
        log::set_max_level(log::LevelFilter::Warn);
        for locale in [
            None,
            Some(""),
            Some("pt-BR"),
            Some("C"),
            Some("POSIX"),
            Some("?"),
        ] {
            assert_eq!(LocLang::from_locale(locale), LocLang::English);
        }
        let warnings = WARNINGS.0.lock().unwrap();
        assert_eq!(warnings.len(), 6);
        assert!(warnings[0].contains("unavailable"));
        assert!(warnings[2].contains("Unsupported system UI language \"pt-BR\""));
        assert!(
            warnings
                .iter()
                .all(|message| message.contains("falling back to English"))
        );
    }
}
