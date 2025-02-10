use rand::seq::IndexedRandom;

/// Prefixes to be add to beginning of strings
pub const PREFIXES: [&'static str; 10] = [
    "<3 ",
    "0w0 ",
    "H-hewwo?? ",
    "HIIII! ",
    "Haiiii! ",
    "Huohhhh. ",
    "OWO ",
    "OwO ",
    "UwU ",
    "uwu ",
];

/// Suffixes to be add to beginning of strings
pub const SUFFIXES: [&str; 31] = [
    " :3",
    " (●´ω｀●)",
    " UwU",
    " (✿ ♡‿♡)",
    " ÙωÙ",
    " ʕʘ‿ʘʔ",
    " ʕ•̫͡•ʔ",
    " >_>",
    " ^_^",
    "..",
    " Huoh.",
    " ^-^",
    " ;_;",
    " ;-;",
    " xD",
    " x3",
    " :D",
    " :P",
    " ;3",
    " XDDD",
    ", fwendo",
    " ㅇㅅㅇ",
    " (人◕ω◕)",
    "（＾ｖ＾）",
    " x3",
    " ._.",
    " (　\"◟ \")",
    " (• o •)",
    " (；ω；)",
    " (◠‿◠✿)",
    " >_<",
];

/// All chars/words to be substituted with owo'd versions
pub const SUBSTITUTIONS: [(&str, &str); 10] = [
    ("r", "w"),
    ("l", "w"),
    ("R", "W"),
    ("L", "W"),
    // ("ow", "OwO"),
    ("no", "nu"),
    ("has", "haz"),
    ("have", "haz"),
    ("you", "uu"),
    ("the ", "da "),
    ("The ", "Da "),
];

/// Add random prefixes and suffixes from PREFIXES and SUFFIXES arrays
///
/// # Examples
///
/// ```
/// use owo::add_affixes;
/// let hewwo = add_affixes("hello".to_string());
/// ```
pub fn add_affixes(msg: String) -> String {
    let prefix = PREFIXES.choose(&mut rand::rng()).unwrap_or(&"Huohhhh. ");
    let suffix = SUFFIXES.choose(&mut rand::rng()).unwrap_or(&" UwU");

    format!("{prefix}{msg}{suffix}")
}

/// Replace chars in the original message based on the SUBSTITUTIONS array
///
/// # Examples
///
/// ```
/// use owo::{substitute, add_affixes};
/// let hewwo = substitute(&mut "hello".to_string());
/// // or 
/// let mut hewwo = add_affixes("hello".to_string());
/// let huoh = substitute(&mut hewwo);
/// ```
pub fn substitute(msg: &mut String) -> &String {
    for substiution in SUBSTITUTIONS.iter() {
        *msg = msg.replace(substiution.0, substiution.1);
    }

    msg
}

/// owo a string
///
/// # Examples
///
/// ```
/// use owo::owo;
/// 
/// println!("{}", owo::owo("Jimmy Carter wins posthumous Grammy for narrating an audiobook of his Sunday school lessons".to_string()))
/// ```
pub fn owo(msg: String) -> String {
    let mut new_msg = add_affixes(msg);
    substitute(&mut new_msg).to_owned()
}
