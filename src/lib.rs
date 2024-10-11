//! detox is a tool to clean file names.

#![warn(missing_docs)]

use std::{
    ffi::{OsStr, OsString},
    fs::DirEntry,
    path::PathBuf,
};

#[derive(Debug)]
/// Contains informations about verbosity options
pub struct VerbosityFields {
    /// if true, the program will print json
    pub json: bool,
    /// if true, the program will print json with pretty print
    pub json_pretty: bool,
    /// if true, the program will only print errors as json
    pub json_error: bool,
    /// verbosity of the program
    pub verbose: bool,
}

#[derive(Debug)]
/// contains informations about cleaning options
pub struct OptionsFields {
    /// if true, the program will not rename files
    pub dry_run: bool,
}

#[derive(Debug)]
/// Options for the program
pub struct OptionnalFields {
    /// if true, the program will not rename files
    pub options: OptionsFields,
    /// contains informations about JsonFields
    pub verbosity: VerbosityFields,
}

impl PartialEq<OptionnalFields> for OptionnalFields {
    fn eq(&self, other: &OptionnalFields) -> bool {
        self.options.dry_run == other.options.dry_run
            && self.verbosity.verbose == other.verbosity.verbose
            && self.verbosity.json == other.verbosity.json
            && self.verbosity.json_pretty == other.verbosity.json_pretty
            && self.verbosity.json_error == other.verbosity.json_error
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
/// Contains informations about a result of a single file
pub struct CustomSingleResult {
    /// path of the file
    pub path: PathBuf,
    /// if the file has been renamed, contains the new path
    pub modified: Option<PathBuf>,
    /// if the file has not been renamed, contains the error
    pub error: Option<String>,
}

fn push_underscore_if(stri: &mut String, to_push: char, condition: bool) {
    if condition {
        stri.push(to_push);
    }
}

/// Check if a vector of bytes is similar to a char
pub fn check_similar(vector: Vec<u8>, name_acc: &mut String, last_was_under: bool) -> bool {
    let vec_to_string = String::from_utf8(vector).unwrap_or("".to_string());
    for one_char in vec_to_string.chars() {
        match one_char {
            'A' | 'Ⓐ' | 'Ａ' | 'À' | 'Á' | 'Â' | 'Ầ' | 'Ấ' | 'Ẫ' | 'Ẩ' | 'Ã' | 'Ā' | 'Ă' | 'Ằ'
            | 'Ắ' | 'Ẵ' | 'Ẳ' | 'Ȧ' | 'Ǡ' | 'Ä' | 'Ǟ' | 'Ả' | 'Å' | 'Ǻ' | 'Ǎ' | 'Ȁ' | 'Ȃ' | 'Ạ'
            | 'Ậ' | 'Ặ' | 'Ḁ' | 'Ą' | 'Ⱥ' | 'Ɐ' => name_acc.push('A'),
            'Ꜳ' => name_acc.push_str("AA"),
            'Æ' | 'Ǽ' | 'Ǣ' => name_acc.push('A'),
            'Ꜵ' => name_acc.push_str("AO"),
            'Ꜷ' => name_acc.push_str("AU"),
            'Ꜹ' | 'Ꜻ' => name_acc.push_str("AV"),
            'Ꜽ' => name_acc.push_str("AY"),
            'B' | 'Ⓑ' | 'Ｂ' | 'Ḃ' | 'Ḅ' | 'Ḇ' | 'Ƀ' | 'Ƃ' | 'Ɓ' => name_acc.push('B'),
            'C' | 'Ⓒ' | 'Ｃ' | 'Ć' | 'Ĉ' | 'Ċ' | 'Č' | 'Ç' | 'Ḉ' | 'Ƈ' | 'Ȼ' | 'Ꜿ' => {
                name_acc.push('C')
            }
            'D' | 'Ⓓ' | 'Ｄ' | 'Ḋ' | 'Ď' | 'Ḍ' | 'Ḑ' | 'Ḓ' | 'Ḏ' | 'Đ' | 'Ƌ' | 'Ɗ' | 'Ɖ' | 'Ꝺ' => {
                name_acc.push('D')
            }
            'Ǳ' | 'Ǆ' => name_acc.push_str("DZ"),
            'ǲ' | 'ǅ' => name_acc.push_str("Dz"),
            'E' | 'Ⓔ' | 'Ｅ' | 'È' | 'É' | 'Ê' | 'Ề' | 'Ế' | 'Ễ' | 'Ể' | 'Ẽ' | 'Ē' | 'Ḕ' | 'Ḗ'
            | 'Ĕ' | 'Ė' | 'Ë' | 'Ẻ' | 'Ě' | 'Ȅ' | 'Ȇ' | 'Ẹ' | 'Ệ' | 'Ȩ' | 'Ḝ' | 'Ę' | 'Ḙ' | 'Ḛ'
            | 'Ɛ' | 'Ǝ' => name_acc.push('E'),
            'F' | 'Ⓕ' | 'Ｆ' | 'Ḟ' | 'Ƒ' | 'Ꝼ' => name_acc.push('F'),
            'G' | 'Ⓖ' | 'Ｇ' | 'Ǵ' | 'Ĝ' | 'Ḡ' | 'Ğ' | 'Ġ' | 'Ǧ' | 'Ģ' | 'Ǥ' | 'Ɠ' | 'Ꞡ' | 'Ᵹ'
            | 'Ꝿ' => name_acc.push('G'),
            'H' | 'Ⓗ' | 'Ｈ' | 'Ĥ' | 'Ḣ' | 'Ḧ' | 'Ȟ' | 'Ḥ' | 'Ḩ' | 'Ḫ' | 'Ħ' | 'Ⱨ' | 'Ⱶ' | 'Ɥ' => {
                name_acc.push('H')
            }
            'I' | 'Ⓘ' | 'Ｉ' | 'Ì' | 'Í' | 'Î' | 'Ĩ' | 'Ī' | 'Ĭ' | 'İ' | 'Ï' | 'Ḯ' | 'Ỉ' | 'Ǐ'
            | 'Ȉ' | 'Ȋ' | 'Ị' | 'Į' | 'Ḭ' | 'Ɨ' => name_acc.push('I'),
            'J' | 'Ⓙ' | 'Ｊ' | 'Ĵ' | 'Ɉ' => name_acc.push('J'),
            'K' | 'Ⓚ' | 'Ｋ' | 'Ḱ' | 'Ǩ' | 'Ḳ' | 'Ķ' | 'Ḵ' | 'Ƙ' | 'Ⱪ' | 'Ꝁ' | 'Ꝃ' | 'Ꝅ' | 'Ꞣ' => {
                name_acc.push('K')
            }
            'L' | 'Ⓛ' | 'Ｌ' | 'Ŀ' | 'Ĺ' | 'Ľ' | 'Ḷ' | 'Ḹ' | 'Ļ' | 'Ḽ' | 'Ḻ' | 'Ł' | 'Ƚ' | 'Ɫ'
            | 'Ⱡ' | 'Ꝉ' | 'Ꝇ' | 'Ꞁ' => name_acc.push('L'),
            'Ǉ' => name_acc.push_str("LJ"),
            'ǈ' => name_acc.push_str("Lj"),
            'M' | 'Ⓜ' | 'Ｍ' | 'Ḿ' | 'Ṁ' | 'Ṃ' | 'Ɱ' | 'Ɯ' => name_acc.push('M'),
            'N' | 'Ⓝ' | 'Ｎ' | 'Ǹ' | 'Ń' | 'Ñ' | 'Ṅ' | 'Ň' | 'Ṇ' | 'Ņ' | 'Ṋ' | 'Ṉ' | 'Ƞ' | 'Ɲ'
            | 'Ꞑ' | 'Ꞥ' => name_acc.push('N'),
            'Ǌ' => name_acc.push_str("NJ"),
            'ǋ' => name_acc.push_str("Nj"),
            'O' | 'Ⓞ' | 'Ｏ' | 'Ò' | 'Ó' | 'Ô' | 'Ồ' | 'Ố' | 'Ỗ' | 'Ổ' | 'Õ' | 'Ṍ' | 'Ȭ' | 'Ṏ'
            | 'Ō' | 'Ṑ' | 'Ṓ' | 'Ŏ' | 'Ȯ' | 'Ȱ' | 'Ö' | 'Ȫ' | 'Ỏ' | 'Ő' | 'Ǒ' | 'Ȍ' | 'Ȏ' | 'Ơ'
            | 'Ờ' | 'Ớ' | 'Ỡ' | 'Ở' | 'Ợ' | 'Ọ' | 'Ộ' | 'Ǫ' | 'Ǭ' | 'Ø' | 'Ǿ' | 'Ɔ' | 'Ɵ' | 'Ꝋ'
            | 'Ꝍ' => name_acc.push('O'),
            'Ƣ' => name_acc.push_str("OI"),
            'Ꝏ' => name_acc.push_str("OO"),
            'Ȣ' => name_acc.push_str("OU"),
            '\u{008C}' | 'Œ' => name_acc.push_str("OE"),
            '\u{009C}' | 'œ' => name_acc.push_str("oe"),
            'P' | 'Ⓟ' | 'Ｐ' | 'Ṕ' | 'Ṗ' | 'Ƥ' | 'Ᵽ' | 'Ꝑ' | 'Ꝓ' | 'Ꝕ' => {
                name_acc.push('P')
            }
            'Q' | 'Ⓠ' | 'Ｑ' | 'Ꝗ' | 'Ꝙ' | 'Ɋ' => name_acc.push('Q'),
            'R' | 'Ⓡ' | 'Ｒ' | 'Ŕ' | 'Ṙ' | 'Ř' | 'Ȑ' | 'Ȓ' | 'Ṛ' | 'Ṝ' | 'Ŗ' | 'Ṟ' | 'Ɍ' | 'Ɽ'
            | 'Ꝛ' | 'Ꞧ' | 'Ꞃ' => name_acc.push('R'),
            'S' | 'Ⓢ' | 'Ｓ' | 'ẞ' | 'Ś' | 'Ṥ' | 'Ŝ' | 'Ṡ' | 'Š' | 'Ṧ' | 'Ṣ' | 'Ṩ' | 'Ș' | 'Ş'
            | 'Ȿ' | 'Ꞩ' | 'Ꞅ' => name_acc.push('S'),
            'T' | 'Ⓣ' | 'Ｔ' | 'Ṫ' | 'Ť' | 'Ṭ' | 'Ț' | 'Ţ' | 'Ṱ' | 'Ṯ' | 'Ŧ' | 'Ƭ' | 'Ʈ' | 'Ⱦ'
            | 'Ꞇ' => name_acc.push('T'),
            'Ꜩ' => name_acc.push_str("TZ"),
            'U' | 'Ⓤ' | 'Ｕ' | 'Ù' | 'Ú' | 'Û' | 'Ũ' | 'Ṹ' | 'Ū' | 'Ṻ' | 'Ŭ' | 'Ü' | 'Ǜ' | 'Ǘ'
            | 'Ǖ' | 'Ǚ' | 'Ủ' | 'Ů' | 'Ű' | 'Ǔ' | 'Ȕ' | 'Ȗ' | 'Ư' | 'Ừ' | 'Ứ' | 'Ữ' | 'Ử' | 'Ự'
            | 'Ụ' | 'Ṳ' | 'Ų' | 'Ṷ' | 'Ṵ' | 'Ʉ' => name_acc.push('U'),
            'V' | 'Ⓥ' | 'Ｖ' | 'Ṽ' | 'Ṿ' | 'Ʋ' | 'Ꝟ' | 'Ʌ' => name_acc.push('V'),
            'Ꝡ' => name_acc.push_str("VY"),
            'W' | 'Ⓦ' | 'Ｗ' | 'Ẁ' | 'Ẃ' | 'Ŵ' | 'Ẇ' | 'Ẅ' | 'Ẉ' | 'Ⱳ' => {
                name_acc.push('W')
            }
            'X' | 'Ⓧ' | 'Ｘ' | 'Ẋ' | 'Ẍ' => name_acc.push('X'),
            'Y' | 'Ⓨ' | 'Ｙ' | 'Ỳ' | 'Ý' | 'Ŷ' | 'Ỹ' | 'Ȳ' | 'Ẏ' | 'Ÿ' | 'Ỷ' | 'Ỵ' | 'Ƴ' | 'Ɏ'
            | 'Ỿ' => name_acc.push('Y'),
            'Z' | 'Ⓩ' | 'Ｚ' | 'Ź' | 'Ẑ' | 'Ż' | 'Ž' | 'Ẓ' | 'Ẕ' | 'Ƶ' | 'Ȥ' | 'Ɀ' | 'Ⱬ' | 'Ꝣ' => {
                name_acc.push('Z')
            }
            'a' | 'ⓐ' | 'ａ' | 'ẚ' | 'à' | 'á' | 'â' | 'ầ' | 'ấ' | 'ẫ' | 'ẩ' | 'ã' | 'ā' | 'ă'
            | 'ằ' | 'ắ' | 'ẵ' | 'ẳ' | 'ȧ' | 'ǡ' | 'ä' | 'ǟ' | 'ả' | 'å' | 'ǻ' | 'ǎ' | 'ȁ' | 'ȃ'
            | 'ạ' | 'ậ' | 'ặ' | 'ḁ' | 'ą' | 'ⱥ' | 'ɐ' => name_acc.push('a'),
            'ꜳ' => name_acc.push_str("aa"),
            'æ' | 'ǽ' | 'ǣ' => name_acc.push('a'),
            'ꜵ' => name_acc.push_str("ao"),
            'ꜷ' => name_acc.push_str("au"),
            'ꜹ' | 'ꜻ' => name_acc.push_str("av"),
            'ꜽ' => name_acc.push_str("ay"),
            'b' | 'ⓑ' | 'ｂ' | 'ḃ' | 'ḅ' | 'ḇ' | 'ƀ' | 'ƃ' | 'ɓ' | 'þ' => {
                name_acc.push('b')
            }
            'c' | 'ⓒ' | 'ｃ' | 'ć' | 'ĉ' | 'ċ' | 'č' | 'ç' | 'ḉ' | 'ƈ' | 'ȼ' | 'ꜿ' | 'ↄ' => {
                name_acc.push('c')
            }
            'd' | 'ⓓ' | 'ｄ' | 'ḋ' | 'ď' | 'ḍ' | 'ḑ' | 'ḓ' | 'ḏ' | 'đ' | 'ƌ' | 'ɖ' | 'ɗ' | 'ꝺ' => {
                name_acc.push('d')
            }
            'ǳ' | 'ǆ' => name_acc.push_str("dz"),
            'e' | 'ⓔ' | 'ｅ' | 'è' | 'é' | 'ê' | 'ề' | 'ế' | 'ễ' | 'ể' | 'ẽ' | 'ē' | 'ḕ' | 'ḗ'
            | 'ĕ' | 'ė' | 'ë' | 'ẻ' | 'ě' | 'ȅ' | 'ȇ' | 'ẹ' | 'ệ' | 'ȩ' | 'ḝ' | 'ę' | 'ḙ' | 'ḛ'
            | 'ɇ' | 'ɛ' | 'ǝ' => name_acc.push('e'),
            'f' | 'ⓕ' | 'ｆ' | 'ḟ' | 'ƒ' | 'ꝼ' => name_acc.push('f'),
            'g' | 'ⓖ' | 'ｇ' | 'ǵ' | 'ĝ' | 'ḡ' | 'ğ' | 'ġ' | 'ǧ' | 'ģ' | 'ǥ' | 'ɠ' | 'ꞡ' | 'ᵹ'
            | 'ꝿ' => name_acc.push('g'),
            'h' | 'ⓗ' | 'ｈ' | 'ĥ' | 'ḣ' | 'ḧ' | 'ȟ' | 'ḥ' | 'ḩ' | 'ḫ' | 'ẖ' | 'ħ' | 'ⱨ' | 'ⱶ'
            | 'ɥ' => name_acc.push('h'),
            'ƕ' => name_acc.push_str("hv"),
            'i' | 'ⓘ' | 'ｉ' | 'ì' | 'í' | 'î' | 'ĩ' | 'ī' | 'ĭ' | 'ï' | 'ḯ' | 'ỉ' | 'ǐ' | 'ȉ'
            | 'ȋ' | 'ị' | 'į' | 'ḭ' | 'ɨ' | 'ı' => name_acc.push('i'),
            'j' | 'ⓙ' | 'ｊ' | 'ĵ' | 'ǰ' | 'ɉ' => name_acc.push('j'),
            'k' | 'ⓚ' | 'ｋ' | 'ḱ' | 'ǩ' | 'ḳ' | 'ķ' | 'ḵ' | 'ƙ' | 'ⱪ' | 'ꝁ' | 'ꝃ' | 'ꝅ' | 'ꞣ' => {
                name_acc.push('k')
            }
            'l' | 'ⓛ' | 'ｌ' | 'ŀ' | 'ĺ' | 'ľ' | 'ḷ' | 'ḹ' | 'ļ' | 'ḽ' | 'ḻ' | 'ſ' | 'ł' | 'ƚ'
            | 'ɫ' | 'ⱡ' | 'ꝉ' | 'ꞁ' | 'ꝇ' => name_acc.push('l'),
            'ǉ' => name_acc.push_str("lj"),
            'm' | 'ⓜ' | 'ｍ' | 'ḿ' | 'ṁ' | 'ṃ' | 'ɱ' | 'ɯ' => name_acc.push('m'),
            'n' | 'ⓝ' | 'ｎ' | 'ǹ' | 'ń' | 'ñ' | 'ṅ' | 'ň' | 'ṇ' | 'ņ' | 'ṋ' | 'ṉ' | 'ƞ' | 'ɲ'
            | 'ŉ' | 'ꞑ' | 'ꞥ' => name_acc.push('n'),
            'ǌ' => name_acc.push_str("nj"),
            'o' | 'ⓞ' | 'ｏ' | 'ò' | 'ó' | 'ô' | 'ồ' | 'ố' | 'ỗ' | 'ổ' | 'õ' | 'ṍ' | 'ȭ' | 'ṏ'
            | 'ō' | 'ṑ' | 'ṓ' | 'ŏ' | 'ȯ' | 'ȱ' | 'ö' | 'ȫ' | 'ỏ' | 'ő' | 'ǒ' | 'ȍ' | 'ȏ' | 'ơ'
            | 'ờ' | 'ớ' | 'ỡ' | 'ở' | 'ợ' | 'ọ' | 'ộ' | 'ǫ' | 'ǭ' | 'ø' | 'ǿ' | 'ɔ' | 'ꝋ' | 'ꝍ'
            | 'ɵ' => name_acc.push('o'),
            'ƣ' => name_acc.push_str("oi"),
            'ȣ' => name_acc.push_str("ou"),
            'ꝏ' => name_acc.push_str("oo"),
            'p' | 'ⓟ' | 'ｐ' | 'ṕ' | 'ṗ' | 'ƥ' | 'ᵽ' | 'ꝑ' | 'ꝓ' | 'ꝕ' => {
                name_acc.push('p')
            }
            'q' | 'ⓠ' | 'ｑ' | 'ɋ' | 'ꝗ' | 'ꝙ' => name_acc.push('q'),
            'r' | 'ⓡ' | 'ｒ' | 'ŕ' | 'ṙ' | 'ř' | 'ȑ' | 'ȓ' | 'ṛ' | 'ṝ' | 'ŗ' | 'ṟ' | 'ɍ' | 'ɽ'
            | 'ꝛ' | 'ꞧ' | 'ꞃ' => name_acc.push('r'),
            's' | 'ⓢ' | 'ｓ' | 'ß' | 'ś' | 'ṥ' | 'ŝ' | 'ṡ' | 'š' | 'ṧ' | 'ṣ' | 'ṩ' | 'ș' | 'ş'
            | 'ȿ' | 'ꞩ' | 'ꞅ' | 'ẛ' => name_acc.push('s'),
            't' | 'ⓣ' | 'ｔ' | 'ṫ' | 'ẗ' | 'ť' | 'ṭ' | 'ț' | 'ţ' | 'ṱ' | 'ṯ' | 'ŧ' | 'ƭ' | 'ʈ'
            | 'ⱦ' | 'ꞇ' => name_acc.push('t'),
            'ꜩ' => name_acc.push_str("tz"),
            'u' | 'ⓤ' | 'ｕ' | 'ù' | 'ú' | 'û' | 'ũ' | 'ṹ' | 'ū' | 'ṻ' | 'ŭ' | 'ü' | 'ǜ' | 'ǘ'
            | 'ǖ' | 'ǚ' | 'ủ' | 'ů' | 'ű' | 'ǔ' | 'ȕ' | 'ȗ' | 'ư' | 'ừ' | 'ứ' | 'ữ' | 'ử' | 'ự'
            | 'ụ' | 'ṳ' | 'ų' | 'ṷ' | 'ṵ' | 'ʉ' => name_acc.push('u'),
            'v' | 'ⓥ' | 'ｖ' | 'ṽ' | 'ṿ' | 'ʋ' | 'ꝟ' | 'ʌ' => name_acc.push('v'),
            'ꝡ' => name_acc.push_str("vy"),
            'w' | 'ⓦ' | 'ｗ' | 'ẁ' | 'ẃ' | 'ŵ' | 'ẇ' | 'ẅ' | 'ẘ' | 'ẉ' | 'ⱳ' => {
                name_acc.push('w')
            }
            'x' | 'ⓧ' | 'ｘ' | 'ẋ' | 'ẍ' => name_acc.push('x'),
            'y' | 'ⓨ' | 'ｙ' | 'ỳ' | 'ý' | 'ŷ' | 'ỹ' | 'ȳ' | 'ẏ' | 'ÿ' | 'ỷ' | 'ẙ' | 'ỵ' | 'ƴ'
            | 'ɏ' | 'ỿ' => name_acc.push('y'),
            'z' | 'ⓩ' | 'ｚ' | 'ź' | 'ẑ' | 'ż' | 'ž' | 'ẓ' | 'ẕ' | 'ƶ' | 'ȥ' | 'ɀ' | 'ⱬ' | 'ꝣ' => {
                name_acc.push('z')
            }
            '–' => {
                name_acc.push('-');
                return false;
            }
            '\u{0300}'..='\u{036F}' | '\u{1AB0}'..='\u{1AFF}' | '\u{1DC0}'..='\u{1DFF}' => {}
            _ => {
                if !last_was_under {
                    name_acc.push('_');
                }
                return true;
            }
        };
        return false;
    }
    return false;
}

fn clean_name(path: &OsStr, _options: &OptionsFields) -> OsString {
    // for each byte of the path if it's not ascii, replace it with _
    let mut new_name = String::new();
    let mut vec_grapheme = Vec::with_capacity(4);
    let mut last_was_underscore = false;
    for byte in path.as_encoded_bytes().to_owned() {
        if vec_grapheme.len() == 0 && byte < 128 {
            match byte {
                0..=44 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                46 => {
                    new_name.push('.');
                    last_was_underscore = false;
                }
                47 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                58..=64 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                91..=96 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                123..=127 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                _ => {
                    new_name.push(byte as char);
                    last_was_underscore = false;
                }
            }
        } else {
            vec_grapheme.push(byte);
            let first_byte = vec_grapheme[0];
            if first_byte >= 240 && vec_grapheme.len() == 4 {
                // four bytes grapheme
                last_was_underscore =
                    check_similar(vec_grapheme.clone(), &mut new_name, last_was_underscore);
                vec_grapheme.clear();
            } else if first_byte >= 224 && first_byte < 240 && vec_grapheme.len() == 3 {
                // three bytes grapheme
                last_was_underscore =
                    check_similar(vec_grapheme.clone(), &mut new_name, last_was_underscore);
                vec_grapheme.clear();
            } else if first_byte >= 128 && first_byte < 224 && vec_grapheme.len() == 2 {
                // two bytes grapheme
                last_was_underscore =
                    check_similar(vec_grapheme.clone(), &mut new_name, last_was_underscore);
                vec_grapheme.clear();
            }
        }
    }
    return OsString::from(new_name);
}

fn clean_path(file_path: &PathBuf, options: &OptionsFields) -> CustomSingleResult {
    let file_name = file_path.file_name();
    if file_name.is_none() {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: None,
            error: None,
        };
    }
    let file_name = file_name.unwrap();
    let cleaned_name = clean_name(file_name, &options);
    if &cleaned_name == file_name {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: None,
            error: None,
        };
    }
    let cleaned_path = file_path.with_file_name(cleaned_name);
    if options.dry_run {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: Some(cleaned_path),
            error: Some("dry-run".to_string()),
        };
    }
    let is_renamed = std::fs::rename(file_path, &cleaned_path);
    if let Err(rename_error) = is_renamed {
        return CustomSingleResult {
            path: file_path.to_path_buf(),
            modified: Some(cleaned_path),
            error: Some(rename_error.to_string()),
        };
    }
    return CustomSingleResult {
        path: file_path.to_path_buf(),
        modified: Some(cleaned_path),
        error: None,
    };
}

