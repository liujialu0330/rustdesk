use hbb_common::regex::Regex;
use std::ops::Deref;

#[cfg(target_os = "android")]
mod android_viewer;
#[cfg(not(target_os = "android"))]
mod ar;
#[cfg(not(target_os = "android"))]
mod be;
#[cfg(not(target_os = "android"))]
mod bg;
#[cfg(not(target_os = "android"))]
mod ca;
#[cfg(not(target_os = "android"))]
mod cn;
#[cfg(not(target_os = "android"))]
mod cs;
#[cfg(not(target_os = "android"))]
mod da;
#[cfg(not(target_os = "android"))]
mod de;
#[cfg(not(target_os = "android"))]
mod el;
#[cfg(not(target_os = "android"))]
mod en;
#[cfg(not(target_os = "android"))]
mod eo;
#[cfg(not(target_os = "android"))]
mod es;
#[cfg(not(target_os = "android"))]
mod et;
#[cfg(not(target_os = "android"))]
mod eu;
#[cfg(not(target_os = "android"))]
mod fa;
#[cfg(not(target_os = "android"))]
mod gu;
#[cfg(not(target_os = "android"))]
mod fr;
#[cfg(not(target_os = "android"))]
mod he;
#[cfg(not(target_os = "android"))]
mod hi;
#[cfg(not(target_os = "android"))]
mod hr;
#[cfg(not(target_os = "android"))]
mod hu;
#[cfg(not(target_os = "android"))]
mod id;
#[cfg(not(target_os = "android"))]
mod it;
#[cfg(not(target_os = "android"))]
mod ja;
#[cfg(not(target_os = "android"))]
mod ko;
#[cfg(not(target_os = "android"))]
mod kz;
#[cfg(not(target_os = "android"))]
mod lt;
#[cfg(not(target_os = "android"))]
mod lv;
#[cfg(not(target_os = "android"))]
mod nb;
#[cfg(not(target_os = "android"))]
mod nl;
#[cfg(not(target_os = "android"))]
mod pl;
#[cfg(not(target_os = "android"))]
mod ptbr;
#[cfg(not(target_os = "android"))]
mod ro;
#[cfg(not(target_os = "android"))]
mod ru;
#[cfg(not(target_os = "android"))]
mod sc;
#[cfg(not(target_os = "android"))]
mod sk;
#[cfg(not(target_os = "android"))]
mod sl;
#[cfg(not(target_os = "android"))]
mod sq;
#[cfg(not(target_os = "android"))]
mod sr;
#[cfg(not(target_os = "android"))]
mod sv;
#[cfg(not(target_os = "android"))]
mod th;
#[cfg(not(target_os = "android"))]
mod tr;
#[cfg(not(target_os = "android"))]
mod tw;
#[cfg(not(target_os = "android"))]
mod uk;
#[cfg(not(target_os = "android"))]
mod vi;
#[cfg(not(target_os = "android"))]
mod ta;
#[cfg(not(target_os = "android"))]
mod ge;
#[cfg(not(target_os = "android"))]
mod fi;
#[cfg(not(target_os = "android"))]
mod ml;

#[cfg(target_os = "android")]
pub const LANGS: &[(&str, &str)] = &[("zh-cn", "简体中文"), ("en", "English")];