fn is_directory(path: &PathBuf) -> bool {
    if let Ok(metadata) = std::fs::metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}

fn is_directory_entry(entry: &DirEntry) -> bool {
    if let Ok(metadata) = entry.metadata() {
        metadata.is_dir()
    } else {
        false
    }
}

fn clean_directory(dir_path: &PathBuf, options: &OptionsFields) -> Vec<CustomSingleResult> {
    let mut dir_path = dir_path.clone();
    let mut res = Vec::new();
    let res_dir = clean_path(&dir_path, &options);
    if res_dir.modified.is_some() && !options.dry_run && res_dir.error.is_none() {
        if let Some(ref modified) = res_dir.modified {
            dir_path = modified.clone();
        }
    }
    res.push(res_dir);
    if let Ok(entries) = std::fs::read_dir(&dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if is_directory_entry(&entry) {
                    let e = clean_directory(&file_path, &options);
                    res.extend(e);
                } else {
                    let e = clean_path(&file_path, &options);
                    res.push(e);
                }
            } else {
                res.push(CustomSingleResult {
                    path: dir_path.clone(),
                    modified: None,
                    error: Some("Entry error".to_string()),
                });
            }
        }
    } else {
        res.push(CustomSingleResult {
            path: dir_path,
            modified: None,
            error: Some("Error while reading directory".to_string()),
        });
    }
    return res;
}

fn clean(path: PathBuf, options: &OptionsFields) -> Vec<CustomSingleResult> {
    if is_directory(&path) {
        return clean_directory(&path, &options);
    } else {
        let res = clean_path(&path, &options);
        return Vec::from([res]);
    }
}

fn get_path_of_dir(dir_path: &str) -> Vec<PathBuf> {
    let mut path_to_check: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                path_to_check.push(file_path)
            }
        }
    }
    return path_to_check;
}

fn show_version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    println!("detox {} by {}", &VERSION, &AUTHORS)
}

/// Parse the arguments and return the options and the paths to check
pub fn parse_args(args: Vec<String>) -> Result<(OptionnalFields, Vec<PathBuf>), i32> {
    let mut dry_run = true;
    let mut verbose = true;
    let mut json = false;
    let mut json_pretty = false;
    let mut json_error = false;
    let mut path_to_check: Vec<PathBuf> = Vec::new();
    for one_arg in &args[1..] {
        if one_arg == "-d" || one_arg == "--do" {
            dry_run = false;
        } else if one_arg == "-v" || one_arg == "--version" {
            show_version();
            return Err(1);
        } else if one_arg == "-p" || one_arg == "--json-pretty" {
            json = true;
            json_pretty = true;
            verbose = false;
        } else if one_arg == "-e" || one_arg == "--json-error" {
            json = true;
            json_error = true;
            verbose = false;
        } else if one_arg == "-j" || one_arg == "--json" {
            json = true;
            verbose = false;
        } else if one_arg == "-q" || one_arg == "--quiet" {
            verbose = false;
        } else if one_arg == "*" {
            let paths = get_path_of_dir(".");
            path_to_check.extend(paths);
        } else {
            if std::fs::metadata(one_arg).is_ok() {
                path_to_check.push(PathBuf::from(one_arg))
            } else {
                if verbose {
                    println!("Cannot find path: {}", one_arg);
                }
            }
        }
    }
    if path_to_check.len() == 0 {
        let paths = get_path_of_dir(".");
        path_to_check.extend(paths);
    }
    return Ok((
        OptionnalFields {
            options: OptionsFields { dry_run },
            verbosity: VerbosityFields {
                verbose,
                json,
                json_pretty,
                json_error,
            },
        },
        path_to_check,
    ));
}