#[cfg(not(target_os = "android"))]
pub const LANGS: &[(&str, &str)] = &[
    ("en", "English"),
    ("it", "Italiano"),
    ("fr", "Français"),
    ("de", "Deutsch"),
    ("nl", "Nederlands"),
    ("nb", "Norsk bokmål"),
    ("zh-cn", "简体中文"),
    ("zh-tw", "繁體中文"),
    ("pt", "Português"),
    ("es", "Español"),
    ("et", "Eesti keel"),
    ("eu", "Euskara"),
    ("hu", "Magyar"),
    ("bg", "Български"),
    ("be", "Беларуская"),
    ("ru", "Русский"),
    ("sk", "Slovenčina"),
    ("id", "Indonesia"),
    ("cs", "Čeština"),
    ("da", "Dansk"),
    ("eo", "Esperanto"),
    ("tr", "Türkçe"),
    ("vi", "Tiếng Việt"),
    ("pl", "Polski"),
    ("ja", "日本語"),
    ("ko", "한국어"),
    ("kz", "Қазақ"),
    ("uk", "Українська"),
    ("fa", "فارسی"),
    ("ca", "Català"),
    ("el", "Ελληνικά"),
    ("sv", "Svenska"),
    ("sq", "Shqip"),
    ("sr", "Srpski"),
    ("th", "ภาษาไทย"),
    ("sl", "Slovenščina"),
    ("ro", "Română"),
    ("lt", "Lietuvių"),
    ("lv", "Latviešu"),
    ("ar", "العربية"),
    ("he", "עברית"),
    ("hr", "Hrvatski"),
    ("sc", "Sardu"),
    ("ta", "தமிழ்"),
    ("ge", "ქართული"),
    ("fi", "Suomi"),
    ("ml", "മലയാളം"),
    ("hi", "हिंदी"),
    ("gu", "ગુજરાતી"),
];

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn translate(name: String) -> String {
    let locale = sys_locale::get_locale().unwrap_or_default();
    translate_locale(name, &locale)
}
pub fn translate_locale(name: String, locale: &str) -> String {
    let locale = locale.to_lowercase();
    let mut lang = hbb_common::config::LocalConfig::get_option("lang").to_lowercase();
    if lang.is_empty() {
        // zh_CN on Linux, zh-Hans-CN on mac, zh_CN_#Hans on Android
        if locale.starts_with("zh") {
            lang = (if locale.contains("tw") {
                "zh-tw"
            } else {
                "zh-cn"
            })
            .to_owned();
        }
    }
    if lang.is_empty() {
        lang = locale
            .split("-")
            .next()
            .map(|x| x.split("_").next().unwrap_or_default())
            .unwrap_or_default()
            .to_owned();
    }
    let lang = lang.to_lowercase();
    #[cfg(target_os = "android")]
    let m = android_viewer::T.deref();
    #[cfg(not(target_os = "android"))]
    let m = match lang.as_str() {
        "fr" => fr::T.deref(),
        "zh-cn" => cn::T.deref(),
        "it" => it::T.deref(),
        "zh-tw" => tw::T.deref(),
        "de" => de::T.deref(),
        "nb" => nb::T.deref(),
        "nl" => nl::T.deref(),
        "es" => es::T.deref(),
        "et" => et::T.deref(),
        "eu" => eu::T.deref(),
        "hu" => hu::T.deref(),
        "ru" => ru::T.deref(),
        "eo" => eo::T.deref(),
        "id" => id::T.deref(),
        "br" => ptbr::T.deref(),
        "pt" => ptbr::T.deref(),
        "tr" => tr::T.deref(),
        "cs" => cs::T.deref(),
        "da" => da::T.deref(),
        "sk" => sk::T.deref(),
        "vi" => vi::T.deref(),
        "pl" => pl::T.deref(),
        "ja" => ja::T.deref(),
        "ko" => ko::T.deref(),
        "kz" => kz::T.deref(),
        "uk" => uk::T.deref(),
        "fa" => fa::T.deref(),
        "fi" => fi::T.deref(),
        "ca" => ca::T.deref(),
        "el" => el::T.deref(),
        "sv" => sv::T.deref(),
        "sq" => sq::T.deref(),
        "sr" => sr::T.deref(),
        "th" => th::T.deref(),
        "sl" => sl::T.deref(),
        "ro" => ro::T.deref(),
        "lt" => lt::T.deref(),
        "lv" => lv::T.deref(),
        "ar" => ar::T.deref(),
        "bg" => bg::T.deref(),
        "be" => be::T.deref(),
        "he" => he::T.deref(),
        "hr" => hr::T.deref(),
        "sc" => sc::T.deref(),
        "ta" => ta::T.deref(),
        "ge" => ge::T.deref(),
        "ml" => ml::T.deref(),
        "hi" => hi::T.deref(),
        "gu" => gu::T.deref(),
        _ => en::T.deref(),
    };
    let (name, placeholder_value) = extract_placeholder(&name);
    let replace = |s: &&str| {
        let mut s = s.to_string();
        if let Some(value) = placeholder_value.as_ref() {
            s = s.replace("{}", &value);
        }
        if !crate::is_deskviewer() {
            if s.contains("Desk Viewer")
                && !name.starts_with("upgrade_deskviewer_server_pro")
                && name != "powered_by_me"
            {
                let app_name = crate::get_app_name();
                if !app_name.contains("Desk Viewer") {
                    s = s.replace("Desk Viewer", &app_name);
                } else {
                    // app_name only contains alphanumeric and hyphen.
                    const PLACEHOLDER: &str = "#A-P-P-N-A-M-E#";
                    if !s.contains(PLACEHOLDER) {
                        s = s.replace(&app_name, PLACEHOLDER);
                        s = s.replace("Desk Viewer", &app_name);
                        s = s.replace(PLACEHOLDER, &app_name);
                    } else {
                        // It's very unlikely to reach here.
                        // Skip replacement to avoid incorrect result.
                    }
                }
            }
        }
        s
    };
    if let Some(v) = m.get(&name as &str) {
        if !v.is_empty() {
            return replace(v);
        }
    }
    #[cfg(not(target_os = "android"))]
    if lang != "en" {
        if let Some(v) = en::T.get(&name as &str) {
            if !v.is_empty() {
                return replace(v);
            }
        }
    }
    replace(&name.as_str())
}

// Matching pattern is {}
// Write {value} in the UI and {} in the translation file
//
// Example:
// Write in the UI: translate("There are {24} hours in a day")
// Write in the translation file: ("There are {} hours in a day", "{} hours make up a day")
fn extract_placeholder(input: &str) -> (String, Option<String>) {
    if let Ok(re) = Regex::new(r#"\{(.*?)\}"#) {
        if let Some(captures) = re.captures(input) {
            if let Some(inner_match) = captures.get(1) {
                let name = re.replace(input, "{}").to_string();
                let value = inner_match.as_str().to_string();
                return (name, Some(value));
            }
        }
    }
    (input.to_string(), None)
}

mod test {
    #[test]
    fn test_extract_placeholders() {
        use super::extract_placeholder as f;

        assert_eq!(f(""), ("".to_string(), None));
        assert_eq!(
            f("{3} sessions"),
            ("{} sessions".to_string(), Some("3".to_string()))
        );
        assert_eq!(f(" } { "), (" } { ".to_string(), None));
        // Allow empty value
        assert_eq!(
            f("{} sessions"),
            ("{} sessions".to_string(), Some("".to_string()))
        );
        // Match only the first one
        assert_eq!(
            f("{2} times {4} makes {8}"),
            ("{} times {4} makes {8}".to_string(), Some("2".to_string()))
        );
    }
}