/// Print the output of the program conforming to the options
pub fn print_output(
    options: &VerbosityFields,
    final_res: Vec<CustomSingleResult>,
) -> Result<(), i32> {
    if options.verbose {
        let len = final_res.len();
        for one_res in final_res {
            if one_res.modified.is_some() && one_res.error.is_some() {
                println!(
                    "{:?} -> {:?} : {}",
                    one_res.path,
                    one_res.modified.unwrap(),
                    one_res.error.unwrap()
                );
            } else if one_res.error.is_some() {
                println!("{:?} : {}", one_res.path, one_res.error.unwrap());
            } else if one_res.modified.is_some() {
                println!("{:?} -> {:?}", one_res.path, one_res.modified.unwrap());
            }
        }
        if len == 1 {
            println!("{} file checked", len);
        } else {
            println!("{} files checked", len);
        }
    } else if options.json {
        #[cfg(feature = "serde")]
        {
            let vec_to_json = if options.json_error {
                let mut vec_to_json: Vec<CustomSingleResult> = Vec::new();
                for one_res in final_res {
                    if one_res.error.is_some() {
                        vec_to_json.push(one_res);
                    }
                }
                vec_to_json
            } else {
                final_res
            };
            let json_string = if options.json_pretty {
                serde_json::to_string_pretty(&vec_to_json)
            } else {
                serde_json::to_string(&vec_to_json)
            };
            if let Ok(stringed) = json_string {
                println!("{}", stringed);
            } else {
                println!(r#"{{"error": "Cannot serialize result"}}"#);
                return Err(2);
            }
        }
    }
    Ok(())
}

/// Do the program, return the Vector of result
pub fn detox(
    full_options: &OptionnalFields,
    paths_to_check: Vec<PathBuf>,
) -> Vec<CustomSingleResult> {
    if full_options.verbosity.verbose {
        println!("Running with options: {:?}", &full_options);
    }
    let mut final_res = Vec::new();
    for one_path in paths_to_check {
        if full_options.verbosity.verbose {
            println!("Checking: {:?}", one_path);
        }
        let one_res = clean(one_path, &full_options.options);
        final_res.extend(one_res);
    }
    return final_res;
}

/// main function of the program: clean and print the output
pub fn detox_full(full_options: &OptionnalFields, paths_to_check: Vec<PathBuf>) -> i32 {
    let final_res = detox(&full_options, paths_to_check);
    if let Err(code) = print_output(&full_options.verbosity, final_res) {
        return code;
    }
    0
}
