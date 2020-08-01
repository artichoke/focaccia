// This file is generated. See `gen_full_mapping.rb`.

/// Compare two strings with Full Unicode case folding for Turkic languages.
///
/// This function is implemented with a lookup table generated from Unicode case
/// folding tables.
#[must_use]
#[allow(clippy::match_same_arms)]
pub fn casecmp(left: &str, right: &str) -> bool {
    let mut left = left.chars();
    let mut right = right.chars();
    loop {
        match (left.next(), right.next()) {
            (None, None) => return true,
            (Some(_), None) | (None, Some(_)) => return false,
            (Some(left), Some(right)) if left == right => continue,
            (Some('\u{0041}'), Some('\u{0061}')) => continue,
            (Some('\u{0061}'), Some('\u{0041}')) => continue,
            (Some('\u{0042}'), Some('\u{0062}')) => continue,
            (Some('\u{0062}'), Some('\u{0042}')) => continue,
            (Some('\u{0043}'), Some('\u{0063}')) => continue,
            (Some('\u{0063}'), Some('\u{0043}')) => continue,
            (Some('\u{0044}'), Some('\u{0064}')) => continue,
            (Some('\u{0064}'), Some('\u{0044}')) => continue,
            (Some('\u{0045}'), Some('\u{0065}')) => continue,
            (Some('\u{0065}'), Some('\u{0045}')) => continue,
            (Some('\u{0046}'), Some('\u{0066}')) => continue,
            (Some('\u{0066}'), Some('\u{0046}')) => continue,
            (Some('\u{0047}'), Some('\u{0067}')) => continue,
            (Some('\u{0067}'), Some('\u{0047}')) => continue,
            (Some('\u{0048}'), Some('\u{0068}')) => continue,
            (Some('\u{0068}'), Some('\u{0048}')) => continue,
            (Some('\u{0049}'), Some('\u{0131}')) => continue,
            (Some('\u{0131}'), Some('\u{0049}')) => continue,
            (Some('\u{004A}'), Some('\u{006A}')) => continue,
            (Some('\u{006A}'), Some('\u{004A}')) => continue,
            (Some('\u{004B}'), Some('\u{006B}')) => continue,
            (Some('\u{006B}'), Some('\u{004B}')) => continue,
            (Some('\u{004C}'), Some('\u{006C}')) => continue,
            (Some('\u{006C}'), Some('\u{004C}')) => continue,
            (Some('\u{004D}'), Some('\u{006D}')) => continue,
            (Some('\u{006D}'), Some('\u{004D}')) => continue,
            (Some('\u{004E}'), Some('\u{006E}')) => continue,
            (Some('\u{006E}'), Some('\u{004E}')) => continue,
            (Some('\u{004F}'), Some('\u{006F}')) => continue,
            (Some('\u{006F}'), Some('\u{004F}')) => continue,
            (Some('\u{0050}'), Some('\u{0070}')) => continue,
            (Some('\u{0070}'), Some('\u{0050}')) => continue,
            (Some('\u{0051}'), Some('\u{0071}')) => continue,
            (Some('\u{0071}'), Some('\u{0051}')) => continue,
            (Some('\u{0052}'), Some('\u{0072}')) => continue,
            (Some('\u{0072}'), Some('\u{0052}')) => continue,
            (Some('\u{0053}'), Some('\u{0073}')) => continue,
            (Some('\u{0073}'), Some('\u{0053}')) => continue,
            (Some('\u{0054}'), Some('\u{0074}')) => continue,
            (Some('\u{0074}'), Some('\u{0054}')) => continue,
            (Some('\u{0055}'), Some('\u{0075}')) => continue,
            (Some('\u{0075}'), Some('\u{0055}')) => continue,
            (Some('\u{0056}'), Some('\u{0076}')) => continue,
            (Some('\u{0076}'), Some('\u{0056}')) => continue,
            (Some('\u{0057}'), Some('\u{0077}')) => continue,
            (Some('\u{0077}'), Some('\u{0057}')) => continue,
            (Some('\u{0058}'), Some('\u{0078}')) => continue,
            (Some('\u{0078}'), Some('\u{0058}')) => continue,
            (Some('\u{0059}'), Some('\u{0079}')) => continue,
            (Some('\u{0079}'), Some('\u{0059}')) => continue,
            (Some('\u{005A}'), Some('\u{007A}')) => continue,
            (Some('\u{007A}'), Some('\u{005A}')) => continue,
            (Some('\u{00B5}'), Some('\u{03BC}')) => continue,
            (Some('\u{03BC}'), Some('\u{00B5}')) => continue,
            (Some('\u{00C0}'), Some('\u{00E0}')) => continue,
            (Some('\u{00E0}'), Some('\u{00C0}')) => continue,
            (Some('\u{00C1}'), Some('\u{00E1}')) => continue,
            (Some('\u{00E1}'), Some('\u{00C1}')) => continue,
            (Some('\u{00C2}'), Some('\u{00E2}')) => continue,
            (Some('\u{00E2}'), Some('\u{00C2}')) => continue,
            (Some('\u{00C3}'), Some('\u{00E3}')) => continue,
            (Some('\u{00E3}'), Some('\u{00C3}')) => continue,
            (Some('\u{00C4}'), Some('\u{00E4}')) => continue,
            (Some('\u{00E4}'), Some('\u{00C4}')) => continue,
            (Some('\u{00C5}'), Some('\u{00E5}')) => continue,
            (Some('\u{00E5}'), Some('\u{00C5}')) => continue,
            (Some('\u{00C6}'), Some('\u{00E6}')) => continue,
            (Some('\u{00E6}'), Some('\u{00C6}')) => continue,
            (Some('\u{00C7}'), Some('\u{00E7}')) => continue,
            (Some('\u{00E7}'), Some('\u{00C7}')) => continue,
            (Some('\u{00C8}'), Some('\u{00E8}')) => continue,
            (Some('\u{00E8}'), Some('\u{00C8}')) => continue,
            (Some('\u{00C9}'), Some('\u{00E9}')) => continue,
            (Some('\u{00E9}'), Some('\u{00C9}')) => continue,
            (Some('\u{00CA}'), Some('\u{00EA}')) => continue,
            (Some('\u{00EA}'), Some('\u{00CA}')) => continue,
            (Some('\u{00CB}'), Some('\u{00EB}')) => continue,
            (Some('\u{00EB}'), Some('\u{00CB}')) => continue,
            (Some('\u{00CC}'), Some('\u{00EC}')) => continue,
            (Some('\u{00EC}'), Some('\u{00CC}')) => continue,
            (Some('\u{00CD}'), Some('\u{00ED}')) => continue,
            (Some('\u{00ED}'), Some('\u{00CD}')) => continue,
            (Some('\u{00CE}'), Some('\u{00EE}')) => continue,
            (Some('\u{00EE}'), Some('\u{00CE}')) => continue,
            (Some('\u{00CF}'), Some('\u{00EF}')) => continue,
            (Some('\u{00EF}'), Some('\u{00CF}')) => continue,
            (Some('\u{00D0}'), Some('\u{00F0}')) => continue,
            (Some('\u{00F0}'), Some('\u{00D0}')) => continue,
            (Some('\u{00D1}'), Some('\u{00F1}')) => continue,
            (Some('\u{00F1}'), Some('\u{00D1}')) => continue,
            (Some('\u{00D2}'), Some('\u{00F2}')) => continue,
            (Some('\u{00F2}'), Some('\u{00D2}')) => continue,
            (Some('\u{00D3}'), Some('\u{00F3}')) => continue,
            (Some('\u{00F3}'), Some('\u{00D3}')) => continue,
            (Some('\u{00D4}'), Some('\u{00F4}')) => continue,
            (Some('\u{00F4}'), Some('\u{00D4}')) => continue,
            (Some('\u{00D5}'), Some('\u{00F5}')) => continue,
            (Some('\u{00F5}'), Some('\u{00D5}')) => continue,
            (Some('\u{00D6}'), Some('\u{00F6}')) => continue,
            (Some('\u{00F6}'), Some('\u{00D6}')) => continue,
            (Some('\u{00D8}'), Some('\u{00F8}')) => continue,
            (Some('\u{00F8}'), Some('\u{00D8}')) => continue,
            (Some('\u{00D9}'), Some('\u{00F9}')) => continue,
            (Some('\u{00F9}'), Some('\u{00D9}')) => continue,
            (Some('\u{00DA}'), Some('\u{00FA}')) => continue,
            (Some('\u{00FA}'), Some('\u{00DA}')) => continue,
            (Some('\u{00DB}'), Some('\u{00FB}')) => continue,
            (Some('\u{00FB}'), Some('\u{00DB}')) => continue,
            (Some('\u{00DC}'), Some('\u{00FC}')) => continue,
            (Some('\u{00FC}'), Some('\u{00DC}')) => continue,
            (Some('\u{00DD}'), Some('\u{00FD}')) => continue,
            (Some('\u{00FD}'), Some('\u{00DD}')) => continue,
            (Some('\u{00DE}'), Some('\u{00FE}')) => continue,
            (Some('\u{00FE}'), Some('\u{00DE}')) => continue,
            (Some('\u{00DF}'), Some('\u{0073}')) if matches!(right.next(), Some('\u{0073}')) => {
                continue
            }
            (Some('\u{0073}'), Some('\u{00DF}')) if matches!(left.next(), Some('\u{0073}')) => {
                continue
            }
            (Some('\u{0100}'), Some('\u{0101}')) => continue,
            (Some('\u{0101}'), Some('\u{0100}')) => continue,
            (Some('\u{0102}'), Some('\u{0103}')) => continue,
            (Some('\u{0103}'), Some('\u{0102}')) => continue,
            (Some('\u{0104}'), Some('\u{0105}')) => continue,
            (Some('\u{0105}'), Some('\u{0104}')) => continue,
            (Some('\u{0106}'), Some('\u{0107}')) => continue,
            (Some('\u{0107}'), Some('\u{0106}')) => continue,
            (Some('\u{0108}'), Some('\u{0109}')) => continue,
            (Some('\u{0109}'), Some('\u{0108}')) => continue,
            (Some('\u{010A}'), Some('\u{010B}')) => continue,
            (Some('\u{010B}'), Some('\u{010A}')) => continue,
            (Some('\u{010C}'), Some('\u{010D}')) => continue,
            (Some('\u{010D}'), Some('\u{010C}')) => continue,
            (Some('\u{010E}'), Some('\u{010F}')) => continue,
            (Some('\u{010F}'), Some('\u{010E}')) => continue,
            (Some('\u{0110}'), Some('\u{0111}')) => continue,
            (Some('\u{0111}'), Some('\u{0110}')) => continue,
            (Some('\u{0112}'), Some('\u{0113}')) => continue,
            (Some('\u{0113}'), Some('\u{0112}')) => continue,
            (Some('\u{0114}'), Some('\u{0115}')) => continue,
            (Some('\u{0115}'), Some('\u{0114}')) => continue,
            (Some('\u{0116}'), Some('\u{0117}')) => continue,
            (Some('\u{0117}'), Some('\u{0116}')) => continue,
            (Some('\u{0118}'), Some('\u{0119}')) => continue,
            (Some('\u{0119}'), Some('\u{0118}')) => continue,
            (Some('\u{011A}'), Some('\u{011B}')) => continue,
            (Some('\u{011B}'), Some('\u{011A}')) => continue,
            (Some('\u{011C}'), Some('\u{011D}')) => continue,
            (Some('\u{011D}'), Some('\u{011C}')) => continue,
            (Some('\u{011E}'), Some('\u{011F}')) => continue,
            (Some('\u{011F}'), Some('\u{011E}')) => continue,
            (Some('\u{0120}'), Some('\u{0121}')) => continue,
            (Some('\u{0121}'), Some('\u{0120}')) => continue,
            (Some('\u{0122}'), Some('\u{0123}')) => continue,
            (Some('\u{0123}'), Some('\u{0122}')) => continue,
            (Some('\u{0124}'), Some('\u{0125}')) => continue,
            (Some('\u{0125}'), Some('\u{0124}')) => continue,
            (Some('\u{0126}'), Some('\u{0127}')) => continue,
            (Some('\u{0127}'), Some('\u{0126}')) => continue,
            (Some('\u{0128}'), Some('\u{0129}')) => continue,
            (Some('\u{0129}'), Some('\u{0128}')) => continue,
            (Some('\u{012A}'), Some('\u{012B}')) => continue,
            (Some('\u{012B}'), Some('\u{012A}')) => continue,
            (Some('\u{012C}'), Some('\u{012D}')) => continue,
            (Some('\u{012D}'), Some('\u{012C}')) => continue,
            (Some('\u{012E}'), Some('\u{012F}')) => continue,
            (Some('\u{012F}'), Some('\u{012E}')) => continue,
            (Some('\u{0130}'), Some('\u{0069}')) => continue,
            (Some('\u{0069}'), Some('\u{0130}')) => continue,
            (Some('\u{0132}'), Some('\u{0133}')) => continue,
            (Some('\u{0133}'), Some('\u{0132}')) => continue,
            (Some('\u{0134}'), Some('\u{0135}')) => continue,
            (Some('\u{0135}'), Some('\u{0134}')) => continue,
            (Some('\u{0136}'), Some('\u{0137}')) => continue,
            (Some('\u{0137}'), Some('\u{0136}')) => continue,
            (Some('\u{0139}'), Some('\u{013A}')) => continue,
            (Some('\u{013A}'), Some('\u{0139}')) => continue,
            (Some('\u{013B}'), Some('\u{013C}')) => continue,
            (Some('\u{013C}'), Some('\u{013B}')) => continue,
            (Some('\u{013D}'), Some('\u{013E}')) => continue,
            (Some('\u{013E}'), Some('\u{013D}')) => continue,
            (Some('\u{013F}'), Some('\u{0140}')) => continue,
            (Some('\u{0140}'), Some('\u{013F}')) => continue,
            (Some('\u{0141}'), Some('\u{0142}')) => continue,
            (Some('\u{0142}'), Some('\u{0141}')) => continue,
            (Some('\u{0143}'), Some('\u{0144}')) => continue,
            (Some('\u{0144}'), Some('\u{0143}')) => continue,
            (Some('\u{0145}'), Some('\u{0146}')) => continue,
            (Some('\u{0146}'), Some('\u{0145}')) => continue,
            (Some('\u{0147}'), Some('\u{0148}')) => continue,
            (Some('\u{0148}'), Some('\u{0147}')) => continue,
            (Some('\u{0149}'), Some('\u{02BC}')) if matches!(right.next(), Some('\u{006E}')) => {
                continue
            }
            (Some('\u{02BC}'), Some('\u{0149}')) if matches!(left.next(), Some('\u{006E}')) => {
                continue
            }
            (Some('\u{014A}'), Some('\u{014B}')) => continue,
            (Some('\u{014B}'), Some('\u{014A}')) => continue,
            (Some('\u{014C}'), Some('\u{014D}')) => continue,
            (Some('\u{014D}'), Some('\u{014C}')) => continue,
            (Some('\u{014E}'), Some('\u{014F}')) => continue,
            (Some('\u{014F}'), Some('\u{014E}')) => continue,
            (Some('\u{0150}'), Some('\u{0151}')) => continue,
            (Some('\u{0151}'), Some('\u{0150}')) => continue,
            (Some('\u{0152}'), Some('\u{0153}')) => continue,
            (Some('\u{0153}'), Some('\u{0152}')) => continue,
            (Some('\u{0154}'), Some('\u{0155}')) => continue,
            (Some('\u{0155}'), Some('\u{0154}')) => continue,
            (Some('\u{0156}'), Some('\u{0157}')) => continue,
            (Some('\u{0157}'), Some('\u{0156}')) => continue,
            (Some('\u{0158}'), Some('\u{0159}')) => continue,
            (Some('\u{0159}'), Some('\u{0158}')) => continue,
            (Some('\u{015A}'), Some('\u{015B}')) => continue,
            (Some('\u{015B}'), Some('\u{015A}')) => continue,
            (Some('\u{015C}'), Some('\u{015D}')) => continue,
            (Some('\u{015D}'), Some('\u{015C}')) => continue,
            (Some('\u{015E}'), Some('\u{015F}')) => continue,
            (Some('\u{015F}'), Some('\u{015E}')) => continue,
            (Some('\u{0160}'), Some('\u{0161}')) => continue,
            (Some('\u{0161}'), Some('\u{0160}')) => continue,
            (Some('\u{0162}'), Some('\u{0163}')) => continue,
            (Some('\u{0163}'), Some('\u{0162}')) => continue,
            (Some('\u{0164}'), Some('\u{0165}')) => continue,
            (Some('\u{0165}'), Some('\u{0164}')) => continue,
            (Some('\u{0166}'), Some('\u{0167}')) => continue,
            (Some('\u{0167}'), Some('\u{0166}')) => continue,
            (Some('\u{0168}'), Some('\u{0169}')) => continue,
            (Some('\u{0169}'), Some('\u{0168}')) => continue,
            (Some('\u{016A}'), Some('\u{016B}')) => continue,
            (Some('\u{016B}'), Some('\u{016A}')) => continue,
            (Some('\u{016C}'), Some('\u{016D}')) => continue,
            (Some('\u{016D}'), Some('\u{016C}')) => continue,
            (Some('\u{016E}'), Some('\u{016F}')) => continue,
            (Some('\u{016F}'), Some('\u{016E}')) => continue,
            (Some('\u{0170}'), Some('\u{0171}')) => continue,
            (Some('\u{0171}'), Some('\u{0170}')) => continue,
            (Some('\u{0172}'), Some('\u{0173}')) => continue,
            (Some('\u{0173}'), Some('\u{0172}')) => continue,
            (Some('\u{0174}'), Some('\u{0175}')) => continue,
            (Some('\u{0175}'), Some('\u{0174}')) => continue,
            (Some('\u{0176}'), Some('\u{0177}')) => continue,
            (Some('\u{0177}'), Some('\u{0176}')) => continue,
            (Some('\u{0178}'), Some('\u{00FF}')) => continue,
            (Some('\u{00FF}'), Some('\u{0178}')) => continue,
            (Some('\u{0179}'), Some('\u{017A}')) => continue,
            (Some('\u{017A}'), Some('\u{0179}')) => continue,
            (Some('\u{017B}'), Some('\u{017C}')) => continue,
            (Some('\u{017C}'), Some('\u{017B}')) => continue,
            (Some('\u{017D}'), Some('\u{017E}')) => continue,
            (Some('\u{017E}'), Some('\u{017D}')) => continue,
            (Some('\u{017F}'), Some('\u{0073}')) => continue,
            (Some('\u{0073}'), Some('\u{017F}')) => continue,
            (Some('\u{0181}'), Some('\u{0253}')) => continue,
            (Some('\u{0253}'), Some('\u{0181}')) => continue,
            (Some('\u{0182}'), Some('\u{0183}')) => continue,
            (Some('\u{0183}'), Some('\u{0182}')) => continue,
            (Some('\u{0184}'), Some('\u{0185}')) => continue,
            (Some('\u{0185}'), Some('\u{0184}')) => continue,
            (Some('\u{0186}'), Some('\u{0254}')) => continue,
            (Some('\u{0254}'), Some('\u{0186}')) => continue,
            (Some('\u{0187}'), Some('\u{0188}')) => continue,
            (Some('\u{0188}'), Some('\u{0187}')) => continue,
            (Some('\u{0189}'), Some('\u{0256}')) => continue,
            (Some('\u{0256}'), Some('\u{0189}')) => continue,
            (Some('\u{018A}'), Some('\u{0257}')) => continue,
            (Some('\u{0257}'), Some('\u{018A}')) => continue,
            (Some('\u{018B}'), Some('\u{018C}')) => continue,
            (Some('\u{018C}'), Some('\u{018B}')) => continue,
            (Some('\u{018E}'), Some('\u{01DD}')) => continue,
            (Some('\u{01DD}'), Some('\u{018E}')) => continue,
            (Some('\u{018F}'), Some('\u{0259}')) => continue,
            (Some('\u{0259}'), Some('\u{018F}')) => continue,
            (Some('\u{0190}'), Some('\u{025B}')) => continue,
            (Some('\u{025B}'), Some('\u{0190}')) => continue,
            (Some('\u{0191}'), Some('\u{0192}')) => continue,
            (Some('\u{0192}'), Some('\u{0191}')) => continue,
            (Some('\u{0193}'), Some('\u{0260}')) => continue,
            (Some('\u{0260}'), Some('\u{0193}')) => continue,
            (Some('\u{0194}'), Some('\u{0263}')) => continue,
            (Some('\u{0263}'), Some('\u{0194}')) => continue,
            (Some('\u{0196}'), Some('\u{0269}')) => continue,
            (Some('\u{0269}'), Some('\u{0196}')) => continue,
            (Some('\u{0197}'), Some('\u{0268}')) => continue,
            (Some('\u{0268}'), Some('\u{0197}')) => continue,
            (Some('\u{0198}'), Some('\u{0199}')) => continue,
            (Some('\u{0199}'), Some('\u{0198}')) => continue,
            (Some('\u{019C}'), Some('\u{026F}')) => continue,
            (Some('\u{026F}'), Some('\u{019C}')) => continue,
            (Some('\u{019D}'), Some('\u{0272}')) => continue,
            (Some('\u{0272}'), Some('\u{019D}')) => continue,
            (Some('\u{019F}'), Some('\u{0275}')) => continue,
            (Some('\u{0275}'), Some('\u{019F}')) => continue,
            (Some('\u{01A0}'), Some('\u{01A1}')) => continue,
            (Some('\u{01A1}'), Some('\u{01A0}')) => continue,
            (Some('\u{01A2}'), Some('\u{01A3}')) => continue,
            (Some('\u{01A3}'), Some('\u{01A2}')) => continue,
            (Some('\u{01A4}'), Some('\u{01A5}')) => continue,
            (Some('\u{01A5}'), Some('\u{01A4}')) => continue,
            (Some('\u{01A6}'), Some('\u{0280}')) => continue,
            (Some('\u{0280}'), Some('\u{01A6}')) => continue,
            (Some('\u{01A7}'), Some('\u{01A8}')) => continue,
            (Some('\u{01A8}'), Some('\u{01A7}')) => continue,
            (Some('\u{01A9}'), Some('\u{0283}')) => continue,
            (Some('\u{0283}'), Some('\u{01A9}')) => continue,
            (Some('\u{01AC}'), Some('\u{01AD}')) => continue,
            (Some('\u{01AD}'), Some('\u{01AC}')) => continue,
            (Some('\u{01AE}'), Some('\u{0288}')) => continue,
            (Some('\u{0288}'), Some('\u{01AE}')) => continue,
            (Some('\u{01AF}'), Some('\u{01B0}')) => continue,
            (Some('\u{01B0}'), Some('\u{01AF}')) => continue,
            (Some('\u{01B1}'), Some('\u{028A}')) => continue,
            (Some('\u{028A}'), Some('\u{01B1}')) => continue,
            (Some('\u{01B2}'), Some('\u{028B}')) => continue,
            (Some('\u{028B}'), Some('\u{01B2}')) => continue,
            (Some('\u{01B3}'), Some('\u{01B4}')) => continue,
            (Some('\u{01B4}'), Some('\u{01B3}')) => continue,
            (Some('\u{01B5}'), Some('\u{01B6}')) => continue,
            (Some('\u{01B6}'), Some('\u{01B5}')) => continue,
            (Some('\u{01B7}'), Some('\u{0292}')) => continue,
            (Some('\u{0292}'), Some('\u{01B7}')) => continue,
            (Some('\u{01B8}'), Some('\u{01B9}')) => continue,
            (Some('\u{01B9}'), Some('\u{01B8}')) => continue,
            (Some('\u{01BC}'), Some('\u{01BD}')) => continue,
            (Some('\u{01BD}'), Some('\u{01BC}')) => continue,
            (Some('\u{01C4}'), Some('\u{01C6}')) => continue,
            (Some('\u{01C6}'), Some('\u{01C4}')) => continue,
            (Some('\u{01C5}'), Some('\u{01C6}')) => continue,
            (Some('\u{01C6}'), Some('\u{01C5}')) => continue,
            (Some('\u{01C7}'), Some('\u{01C9}')) => continue,
            (Some('\u{01C9}'), Some('\u{01C7}')) => continue,
            (Some('\u{01C8}'), Some('\u{01C9}')) => continue,
            (Some('\u{01C9}'), Some('\u{01C8}')) => continue,
            (Some('\u{01CA}'), Some('\u{01CC}')) => continue,
            (Some('\u{01CC}'), Some('\u{01CA}')) => continue,
            (Some('\u{01CB}'), Some('\u{01CC}')) => continue,
            (Some('\u{01CC}'), Some('\u{01CB}')) => continue,
            (Some('\u{01CD}'), Some('\u{01CE}')) => continue,
            (Some('\u{01CE}'), Some('\u{01CD}')) => continue,
            (Some('\u{01CF}'), Some('\u{01D0}')) => continue,
            (Some('\u{01D0}'), Some('\u{01CF}')) => continue,
            (Some('\u{01D1}'), Some('\u{01D2}')) => continue,
            (Some('\u{01D2}'), Some('\u{01D1}')) => continue,
            (Some('\u{01D3}'), Some('\u{01D4}')) => continue,
            (Some('\u{01D4}'), Some('\u{01D3}')) => continue,
            (Some('\u{01D5}'), Some('\u{01D6}')) => continue,
            (Some('\u{01D6}'), Some('\u{01D5}')) => continue,
            (Some('\u{01D7}'), Some('\u{01D8}')) => continue,
            (Some('\u{01D8}'), Some('\u{01D7}')) => continue,
            (Some('\u{01D9}'), Some('\u{01DA}')) => continue,
            (Some('\u{01DA}'), Some('\u{01D9}')) => continue,
            (Some('\u{01DB}'), Some('\u{01DC}')) => continue,
            (Some('\u{01DC}'), Some('\u{01DB}')) => continue,
            (Some('\u{01DE}'), Some('\u{01DF}')) => continue,
            (Some('\u{01DF}'), Some('\u{01DE}')) => continue,
            (Some('\u{01E0}'), Some('\u{01E1}')) => continue,
            (Some('\u{01E1}'), Some('\u{01E0}')) => continue,
            (Some('\u{01E2}'), Some('\u{01E3}')) => continue,
            (Some('\u{01E3}'), Some('\u{01E2}')) => continue,
            (Some('\u{01E4}'), Some('\u{01E5}')) => continue,
            (Some('\u{01E5}'), Some('\u{01E4}')) => continue,
            (Some('\u{01E6}'), Some('\u{01E7}')) => continue,
            (Some('\u{01E7}'), Some('\u{01E6}')) => continue,
            (Some('\u{01E8}'), Some('\u{01E9}')) => continue,
            (Some('\u{01E9}'), Some('\u{01E8}')) => continue,
            (Some('\u{01EA}'), Some('\u{01EB}')) => continue,
            (Some('\u{01EB}'), Some('\u{01EA}')) => continue,
            (Some('\u{01EC}'), Some('\u{01ED}')) => continue,
            (Some('\u{01ED}'), Some('\u{01EC}')) => continue,
            (Some('\u{01EE}'), Some('\u{01EF}')) => continue,
            (Some('\u{01EF}'), Some('\u{01EE}')) => continue,
            (Some('\u{01F0}'), Some('\u{006A}')) if matches!(right.next(), Some('\u{030C}')) => {
                continue
            }
            (Some('\u{006A}'), Some('\u{01F0}')) if matches!(left.next(), Some('\u{030C}')) => {
                continue
            }
            (Some('\u{01F1}'), Some('\u{01F3}')) => continue,
            (Some('\u{01F3}'), Some('\u{01F1}')) => continue,
            (Some('\u{01F2}'), Some('\u{01F3}')) => continue,
            (Some('\u{01F3}'), Some('\u{01F2}')) => continue,
            (Some('\u{01F4}'), Some('\u{01F5}')) => continue,
            (Some('\u{01F5}'), Some('\u{01F4}')) => continue,
            (Some('\u{01F6}'), Some('\u{0195}')) => continue,
            (Some('\u{0195}'), Some('\u{01F6}')) => continue,
            (Some('\u{01F7}'), Some('\u{01BF}')) => continue,
            (Some('\u{01BF}'), Some('\u{01F7}')) => continue,
            (Some('\u{01F8}'), Some('\u{01F9}')) => continue,
            (Some('\u{01F9}'), Some('\u{01F8}')) => continue,
            (Some('\u{01FA}'), Some('\u{01FB}')) => continue,
            (Some('\u{01FB}'), Some('\u{01FA}')) => continue,
            (Some('\u{01FC}'), Some('\u{01FD}')) => continue,
            (Some('\u{01FD}'), Some('\u{01FC}')) => continue,
            (Some('\u{01FE}'), Some('\u{01FF}')) => continue,
            (Some('\u{01FF}'), Some('\u{01FE}')) => continue,
            (Some('\u{0200}'), Some('\u{0201}')) => continue,
            (Some('\u{0201}'), Some('\u{0200}')) => continue,
            (Some('\u{0202}'), Some('\u{0203}')) => continue,
            (Some('\u{0203}'), Some('\u{0202}')) => continue,
            (Some('\u{0204}'), Some('\u{0205}')) => continue,
            (Some('\u{0205}'), Some('\u{0204}')) => continue,
            (Some('\u{0206}'), Some('\u{0207}')) => continue,
            (Some('\u{0207}'), Some('\u{0206}')) => continue,
            (Some('\u{0208}'), Some('\u{0209}')) => continue,
            (Some('\u{0209}'), Some('\u{0208}')) => continue,
            (Some('\u{020A}'), Some('\u{020B}')) => continue,
            (Some('\u{020B}'), Some('\u{020A}')) => continue,
            (Some('\u{020C}'), Some('\u{020D}')) => continue,
            (Some('\u{020D}'), Some('\u{020C}')) => continue,
            (Some('\u{020E}'), Some('\u{020F}')) => continue,
            (Some('\u{020F}'), Some('\u{020E}')) => continue,
            (Some('\u{0210}'), Some('\u{0211}')) => continue,
            (Some('\u{0211}'), Some('\u{0210}')) => continue,
            (Some('\u{0212}'), Some('\u{0213}')) => continue,
            (Some('\u{0213}'), Some('\u{0212}')) => continue,
            (Some('\u{0214}'), Some('\u{0215}')) => continue,
            (Some('\u{0215}'), Some('\u{0214}')) => continue,
            (Some('\u{0216}'), Some('\u{0217}')) => continue,
            (Some('\u{0217}'), Some('\u{0216}')) => continue,
            (Some('\u{0218}'), Some('\u{0219}')) => continue,
            (Some('\u{0219}'), Some('\u{0218}')) => continue,
            (Some('\u{021A}'), Some('\u{021B}')) => continue,
            (Some('\u{021B}'), Some('\u{021A}')) => continue,
            (Some('\u{021C}'), Some('\u{021D}')) => continue,
            (Some('\u{021D}'), Some('\u{021C}')) => continue,
            (Some('\u{021E}'), Some('\u{021F}')) => continue,
            (Some('\u{021F}'), Some('\u{021E}')) => continue,
            (Some('\u{0220}'), Some('\u{019E}')) => continue,
            (Some('\u{019E}'), Some('\u{0220}')) => continue,
            (Some('\u{0222}'), Some('\u{0223}')) => continue,
            (Some('\u{0223}'), Some('\u{0222}')) => continue,
            (Some('\u{0224}'), Some('\u{0225}')) => continue,
            (Some('\u{0225}'), Some('\u{0224}')) => continue,
            (Some('\u{0226}'), Some('\u{0227}')) => continue,
            (Some('\u{0227}'), Some('\u{0226}')) => continue,
            (Some('\u{0228}'), Some('\u{0229}')) => continue,
            (Some('\u{0229}'), Some('\u{0228}')) => continue,
            (Some('\u{022A}'), Some('\u{022B}')) => continue,
            (Some('\u{022B}'), Some('\u{022A}')) => continue,
            (Some('\u{022C}'), Some('\u{022D}')) => continue,
            (Some('\u{022D}'), Some('\u{022C}')) => continue,
            (Some('\u{022E}'), Some('\u{022F}')) => continue,
            (Some('\u{022F}'), Some('\u{022E}')) => continue,
            (Some('\u{0230}'), Some('\u{0231}')) => continue,
            (Some('\u{0231}'), Some('\u{0230}')) => continue,
            (Some('\u{0232}'), Some('\u{0233}')) => continue,
            (Some('\u{0233}'), Some('\u{0232}')) => continue,
            (Some('\u{023A}'), Some('\u{2C65}')) => continue,
            (Some('\u{2C65}'), Some('\u{023A}')) => continue,
            (Some('\u{023B}'), Some('\u{023C}')) => continue,
            (Some('\u{023C}'), Some('\u{023B}')) => continue,
            (Some('\u{023D}'), Some('\u{019A}')) => continue,
            (Some('\u{019A}'), Some('\u{023D}')) => continue,
            (Some('\u{023E}'), Some('\u{2C66}')) => continue,
            (Some('\u{2C66}'), Some('\u{023E}')) => continue,
            (Some('\u{0241}'), Some('\u{0242}')) => continue,
            (Some('\u{0242}'), Some('\u{0241}')) => continue,
            (Some('\u{0243}'), Some('\u{0180}')) => continue,
            (Some('\u{0180}'), Some('\u{0243}')) => continue,
            (Some('\u{0244}'), Some('\u{0289}')) => continue,
            (Some('\u{0289}'), Some('\u{0244}')) => continue,
            (Some('\u{0245}'), Some('\u{028C}')) => continue,
            (Some('\u{028C}'), Some('\u{0245}')) => continue,
            (Some('\u{0246}'), Some('\u{0247}')) => continue,
            (Some('\u{0247}'), Some('\u{0246}')) => continue,
            (Some('\u{0248}'), Some('\u{0249}')) => continue,
            (Some('\u{0249}'), Some('\u{0248}')) => continue,
            (Some('\u{024A}'), Some('\u{024B}')) => continue,
            (Some('\u{024B}'), Some('\u{024A}')) => continue,
            (Some('\u{024C}'), Some('\u{024D}')) => continue,
            (Some('\u{024D}'), Some('\u{024C}')) => continue,
            (Some('\u{024E}'), Some('\u{024F}')) => continue,
            (Some('\u{024F}'), Some('\u{024E}')) => continue,
            (Some('\u{0345}'), Some('\u{03B9}')) => continue,
            (Some('\u{03B9}'), Some('\u{0345}')) => continue,
            (Some('\u{0370}'), Some('\u{0371}')) => continue,
            (Some('\u{0371}'), Some('\u{0370}')) => continue,
            (Some('\u{0372}'), Some('\u{0373}')) => continue,
            (Some('\u{0373}'), Some('\u{0372}')) => continue,
            (Some('\u{0376}'), Some('\u{0377}')) => continue,
            (Some('\u{0377}'), Some('\u{0376}')) => continue,
            (Some('\u{037F}'), Some('\u{03F3}')) => continue,
            (Some('\u{03F3}'), Some('\u{037F}')) => continue,
            (Some('\u{0386}'), Some('\u{03AC}')) => continue,
            (Some('\u{03AC}'), Some('\u{0386}')) => continue,
            (Some('\u{0388}'), Some('\u{03AD}')) => continue,
            (Some('\u{03AD}'), Some('\u{0388}')) => continue,
            (Some('\u{0389}'), Some('\u{03AE}')) => continue,
            (Some('\u{03AE}'), Some('\u{0389}')) => continue,
            (Some('\u{038A}'), Some('\u{03AF}')) => continue,
            (Some('\u{03AF}'), Some('\u{038A}')) => continue,
            (Some('\u{038C}'), Some('\u{03CC}')) => continue,
            (Some('\u{03CC}'), Some('\u{038C}')) => continue,
            (Some('\u{038E}'), Some('\u{03CD}')) => continue,
            (Some('\u{03CD}'), Some('\u{038E}')) => continue,
            (Some('\u{038F}'), Some('\u{03CE}')) => continue,
            (Some('\u{03CE}'), Some('\u{038F}')) => continue,
            (Some('\u{0390}'), Some('\u{03B9}'))
                if matches!(right.next(), Some('\u{0308}'))
                    && matches!(right.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{03B9}'), Some('\u{0390}'))
                if matches!(left.next(), Some('\u{0308}'))
                    && matches!(left.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{0391}'), Some('\u{03B1}')) => continue,
            (Some('\u{03B1}'), Some('\u{0391}')) => continue,
            (Some('\u{0392}'), Some('\u{03B2}')) => continue,
            (Some('\u{03B2}'), Some('\u{0392}')) => continue,
            (Some('\u{0393}'), Some('\u{03B3}')) => continue,
            (Some('\u{03B3}'), Some('\u{0393}')) => continue,
            (Some('\u{0394}'), Some('\u{03B4}')) => continue,
            (Some('\u{03B4}'), Some('\u{0394}')) => continue,
            (Some('\u{0395}'), Some('\u{03B5}')) => continue,
            (Some('\u{03B5}'), Some('\u{0395}')) => continue,
            (Some('\u{0396}'), Some('\u{03B6}')) => continue,
            (Some('\u{03B6}'), Some('\u{0396}')) => continue,
            (Some('\u{0397}'), Some('\u{03B7}')) => continue,
            (Some('\u{03B7}'), Some('\u{0397}')) => continue,
            (Some('\u{0398}'), Some('\u{03B8}')) => continue,
            (Some('\u{03B8}'), Some('\u{0398}')) => continue,
            (Some('\u{0399}'), Some('\u{03B9}')) => continue,
            (Some('\u{03B9}'), Some('\u{0399}')) => continue,
            (Some('\u{039A}'), Some('\u{03BA}')) => continue,
            (Some('\u{03BA}'), Some('\u{039A}')) => continue,
            (Some('\u{039B}'), Some('\u{03BB}')) => continue,
            (Some('\u{03BB}'), Some('\u{039B}')) => continue,
            (Some('\u{039C}'), Some('\u{03BC}')) => continue,
            (Some('\u{03BC}'), Some('\u{039C}')) => continue,
            (Some('\u{039D}'), Some('\u{03BD}')) => continue,
            (Some('\u{03BD}'), Some('\u{039D}')) => continue,
            (Some('\u{039E}'), Some('\u{03BE}')) => continue,
            (Some('\u{03BE}'), Some('\u{039E}')) => continue,
            (Some('\u{039F}'), Some('\u{03BF}')) => continue,
            (Some('\u{03BF}'), Some('\u{039F}')) => continue,
            (Some('\u{03A0}'), Some('\u{03C0}')) => continue,
            (Some('\u{03C0}'), Some('\u{03A0}')) => continue,
            (Some('\u{03A1}'), Some('\u{03C1}')) => continue,
            (Some('\u{03C1}'), Some('\u{03A1}')) => continue,
            (Some('\u{03A3}'), Some('\u{03C3}')) => continue,
            (Some('\u{03C3}'), Some('\u{03A3}')) => continue,
            (Some('\u{03A4}'), Some('\u{03C4}')) => continue,
            (Some('\u{03C4}'), Some('\u{03A4}')) => continue,
            (Some('\u{03A5}'), Some('\u{03C5}')) => continue,
            (Some('\u{03C5}'), Some('\u{03A5}')) => continue,
            (Some('\u{03A6}'), Some('\u{03C6}')) => continue,
            (Some('\u{03C6}'), Some('\u{03A6}')) => continue,
            (Some('\u{03A7}'), Some('\u{03C7}')) => continue,
            (Some('\u{03C7}'), Some('\u{03A7}')) => continue,
            (Some('\u{03A8}'), Some('\u{03C8}')) => continue,
            (Some('\u{03C8}'), Some('\u{03A8}')) => continue,
            (Some('\u{03A9}'), Some('\u{03C9}')) => continue,
            (Some('\u{03C9}'), Some('\u{03A9}')) => continue,
            (Some('\u{03AA}'), Some('\u{03CA}')) => continue,
            (Some('\u{03CA}'), Some('\u{03AA}')) => continue,
            (Some('\u{03AB}'), Some('\u{03CB}')) => continue,
            (Some('\u{03CB}'), Some('\u{03AB}')) => continue,
            (Some('\u{03B0}'), Some('\u{03C5}'))
                if matches!(right.next(), Some('\u{0308}'))
                    && matches!(right.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{03C5}'), Some('\u{03B0}'))
                if matches!(left.next(), Some('\u{0308}'))
                    && matches!(left.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{03C2}'), Some('\u{03C3}')) => continue,
            (Some('\u{03C3}'), Some('\u{03C2}')) => continue,
            (Some('\u{03CF}'), Some('\u{03D7}')) => continue,
            (Some('\u{03D7}'), Some('\u{03CF}')) => continue,
            (Some('\u{03D0}'), Some('\u{03B2}')) => continue,
            (Some('\u{03B2}'), Some('\u{03D0}')) => continue,
            (Some('\u{03D1}'), Some('\u{03B8}')) => continue,
            (Some('\u{03B8}'), Some('\u{03D1}')) => continue,
            (Some('\u{03D5}'), Some('\u{03C6}')) => continue,
            (Some('\u{03C6}'), Some('\u{03D5}')) => continue,
            (Some('\u{03D6}'), Some('\u{03C0}')) => continue,
            (Some('\u{03C0}'), Some('\u{03D6}')) => continue,
            (Some('\u{03D8}'), Some('\u{03D9}')) => continue,
            (Some('\u{03D9}'), Some('\u{03D8}')) => continue,
            (Some('\u{03DA}'), Some('\u{03DB}')) => continue,
            (Some('\u{03DB}'), Some('\u{03DA}')) => continue,
            (Some('\u{03DC}'), Some('\u{03DD}')) => continue,
            (Some('\u{03DD}'), Some('\u{03DC}')) => continue,
            (Some('\u{03DE}'), Some('\u{03DF}')) => continue,
            (Some('\u{03DF}'), Some('\u{03DE}')) => continue,
            (Some('\u{03E0}'), Some('\u{03E1}')) => continue,
            (Some('\u{03E1}'), Some('\u{03E0}')) => continue,
            (Some('\u{03E2}'), Some('\u{03E3}')) => continue,
            (Some('\u{03E3}'), Some('\u{03E2}')) => continue,
            (Some('\u{03E4}'), Some('\u{03E5}')) => continue,
            (Some('\u{03E5}'), Some('\u{03E4}')) => continue,
            (Some('\u{03E6}'), Some('\u{03E7}')) => continue,
            (Some('\u{03E7}'), Some('\u{03E6}')) => continue,
            (Some('\u{03E8}'), Some('\u{03E9}')) => continue,
            (Some('\u{03E9}'), Some('\u{03E8}')) => continue,
            (Some('\u{03EA}'), Some('\u{03EB}')) => continue,
            (Some('\u{03EB}'), Some('\u{03EA}')) => continue,
            (Some('\u{03EC}'), Some('\u{03ED}')) => continue,
            (Some('\u{03ED}'), Some('\u{03EC}')) => continue,
            (Some('\u{03EE}'), Some('\u{03EF}')) => continue,
            (Some('\u{03EF}'), Some('\u{03EE}')) => continue,
            (Some('\u{03F0}'), Some('\u{03BA}')) => continue,
            (Some('\u{03BA}'), Some('\u{03F0}')) => continue,
            (Some('\u{03F1}'), Some('\u{03C1}')) => continue,
            (Some('\u{03C1}'), Some('\u{03F1}')) => continue,
            (Some('\u{03F4}'), Some('\u{03B8}')) => continue,
            (Some('\u{03B8}'), Some('\u{03F4}')) => continue,
            (Some('\u{03F5}'), Some('\u{03B5}')) => continue,
            (Some('\u{03B5}'), Some('\u{03F5}')) => continue,
            (Some('\u{03F7}'), Some('\u{03F8}')) => continue,
            (Some('\u{03F8}'), Some('\u{03F7}')) => continue,
            (Some('\u{03F9}'), Some('\u{03F2}')) => continue,
            (Some('\u{03F2}'), Some('\u{03F9}')) => continue,
            (Some('\u{03FA}'), Some('\u{03FB}')) => continue,
            (Some('\u{03FB}'), Some('\u{03FA}')) => continue,
            (Some('\u{03FD}'), Some('\u{037B}')) => continue,
            (Some('\u{037B}'), Some('\u{03FD}')) => continue,
            (Some('\u{03FE}'), Some('\u{037C}')) => continue,
            (Some('\u{037C}'), Some('\u{03FE}')) => continue,
            (Some('\u{03FF}'), Some('\u{037D}')) => continue,
            (Some('\u{037D}'), Some('\u{03FF}')) => continue,
            (Some('\u{0400}'), Some('\u{0450}')) => continue,
            (Some('\u{0450}'), Some('\u{0400}')) => continue,
            (Some('\u{0401}'), Some('\u{0451}')) => continue,
            (Some('\u{0451}'), Some('\u{0401}')) => continue,
            (Some('\u{0402}'), Some('\u{0452}')) => continue,
            (Some('\u{0452}'), Some('\u{0402}')) => continue,
            (Some('\u{0403}'), Some('\u{0453}')) => continue,
            (Some('\u{0453}'), Some('\u{0403}')) => continue,
            (Some('\u{0404}'), Some('\u{0454}')) => continue,
            (Some('\u{0454}'), Some('\u{0404}')) => continue,
            (Some('\u{0405}'), Some('\u{0455}')) => continue,
            (Some('\u{0455}'), Some('\u{0405}')) => continue,
            (Some('\u{0406}'), Some('\u{0456}')) => continue,
            (Some('\u{0456}'), Some('\u{0406}')) => continue,
            (Some('\u{0407}'), Some('\u{0457}')) => continue,
            (Some('\u{0457}'), Some('\u{0407}')) => continue,
            (Some('\u{0408}'), Some('\u{0458}')) => continue,
            (Some('\u{0458}'), Some('\u{0408}')) => continue,
            (Some('\u{0409}'), Some('\u{0459}')) => continue,
            (Some('\u{0459}'), Some('\u{0409}')) => continue,
            (Some('\u{040A}'), Some('\u{045A}')) => continue,
            (Some('\u{045A}'), Some('\u{040A}')) => continue,
            (Some('\u{040B}'), Some('\u{045B}')) => continue,
            (Some('\u{045B}'), Some('\u{040B}')) => continue,
            (Some('\u{040C}'), Some('\u{045C}')) => continue,
            (Some('\u{045C}'), Some('\u{040C}')) => continue,
            (Some('\u{040D}'), Some('\u{045D}')) => continue,
            (Some('\u{045D}'), Some('\u{040D}')) => continue,
            (Some('\u{040E}'), Some('\u{045E}')) => continue,
            (Some('\u{045E}'), Some('\u{040E}')) => continue,
            (Some('\u{040F}'), Some('\u{045F}')) => continue,
            (Some('\u{045F}'), Some('\u{040F}')) => continue,
            (Some('\u{0410}'), Some('\u{0430}')) => continue,
            (Some('\u{0430}'), Some('\u{0410}')) => continue,
            (Some('\u{0411}'), Some('\u{0431}')) => continue,
            (Some('\u{0431}'), Some('\u{0411}')) => continue,
            (Some('\u{0412}'), Some('\u{0432}')) => continue,
            (Some('\u{0432}'), Some('\u{0412}')) => continue,
            (Some('\u{0413}'), Some('\u{0433}')) => continue,
            (Some('\u{0433}'), Some('\u{0413}')) => continue,
            (Some('\u{0414}'), Some('\u{0434}')) => continue,
            (Some('\u{0434}'), Some('\u{0414}')) => continue,
            (Some('\u{0415}'), Some('\u{0435}')) => continue,
            (Some('\u{0435}'), Some('\u{0415}')) => continue,
            (Some('\u{0416}'), Some('\u{0436}')) => continue,
            (Some('\u{0436}'), Some('\u{0416}')) => continue,
            (Some('\u{0417}'), Some('\u{0437}')) => continue,
            (Some('\u{0437}'), Some('\u{0417}')) => continue,
            (Some('\u{0418}'), Some('\u{0438}')) => continue,
            (Some('\u{0438}'), Some('\u{0418}')) => continue,
            (Some('\u{0419}'), Some('\u{0439}')) => continue,
            (Some('\u{0439}'), Some('\u{0419}')) => continue,
            (Some('\u{041A}'), Some('\u{043A}')) => continue,
            (Some('\u{043A}'), Some('\u{041A}')) => continue,
            (Some('\u{041B}'), Some('\u{043B}')) => continue,
            (Some('\u{043B}'), Some('\u{041B}')) => continue,
            (Some('\u{041C}'), Some('\u{043C}')) => continue,
            (Some('\u{043C}'), Some('\u{041C}')) => continue,
            (Some('\u{041D}'), Some('\u{043D}')) => continue,
            (Some('\u{043D}'), Some('\u{041D}')) => continue,
            (Some('\u{041E}'), Some('\u{043E}')) => continue,
            (Some('\u{043E}'), Some('\u{041E}')) => continue,
            (Some('\u{041F}'), Some('\u{043F}')) => continue,
            (Some('\u{043F}'), Some('\u{041F}')) => continue,
            (Some('\u{0420}'), Some('\u{0440}')) => continue,
            (Some('\u{0440}'), Some('\u{0420}')) => continue,
            (Some('\u{0421}'), Some('\u{0441}')) => continue,
            (Some('\u{0441}'), Some('\u{0421}')) => continue,
            (Some('\u{0422}'), Some('\u{0442}')) => continue,
            (Some('\u{0442}'), Some('\u{0422}')) => continue,
            (Some('\u{0423}'), Some('\u{0443}')) => continue,
            (Some('\u{0443}'), Some('\u{0423}')) => continue,
            (Some('\u{0424}'), Some('\u{0444}')) => continue,
            (Some('\u{0444}'), Some('\u{0424}')) => continue,
            (Some('\u{0425}'), Some('\u{0445}')) => continue,
            (Some('\u{0445}'), Some('\u{0425}')) => continue,
            (Some('\u{0426}'), Some('\u{0446}')) => continue,
            (Some('\u{0446}'), Some('\u{0426}')) => continue,
            (Some('\u{0427}'), Some('\u{0447}')) => continue,
            (Some('\u{0447}'), Some('\u{0427}')) => continue,
            (Some('\u{0428}'), Some('\u{0448}')) => continue,
            (Some('\u{0448}'), Some('\u{0428}')) => continue,
            (Some('\u{0429}'), Some('\u{0449}')) => continue,
            (Some('\u{0449}'), Some('\u{0429}')) => continue,
            (Some('\u{042A}'), Some('\u{044A}')) => continue,
            (Some('\u{044A}'), Some('\u{042A}')) => continue,
            (Some('\u{042B}'), Some('\u{044B}')) => continue,
            (Some('\u{044B}'), Some('\u{042B}')) => continue,
            (Some('\u{042C}'), Some('\u{044C}')) => continue,
            (Some('\u{044C}'), Some('\u{042C}')) => continue,
            (Some('\u{042D}'), Some('\u{044D}')) => continue,
            (Some('\u{044D}'), Some('\u{042D}')) => continue,
            (Some('\u{042E}'), Some('\u{044E}')) => continue,
            (Some('\u{044E}'), Some('\u{042E}')) => continue,
            (Some('\u{042F}'), Some('\u{044F}')) => continue,
            (Some('\u{044F}'), Some('\u{042F}')) => continue,
            (Some('\u{0460}'), Some('\u{0461}')) => continue,
            (Some('\u{0461}'), Some('\u{0460}')) => continue,
            (Some('\u{0462}'), Some('\u{0463}')) => continue,
            (Some('\u{0463}'), Some('\u{0462}')) => continue,
            (Some('\u{0464}'), Some('\u{0465}')) => continue,
            (Some('\u{0465}'), Some('\u{0464}')) => continue,
            (Some('\u{0466}'), Some('\u{0467}')) => continue,
            (Some('\u{0467}'), Some('\u{0466}')) => continue,
            (Some('\u{0468}'), Some('\u{0469}')) => continue,
            (Some('\u{0469}'), Some('\u{0468}')) => continue,
            (Some('\u{046A}'), Some('\u{046B}')) => continue,
            (Some('\u{046B}'), Some('\u{046A}')) => continue,
            (Some('\u{046C}'), Some('\u{046D}')) => continue,
            (Some('\u{046D}'), Some('\u{046C}')) => continue,
            (Some('\u{046E}'), Some('\u{046F}')) => continue,
            (Some('\u{046F}'), Some('\u{046E}')) => continue,
            (Some('\u{0470}'), Some('\u{0471}')) => continue,
            (Some('\u{0471}'), Some('\u{0470}')) => continue,
            (Some('\u{0472}'), Some('\u{0473}')) => continue,
            (Some('\u{0473}'), Some('\u{0472}')) => continue,
            (Some('\u{0474}'), Some('\u{0475}')) => continue,
            (Some('\u{0475}'), Some('\u{0474}')) => continue,
            (Some('\u{0476}'), Some('\u{0477}')) => continue,
            (Some('\u{0477}'), Some('\u{0476}')) => continue,
            (Some('\u{0478}'), Some('\u{0479}')) => continue,
            (Some('\u{0479}'), Some('\u{0478}')) => continue,
            (Some('\u{047A}'), Some('\u{047B}')) => continue,
            (Some('\u{047B}'), Some('\u{047A}')) => continue,
            (Some('\u{047C}'), Some('\u{047D}')) => continue,
            (Some('\u{047D}'), Some('\u{047C}')) => continue,
            (Some('\u{047E}'), Some('\u{047F}')) => continue,
            (Some('\u{047F}'), Some('\u{047E}')) => continue,
            (Some('\u{0480}'), Some('\u{0481}')) => continue,
            (Some('\u{0481}'), Some('\u{0480}')) => continue,
            (Some('\u{048A}'), Some('\u{048B}')) => continue,
            (Some('\u{048B}'), Some('\u{048A}')) => continue,
            (Some('\u{048C}'), Some('\u{048D}')) => continue,
            (Some('\u{048D}'), Some('\u{048C}')) => continue,
            (Some('\u{048E}'), Some('\u{048F}')) => continue,
            (Some('\u{048F}'), Some('\u{048E}')) => continue,
            (Some('\u{0490}'), Some('\u{0491}')) => continue,
            (Some('\u{0491}'), Some('\u{0490}')) => continue,
            (Some('\u{0492}'), Some('\u{0493}')) => continue,
            (Some('\u{0493}'), Some('\u{0492}')) => continue,
            (Some('\u{0494}'), Some('\u{0495}')) => continue,
            (Some('\u{0495}'), Some('\u{0494}')) => continue,
            (Some('\u{0496}'), Some('\u{0497}')) => continue,
            (Some('\u{0497}'), Some('\u{0496}')) => continue,
            (Some('\u{0498}'), Some('\u{0499}')) => continue,
            (Some('\u{0499}'), Some('\u{0498}')) => continue,
            (Some('\u{049A}'), Some('\u{049B}')) => continue,
            (Some('\u{049B}'), Some('\u{049A}')) => continue,
            (Some('\u{049C}'), Some('\u{049D}')) => continue,
            (Some('\u{049D}'), Some('\u{049C}')) => continue,
            (Some('\u{049E}'), Some('\u{049F}')) => continue,
            (Some('\u{049F}'), Some('\u{049E}')) => continue,
            (Some('\u{04A0}'), Some('\u{04A1}')) => continue,
            (Some('\u{04A1}'), Some('\u{04A0}')) => continue,
            (Some('\u{04A2}'), Some('\u{04A3}')) => continue,
            (Some('\u{04A3}'), Some('\u{04A2}')) => continue,
            (Some('\u{04A4}'), Some('\u{04A5}')) => continue,
            (Some('\u{04A5}'), Some('\u{04A4}')) => continue,
            (Some('\u{04A6}'), Some('\u{04A7}')) => continue,
            (Some('\u{04A7}'), Some('\u{04A6}')) => continue,
            (Some('\u{04A8}'), Some('\u{04A9}')) => continue,
            (Some('\u{04A9}'), Some('\u{04A8}')) => continue,
            (Some('\u{04AA}'), Some('\u{04AB}')) => continue,
            (Some('\u{04AB}'), Some('\u{04AA}')) => continue,
            (Some('\u{04AC}'), Some('\u{04AD}')) => continue,
            (Some('\u{04AD}'), Some('\u{04AC}')) => continue,
            (Some('\u{04AE}'), Some('\u{04AF}')) => continue,
            (Some('\u{04AF}'), Some('\u{04AE}')) => continue,
            (Some('\u{04B0}'), Some('\u{04B1}')) => continue,
            (Some('\u{04B1}'), Some('\u{04B0}')) => continue,
            (Some('\u{04B2}'), Some('\u{04B3}')) => continue,
            (Some('\u{04B3}'), Some('\u{04B2}')) => continue,
            (Some('\u{04B4}'), Some('\u{04B5}')) => continue,
            (Some('\u{04B5}'), Some('\u{04B4}')) => continue,
            (Some('\u{04B6}'), Some('\u{04B7}')) => continue,
            (Some('\u{04B7}'), Some('\u{04B6}')) => continue,
            (Some('\u{04B8}'), Some('\u{04B9}')) => continue,
            (Some('\u{04B9}'), Some('\u{04B8}')) => continue,
            (Some('\u{04BA}'), Some('\u{04BB}')) => continue,
            (Some('\u{04BB}'), Some('\u{04BA}')) => continue,
            (Some('\u{04BC}'), Some('\u{04BD}')) => continue,
            (Some('\u{04BD}'), Some('\u{04BC}')) => continue,
            (Some('\u{04BE}'), Some('\u{04BF}')) => continue,
            (Some('\u{04BF}'), Some('\u{04BE}')) => continue,
            (Some('\u{04C0}'), Some('\u{04CF}')) => continue,
            (Some('\u{04CF}'), Some('\u{04C0}')) => continue,
            (Some('\u{04C1}'), Some('\u{04C2}')) => continue,
            (Some('\u{04C2}'), Some('\u{04C1}')) => continue,
            (Some('\u{04C3}'), Some('\u{04C4}')) => continue,
            (Some('\u{04C4}'), Some('\u{04C3}')) => continue,
            (Some('\u{04C5}'), Some('\u{04C6}')) => continue,
            (Some('\u{04C6}'), Some('\u{04C5}')) => continue,
            (Some('\u{04C7}'), Some('\u{04C8}')) => continue,
            (Some('\u{04C8}'), Some('\u{04C7}')) => continue,
            (Some('\u{04C9}'), Some('\u{04CA}')) => continue,
            (Some('\u{04CA}'), Some('\u{04C9}')) => continue,
            (Some('\u{04CB}'), Some('\u{04CC}')) => continue,
            (Some('\u{04CC}'), Some('\u{04CB}')) => continue,
            (Some('\u{04CD}'), Some('\u{04CE}')) => continue,
            (Some('\u{04CE}'), Some('\u{04CD}')) => continue,
            (Some('\u{04D0}'), Some('\u{04D1}')) => continue,
            (Some('\u{04D1}'), Some('\u{04D0}')) => continue,
            (Some('\u{04D2}'), Some('\u{04D3}')) => continue,
            (Some('\u{04D3}'), Some('\u{04D2}')) => continue,
            (Some('\u{04D4}'), Some('\u{04D5}')) => continue,
            (Some('\u{04D5}'), Some('\u{04D4}')) => continue,
            (Some('\u{04D6}'), Some('\u{04D7}')) => continue,
            (Some('\u{04D7}'), Some('\u{04D6}')) => continue,
            (Some('\u{04D8}'), Some('\u{04D9}')) => continue,
            (Some('\u{04D9}'), Some('\u{04D8}')) => continue,
            (Some('\u{04DA}'), Some('\u{04DB}')) => continue,
            (Some('\u{04DB}'), Some('\u{04DA}')) => continue,
            (Some('\u{04DC}'), Some('\u{04DD}')) => continue,
            (Some('\u{04DD}'), Some('\u{04DC}')) => continue,
            (Some('\u{04DE}'), Some('\u{04DF}')) => continue,
            (Some('\u{04DF}'), Some('\u{04DE}')) => continue,
            (Some('\u{04E0}'), Some('\u{04E1}')) => continue,
            (Some('\u{04E1}'), Some('\u{04E0}')) => continue,
            (Some('\u{04E2}'), Some('\u{04E3}')) => continue,
            (Some('\u{04E3}'), Some('\u{04E2}')) => continue,
            (Some('\u{04E4}'), Some('\u{04E5}')) => continue,
            (Some('\u{04E5}'), Some('\u{04E4}')) => continue,
            (Some('\u{04E6}'), Some('\u{04E7}')) => continue,
            (Some('\u{04E7}'), Some('\u{04E6}')) => continue,
            (Some('\u{04E8}'), Some('\u{04E9}')) => continue,
            (Some('\u{04E9}'), Some('\u{04E8}')) => continue,
            (Some('\u{04EA}'), Some('\u{04EB}')) => continue,
            (Some('\u{04EB}'), Some('\u{04EA}')) => continue,
            (Some('\u{04EC}'), Some('\u{04ED}')) => continue,
            (Some('\u{04ED}'), Some('\u{04EC}')) => continue,
            (Some('\u{04EE}'), Some('\u{04EF}')) => continue,
            (Some('\u{04EF}'), Some('\u{04EE}')) => continue,
            (Some('\u{04F0}'), Some('\u{04F1}')) => continue,
            (Some('\u{04F1}'), Some('\u{04F0}')) => continue,
            (Some('\u{04F2}'), Some('\u{04F3}')) => continue,
            (Some('\u{04F3}'), Some('\u{04F2}')) => continue,
            (Some('\u{04F4}'), Some('\u{04F5}')) => continue,
            (Some('\u{04F5}'), Some('\u{04F4}')) => continue,
            (Some('\u{04F6}'), Some('\u{04F7}')) => continue,
            (Some('\u{04F7}'), Some('\u{04F6}')) => continue,
            (Some('\u{04F8}'), Some('\u{04F9}')) => continue,
            (Some('\u{04F9}'), Some('\u{04F8}')) => continue,
            (Some('\u{04FA}'), Some('\u{04FB}')) => continue,
            (Some('\u{04FB}'), Some('\u{04FA}')) => continue,
            (Some('\u{04FC}'), Some('\u{04FD}')) => continue,
            (Some('\u{04FD}'), Some('\u{04FC}')) => continue,
            (Some('\u{04FE}'), Some('\u{04FF}')) => continue,
            (Some('\u{04FF}'), Some('\u{04FE}')) => continue,
            (Some('\u{0500}'), Some('\u{0501}')) => continue,
            (Some('\u{0501}'), Some('\u{0500}')) => continue,
            (Some('\u{0502}'), Some('\u{0503}')) => continue,
            (Some('\u{0503}'), Some('\u{0502}')) => continue,
            (Some('\u{0504}'), Some('\u{0505}')) => continue,
            (Some('\u{0505}'), Some('\u{0504}')) => continue,
            (Some('\u{0506}'), Some('\u{0507}')) => continue,
            (Some('\u{0507}'), Some('\u{0506}')) => continue,
            (Some('\u{0508}'), Some('\u{0509}')) => continue,
            (Some('\u{0509}'), Some('\u{0508}')) => continue,
            (Some('\u{050A}'), Some('\u{050B}')) => continue,
            (Some('\u{050B}'), Some('\u{050A}')) => continue,
            (Some('\u{050C}'), Some('\u{050D}')) => continue,
            (Some('\u{050D}'), Some('\u{050C}')) => continue,
            (Some('\u{050E}'), Some('\u{050F}')) => continue,
            (Some('\u{050F}'), Some('\u{050E}')) => continue,
            (Some('\u{0510}'), Some('\u{0511}')) => continue,
            (Some('\u{0511}'), Some('\u{0510}')) => continue,
            (Some('\u{0512}'), Some('\u{0513}')) => continue,
            (Some('\u{0513}'), Some('\u{0512}')) => continue,
            (Some('\u{0514}'), Some('\u{0515}')) => continue,
            (Some('\u{0515}'), Some('\u{0514}')) => continue,
            (Some('\u{0516}'), Some('\u{0517}')) => continue,
            (Some('\u{0517}'), Some('\u{0516}')) => continue,
            (Some('\u{0518}'), Some('\u{0519}')) => continue,
            (Some('\u{0519}'), Some('\u{0518}')) => continue,
            (Some('\u{051A}'), Some('\u{051B}')) => continue,
            (Some('\u{051B}'), Some('\u{051A}')) => continue,
            (Some('\u{051C}'), Some('\u{051D}')) => continue,
            (Some('\u{051D}'), Some('\u{051C}')) => continue,
            (Some('\u{051E}'), Some('\u{051F}')) => continue,
            (Some('\u{051F}'), Some('\u{051E}')) => continue,
            (Some('\u{0520}'), Some('\u{0521}')) => continue,
            (Some('\u{0521}'), Some('\u{0520}')) => continue,
            (Some('\u{0522}'), Some('\u{0523}')) => continue,
            (Some('\u{0523}'), Some('\u{0522}')) => continue,
            (Some('\u{0524}'), Some('\u{0525}')) => continue,
            (Some('\u{0525}'), Some('\u{0524}')) => continue,
            (Some('\u{0526}'), Some('\u{0527}')) => continue,
            (Some('\u{0527}'), Some('\u{0526}')) => continue,
            (Some('\u{0528}'), Some('\u{0529}')) => continue,
            (Some('\u{0529}'), Some('\u{0528}')) => continue,
            (Some('\u{052A}'), Some('\u{052B}')) => continue,
            (Some('\u{052B}'), Some('\u{052A}')) => continue,
            (Some('\u{052C}'), Some('\u{052D}')) => continue,
            (Some('\u{052D}'), Some('\u{052C}')) => continue,
            (Some('\u{052E}'), Some('\u{052F}')) => continue,
            (Some('\u{052F}'), Some('\u{052E}')) => continue,
            (Some('\u{0531}'), Some('\u{0561}')) => continue,
            (Some('\u{0561}'), Some('\u{0531}')) => continue,
            (Some('\u{0532}'), Some('\u{0562}')) => continue,
            (Some('\u{0562}'), Some('\u{0532}')) => continue,
            (Some('\u{0533}'), Some('\u{0563}')) => continue,
            (Some('\u{0563}'), Some('\u{0533}')) => continue,
            (Some('\u{0534}'), Some('\u{0564}')) => continue,
            (Some('\u{0564}'), Some('\u{0534}')) => continue,
            (Some('\u{0535}'), Some('\u{0565}')) => continue,
            (Some('\u{0565}'), Some('\u{0535}')) => continue,
            (Some('\u{0536}'), Some('\u{0566}')) => continue,
            (Some('\u{0566}'), Some('\u{0536}')) => continue,
            (Some('\u{0537}'), Some('\u{0567}')) => continue,
            (Some('\u{0567}'), Some('\u{0537}')) => continue,
            (Some('\u{0538}'), Some('\u{0568}')) => continue,
            (Some('\u{0568}'), Some('\u{0538}')) => continue,
            (Some('\u{0539}'), Some('\u{0569}')) => continue,
            (Some('\u{0569}'), Some('\u{0539}')) => continue,
            (Some('\u{053A}'), Some('\u{056A}')) => continue,
            (Some('\u{056A}'), Some('\u{053A}')) => continue,
            (Some('\u{053B}'), Some('\u{056B}')) => continue,
            (Some('\u{056B}'), Some('\u{053B}')) => continue,
            (Some('\u{053C}'), Some('\u{056C}')) => continue,
            (Some('\u{056C}'), Some('\u{053C}')) => continue,
            (Some('\u{053D}'), Some('\u{056D}')) => continue,
            (Some('\u{056D}'), Some('\u{053D}')) => continue,
            (Some('\u{053E}'), Some('\u{056E}')) => continue,
            (Some('\u{056E}'), Some('\u{053E}')) => continue,
            (Some('\u{053F}'), Some('\u{056F}')) => continue,
            (Some('\u{056F}'), Some('\u{053F}')) => continue,
            (Some('\u{0540}'), Some('\u{0570}')) => continue,
            (Some('\u{0570}'), Some('\u{0540}')) => continue,
            (Some('\u{0541}'), Some('\u{0571}')) => continue,
            (Some('\u{0571}'), Some('\u{0541}')) => continue,
            (Some('\u{0542}'), Some('\u{0572}')) => continue,
            (Some('\u{0572}'), Some('\u{0542}')) => continue,
            (Some('\u{0543}'), Some('\u{0573}')) => continue,
            (Some('\u{0573}'), Some('\u{0543}')) => continue,
            (Some('\u{0544}'), Some('\u{0574}')) => continue,
            (Some('\u{0574}'), Some('\u{0544}')) => continue,
            (Some('\u{0545}'), Some('\u{0575}')) => continue,
            (Some('\u{0575}'), Some('\u{0545}')) => continue,
            (Some('\u{0546}'), Some('\u{0576}')) => continue,
            (Some('\u{0576}'), Some('\u{0546}')) => continue,
            (Some('\u{0547}'), Some('\u{0577}')) => continue,
            (Some('\u{0577}'), Some('\u{0547}')) => continue,
            (Some('\u{0548}'), Some('\u{0578}')) => continue,
            (Some('\u{0578}'), Some('\u{0548}')) => continue,
            (Some('\u{0549}'), Some('\u{0579}')) => continue,
            (Some('\u{0579}'), Some('\u{0549}')) => continue,
            (Some('\u{054A}'), Some('\u{057A}')) => continue,
            (Some('\u{057A}'), Some('\u{054A}')) => continue,
            (Some('\u{054B}'), Some('\u{057B}')) => continue,
            (Some('\u{057B}'), Some('\u{054B}')) => continue,
            (Some('\u{054C}'), Some('\u{057C}')) => continue,
            (Some('\u{057C}'), Some('\u{054C}')) => continue,
            (Some('\u{054D}'), Some('\u{057D}')) => continue,
            (Some('\u{057D}'), Some('\u{054D}')) => continue,
            (Some('\u{054E}'), Some('\u{057E}')) => continue,
            (Some('\u{057E}'), Some('\u{054E}')) => continue,
            (Some('\u{054F}'), Some('\u{057F}')) => continue,
            (Some('\u{057F}'), Some('\u{054F}')) => continue,
            (Some('\u{0550}'), Some('\u{0580}')) => continue,
            (Some('\u{0580}'), Some('\u{0550}')) => continue,
            (Some('\u{0551}'), Some('\u{0581}')) => continue,
            (Some('\u{0581}'), Some('\u{0551}')) => continue,
            (Some('\u{0552}'), Some('\u{0582}')) => continue,
            (Some('\u{0582}'), Some('\u{0552}')) => continue,
            (Some('\u{0553}'), Some('\u{0583}')) => continue,
            (Some('\u{0583}'), Some('\u{0553}')) => continue,
            (Some('\u{0554}'), Some('\u{0584}')) => continue,
            (Some('\u{0584}'), Some('\u{0554}')) => continue,
            (Some('\u{0555}'), Some('\u{0585}')) => continue,
            (Some('\u{0585}'), Some('\u{0555}')) => continue,
            (Some('\u{0556}'), Some('\u{0586}')) => continue,
            (Some('\u{0586}'), Some('\u{0556}')) => continue,
            (Some('\u{0587}'), Some('\u{0565}')) if matches!(right.next(), Some('\u{0582}')) => {
                continue
            }
            (Some('\u{0565}'), Some('\u{0587}')) if matches!(left.next(), Some('\u{0582}')) => {
                continue
            }
            (Some('\u{10A0}'), Some('\u{2D00}')) => continue,
            (Some('\u{2D00}'), Some('\u{10A0}')) => continue,
            (Some('\u{10A1}'), Some('\u{2D01}')) => continue,
            (Some('\u{2D01}'), Some('\u{10A1}')) => continue,
            (Some('\u{10A2}'), Some('\u{2D02}')) => continue,
            (Some('\u{2D02}'), Some('\u{10A2}')) => continue,
            (Some('\u{10A3}'), Some('\u{2D03}')) => continue,
            (Some('\u{2D03}'), Some('\u{10A3}')) => continue,
            (Some('\u{10A4}'), Some('\u{2D04}')) => continue,
            (Some('\u{2D04}'), Some('\u{10A4}')) => continue,
            (Some('\u{10A5}'), Some('\u{2D05}')) => continue,
            (Some('\u{2D05}'), Some('\u{10A5}')) => continue,
            (Some('\u{10A6}'), Some('\u{2D06}')) => continue,
            (Some('\u{2D06}'), Some('\u{10A6}')) => continue,
            (Some('\u{10A7}'), Some('\u{2D07}')) => continue,
            (Some('\u{2D07}'), Some('\u{10A7}')) => continue,
            (Some('\u{10A8}'), Some('\u{2D08}')) => continue,
            (Some('\u{2D08}'), Some('\u{10A8}')) => continue,
            (Some('\u{10A9}'), Some('\u{2D09}')) => continue,
            (Some('\u{2D09}'), Some('\u{10A9}')) => continue,
            (Some('\u{10AA}'), Some('\u{2D0A}')) => continue,
            (Some('\u{2D0A}'), Some('\u{10AA}')) => continue,
            (Some('\u{10AB}'), Some('\u{2D0B}')) => continue,
            (Some('\u{2D0B}'), Some('\u{10AB}')) => continue,
            (Some('\u{10AC}'), Some('\u{2D0C}')) => continue,
            (Some('\u{2D0C}'), Some('\u{10AC}')) => continue,
            (Some('\u{10AD}'), Some('\u{2D0D}')) => continue,
            (Some('\u{2D0D}'), Some('\u{10AD}')) => continue,
            (Some('\u{10AE}'), Some('\u{2D0E}')) => continue,
            (Some('\u{2D0E}'), Some('\u{10AE}')) => continue,
            (Some('\u{10AF}'), Some('\u{2D0F}')) => continue,
            (Some('\u{2D0F}'), Some('\u{10AF}')) => continue,
            (Some('\u{10B0}'), Some('\u{2D10}')) => continue,
            (Some('\u{2D10}'), Some('\u{10B0}')) => continue,
            (Some('\u{10B1}'), Some('\u{2D11}')) => continue,
            (Some('\u{2D11}'), Some('\u{10B1}')) => continue,
            (Some('\u{10B2}'), Some('\u{2D12}')) => continue,
            (Some('\u{2D12}'), Some('\u{10B2}')) => continue,
            (Some('\u{10B3}'), Some('\u{2D13}')) => continue,
            (Some('\u{2D13}'), Some('\u{10B3}')) => continue,
            (Some('\u{10B4}'), Some('\u{2D14}')) => continue,
            (Some('\u{2D14}'), Some('\u{10B4}')) => continue,
            (Some('\u{10B5}'), Some('\u{2D15}')) => continue,
            (Some('\u{2D15}'), Some('\u{10B5}')) => continue,
            (Some('\u{10B6}'), Some('\u{2D16}')) => continue,
            (Some('\u{2D16}'), Some('\u{10B6}')) => continue,
            (Some('\u{10B7}'), Some('\u{2D17}')) => continue,
            (Some('\u{2D17}'), Some('\u{10B7}')) => continue,
            (Some('\u{10B8}'), Some('\u{2D18}')) => continue,
            (Some('\u{2D18}'), Some('\u{10B8}')) => continue,
            (Some('\u{10B9}'), Some('\u{2D19}')) => continue,
            (Some('\u{2D19}'), Some('\u{10B9}')) => continue,
            (Some('\u{10BA}'), Some('\u{2D1A}')) => continue,
            (Some('\u{2D1A}'), Some('\u{10BA}')) => continue,
            (Some('\u{10BB}'), Some('\u{2D1B}')) => continue,
            (Some('\u{2D1B}'), Some('\u{10BB}')) => continue,
            (Some('\u{10BC}'), Some('\u{2D1C}')) => continue,
            (Some('\u{2D1C}'), Some('\u{10BC}')) => continue,
            (Some('\u{10BD}'), Some('\u{2D1D}')) => continue,
            (Some('\u{2D1D}'), Some('\u{10BD}')) => continue,
            (Some('\u{10BE}'), Some('\u{2D1E}')) => continue,
            (Some('\u{2D1E}'), Some('\u{10BE}')) => continue,
            (Some('\u{10BF}'), Some('\u{2D1F}')) => continue,
            (Some('\u{2D1F}'), Some('\u{10BF}')) => continue,
            (Some('\u{10C0}'), Some('\u{2D20}')) => continue,
            (Some('\u{2D20}'), Some('\u{10C0}')) => continue,
            (Some('\u{10C1}'), Some('\u{2D21}')) => continue,
            (Some('\u{2D21}'), Some('\u{10C1}')) => continue,
            (Some('\u{10C2}'), Some('\u{2D22}')) => continue,
            (Some('\u{2D22}'), Some('\u{10C2}')) => continue,
            (Some('\u{10C3}'), Some('\u{2D23}')) => continue,
            (Some('\u{2D23}'), Some('\u{10C3}')) => continue,
            (Some('\u{10C4}'), Some('\u{2D24}')) => continue,
            (Some('\u{2D24}'), Some('\u{10C4}')) => continue,
            (Some('\u{10C5}'), Some('\u{2D25}')) => continue,
            (Some('\u{2D25}'), Some('\u{10C5}')) => continue,
            (Some('\u{10C7}'), Some('\u{2D27}')) => continue,
            (Some('\u{2D27}'), Some('\u{10C7}')) => continue,
            (Some('\u{10CD}'), Some('\u{2D2D}')) => continue,
            (Some('\u{2D2D}'), Some('\u{10CD}')) => continue,
            (Some('\u{13F8}'), Some('\u{13F0}')) => continue,
            (Some('\u{13F0}'), Some('\u{13F8}')) => continue,
            (Some('\u{13F9}'), Some('\u{13F1}')) => continue,
            (Some('\u{13F1}'), Some('\u{13F9}')) => continue,
            (Some('\u{13FA}'), Some('\u{13F2}')) => continue,
            (Some('\u{13F2}'), Some('\u{13FA}')) => continue,
            (Some('\u{13FB}'), Some('\u{13F3}')) => continue,
            (Some('\u{13F3}'), Some('\u{13FB}')) => continue,
            (Some('\u{13FC}'), Some('\u{13F4}')) => continue,
            (Some('\u{13F4}'), Some('\u{13FC}')) => continue,
            (Some('\u{13FD}'), Some('\u{13F5}')) => continue,
            (Some('\u{13F5}'), Some('\u{13FD}')) => continue,
            (Some('\u{1C80}'), Some('\u{0432}')) => continue,
            (Some('\u{0432}'), Some('\u{1C80}')) => continue,
            (Some('\u{1C81}'), Some('\u{0434}')) => continue,
            (Some('\u{0434}'), Some('\u{1C81}')) => continue,
            (Some('\u{1C82}'), Some('\u{043E}')) => continue,
            (Some('\u{043E}'), Some('\u{1C82}')) => continue,
            (Some('\u{1C83}'), Some('\u{0441}')) => continue,
            (Some('\u{0441}'), Some('\u{1C83}')) => continue,
            (Some('\u{1C84}'), Some('\u{0442}')) => continue,
            (Some('\u{0442}'), Some('\u{1C84}')) => continue,
            (Some('\u{1C85}'), Some('\u{0442}')) => continue,
            (Some('\u{0442}'), Some('\u{1C85}')) => continue,
            (Some('\u{1C86}'), Some('\u{044A}')) => continue,
            (Some('\u{044A}'), Some('\u{1C86}')) => continue,
            (Some('\u{1C87}'), Some('\u{0463}')) => continue,
            (Some('\u{0463}'), Some('\u{1C87}')) => continue,
            (Some('\u{1C88}'), Some('\u{A64B}')) => continue,
            (Some('\u{A64B}'), Some('\u{1C88}')) => continue,
            (Some('\u{1C90}'), Some('\u{10D0}')) => continue,
            (Some('\u{10D0}'), Some('\u{1C90}')) => continue,
            (Some('\u{1C91}'), Some('\u{10D1}')) => continue,
            (Some('\u{10D1}'), Some('\u{1C91}')) => continue,
            (Some('\u{1C92}'), Some('\u{10D2}')) => continue,
            (Some('\u{10D2}'), Some('\u{1C92}')) => continue,
            (Some('\u{1C93}'), Some('\u{10D3}')) => continue,
            (Some('\u{10D3}'), Some('\u{1C93}')) => continue,
            (Some('\u{1C94}'), Some('\u{10D4}')) => continue,
            (Some('\u{10D4}'), Some('\u{1C94}')) => continue,
            (Some('\u{1C95}'), Some('\u{10D5}')) => continue,
            (Some('\u{10D5}'), Some('\u{1C95}')) => continue,
            (Some('\u{1C96}'), Some('\u{10D6}')) => continue,
            (Some('\u{10D6}'), Some('\u{1C96}')) => continue,
            (Some('\u{1C97}'), Some('\u{10D7}')) => continue,
            (Some('\u{10D7}'), Some('\u{1C97}')) => continue,
            (Some('\u{1C98}'), Some('\u{10D8}')) => continue,
            (Some('\u{10D8}'), Some('\u{1C98}')) => continue,
            (Some('\u{1C99}'), Some('\u{10D9}')) => continue,
            (Some('\u{10D9}'), Some('\u{1C99}')) => continue,
            (Some('\u{1C9A}'), Some('\u{10DA}')) => continue,
            (Some('\u{10DA}'), Some('\u{1C9A}')) => continue,
            (Some('\u{1C9B}'), Some('\u{10DB}')) => continue,
            (Some('\u{10DB}'), Some('\u{1C9B}')) => continue,
            (Some('\u{1C9C}'), Some('\u{10DC}')) => continue,
            (Some('\u{10DC}'), Some('\u{1C9C}')) => continue,
            (Some('\u{1C9D}'), Some('\u{10DD}')) => continue,
            (Some('\u{10DD}'), Some('\u{1C9D}')) => continue,
            (Some('\u{1C9E}'), Some('\u{10DE}')) => continue,
            (Some('\u{10DE}'), Some('\u{1C9E}')) => continue,
            (Some('\u{1C9F}'), Some('\u{10DF}')) => continue,
            (Some('\u{10DF}'), Some('\u{1C9F}')) => continue,
            (Some('\u{1CA0}'), Some('\u{10E0}')) => continue,
            (Some('\u{10E0}'), Some('\u{1CA0}')) => continue,
            (Some('\u{1CA1}'), Some('\u{10E1}')) => continue,
            (Some('\u{10E1}'), Some('\u{1CA1}')) => continue,
            (Some('\u{1CA2}'), Some('\u{10E2}')) => continue,
            (Some('\u{10E2}'), Some('\u{1CA2}')) => continue,
            (Some('\u{1CA3}'), Some('\u{10E3}')) => continue,
            (Some('\u{10E3}'), Some('\u{1CA3}')) => continue,
            (Some('\u{1CA4}'), Some('\u{10E4}')) => continue,
            (Some('\u{10E4}'), Some('\u{1CA4}')) => continue,
            (Some('\u{1CA5}'), Some('\u{10E5}')) => continue,
            (Some('\u{10E5}'), Some('\u{1CA5}')) => continue,
            (Some('\u{1CA6}'), Some('\u{10E6}')) => continue,
            (Some('\u{10E6}'), Some('\u{1CA6}')) => continue,
            (Some('\u{1CA7}'), Some('\u{10E7}')) => continue,
            (Some('\u{10E7}'), Some('\u{1CA7}')) => continue,
            (Some('\u{1CA8}'), Some('\u{10E8}')) => continue,
            (Some('\u{10E8}'), Some('\u{1CA8}')) => continue,
            (Some('\u{1CA9}'), Some('\u{10E9}')) => continue,
            (Some('\u{10E9}'), Some('\u{1CA9}')) => continue,
            (Some('\u{1CAA}'), Some('\u{10EA}')) => continue,
            (Some('\u{10EA}'), Some('\u{1CAA}')) => continue,
            (Some('\u{1CAB}'), Some('\u{10EB}')) => continue,
            (Some('\u{10EB}'), Some('\u{1CAB}')) => continue,
            (Some('\u{1CAC}'), Some('\u{10EC}')) => continue,
            (Some('\u{10EC}'), Some('\u{1CAC}')) => continue,
            (Some('\u{1CAD}'), Some('\u{10ED}')) => continue,
            (Some('\u{10ED}'), Some('\u{1CAD}')) => continue,
            (Some('\u{1CAE}'), Some('\u{10EE}')) => continue,
            (Some('\u{10EE}'), Some('\u{1CAE}')) => continue,
            (Some('\u{1CAF}'), Some('\u{10EF}')) => continue,
            (Some('\u{10EF}'), Some('\u{1CAF}')) => continue,
            (Some('\u{1CB0}'), Some('\u{10F0}')) => continue,
            (Some('\u{10F0}'), Some('\u{1CB0}')) => continue,
            (Some('\u{1CB1}'), Some('\u{10F1}')) => continue,
            (Some('\u{10F1}'), Some('\u{1CB1}')) => continue,
            (Some('\u{1CB2}'), Some('\u{10F2}')) => continue,
            (Some('\u{10F2}'), Some('\u{1CB2}')) => continue,
            (Some('\u{1CB3}'), Some('\u{10F3}')) => continue,
            (Some('\u{10F3}'), Some('\u{1CB3}')) => continue,
            (Some('\u{1CB4}'), Some('\u{10F4}')) => continue,
            (Some('\u{10F4}'), Some('\u{1CB4}')) => continue,
            (Some('\u{1CB5}'), Some('\u{10F5}')) => continue,
            (Some('\u{10F5}'), Some('\u{1CB5}')) => continue,
            (Some('\u{1CB6}'), Some('\u{10F6}')) => continue,
            (Some('\u{10F6}'), Some('\u{1CB6}')) => continue,
            (Some('\u{1CB7}'), Some('\u{10F7}')) => continue,
            (Some('\u{10F7}'), Some('\u{1CB7}')) => continue,
            (Some('\u{1CB8}'), Some('\u{10F8}')) => continue,
            (Some('\u{10F8}'), Some('\u{1CB8}')) => continue,
            (Some('\u{1CB9}'), Some('\u{10F9}')) => continue,
            (Some('\u{10F9}'), Some('\u{1CB9}')) => continue,
            (Some('\u{1CBA}'), Some('\u{10FA}')) => continue,
            (Some('\u{10FA}'), Some('\u{1CBA}')) => continue,
            (Some('\u{1CBD}'), Some('\u{10FD}')) => continue,
            (Some('\u{10FD}'), Some('\u{1CBD}')) => continue,
            (Some('\u{1CBE}'), Some('\u{10FE}')) => continue,
            (Some('\u{10FE}'), Some('\u{1CBE}')) => continue,
            (Some('\u{1CBF}'), Some('\u{10FF}')) => continue,
            (Some('\u{10FF}'), Some('\u{1CBF}')) => continue,
            (Some('\u{1E00}'), Some('\u{1E01}')) => continue,
            (Some('\u{1E01}'), Some('\u{1E00}')) => continue,
            (Some('\u{1E02}'), Some('\u{1E03}')) => continue,
            (Some('\u{1E03}'), Some('\u{1E02}')) => continue,
            (Some('\u{1E04}'), Some('\u{1E05}')) => continue,
            (Some('\u{1E05}'), Some('\u{1E04}')) => continue,
            (Some('\u{1E06}'), Some('\u{1E07}')) => continue,
            (Some('\u{1E07}'), Some('\u{1E06}')) => continue,
            (Some('\u{1E08}'), Some('\u{1E09}')) => continue,
            (Some('\u{1E09}'), Some('\u{1E08}')) => continue,
            (Some('\u{1E0A}'), Some('\u{1E0B}')) => continue,
            (Some('\u{1E0B}'), Some('\u{1E0A}')) => continue,
            (Some('\u{1E0C}'), Some('\u{1E0D}')) => continue,
            (Some('\u{1E0D}'), Some('\u{1E0C}')) => continue,
            (Some('\u{1E0E}'), Some('\u{1E0F}')) => continue,
            (Some('\u{1E0F}'), Some('\u{1E0E}')) => continue,
            (Some('\u{1E10}'), Some('\u{1E11}')) => continue,
            (Some('\u{1E11}'), Some('\u{1E10}')) => continue,
            (Some('\u{1E12}'), Some('\u{1E13}')) => continue,
            (Some('\u{1E13}'), Some('\u{1E12}')) => continue,
            (Some('\u{1E14}'), Some('\u{1E15}')) => continue,
            (Some('\u{1E15}'), Some('\u{1E14}')) => continue,
            (Some('\u{1E16}'), Some('\u{1E17}')) => continue,
            (Some('\u{1E17}'), Some('\u{1E16}')) => continue,
            (Some('\u{1E18}'), Some('\u{1E19}')) => continue,
            (Some('\u{1E19}'), Some('\u{1E18}')) => continue,
            (Some('\u{1E1A}'), Some('\u{1E1B}')) => continue,
            (Some('\u{1E1B}'), Some('\u{1E1A}')) => continue,
            (Some('\u{1E1C}'), Some('\u{1E1D}')) => continue,
            (Some('\u{1E1D}'), Some('\u{1E1C}')) => continue,
            (Some('\u{1E1E}'), Some('\u{1E1F}')) => continue,
            (Some('\u{1E1F}'), Some('\u{1E1E}')) => continue,
            (Some('\u{1E20}'), Some('\u{1E21}')) => continue,
            (Some('\u{1E21}'), Some('\u{1E20}')) => continue,
            (Some('\u{1E22}'), Some('\u{1E23}')) => continue,
            (Some('\u{1E23}'), Some('\u{1E22}')) => continue,
            (Some('\u{1E24}'), Some('\u{1E25}')) => continue,
            (Some('\u{1E25}'), Some('\u{1E24}')) => continue,
            (Some('\u{1E26}'), Some('\u{1E27}')) => continue,
            (Some('\u{1E27}'), Some('\u{1E26}')) => continue,
            (Some('\u{1E28}'), Some('\u{1E29}')) => continue,
            (Some('\u{1E29}'), Some('\u{1E28}')) => continue,
            (Some('\u{1E2A}'), Some('\u{1E2B}')) => continue,
            (Some('\u{1E2B}'), Some('\u{1E2A}')) => continue,
            (Some('\u{1E2C}'), Some('\u{1E2D}')) => continue,
            (Some('\u{1E2D}'), Some('\u{1E2C}')) => continue,
            (Some('\u{1E2E}'), Some('\u{1E2F}')) => continue,
            (Some('\u{1E2F}'), Some('\u{1E2E}')) => continue,
            (Some('\u{1E30}'), Some('\u{1E31}')) => continue,
            (Some('\u{1E31}'), Some('\u{1E30}')) => continue,
            (Some('\u{1E32}'), Some('\u{1E33}')) => continue,
            (Some('\u{1E33}'), Some('\u{1E32}')) => continue,
            (Some('\u{1E34}'), Some('\u{1E35}')) => continue,
            (Some('\u{1E35}'), Some('\u{1E34}')) => continue,
            (Some('\u{1E36}'), Some('\u{1E37}')) => continue,
            (Some('\u{1E37}'), Some('\u{1E36}')) => continue,
            (Some('\u{1E38}'), Some('\u{1E39}')) => continue,
            (Some('\u{1E39}'), Some('\u{1E38}')) => continue,
            (Some('\u{1E3A}'), Some('\u{1E3B}')) => continue,
            (Some('\u{1E3B}'), Some('\u{1E3A}')) => continue,
            (Some('\u{1E3C}'), Some('\u{1E3D}')) => continue,
            (Some('\u{1E3D}'), Some('\u{1E3C}')) => continue,
            (Some('\u{1E3E}'), Some('\u{1E3F}')) => continue,
            (Some('\u{1E3F}'), Some('\u{1E3E}')) => continue,
            (Some('\u{1E40}'), Some('\u{1E41}')) => continue,
            (Some('\u{1E41}'), Some('\u{1E40}')) => continue,
            (Some('\u{1E42}'), Some('\u{1E43}')) => continue,
            (Some('\u{1E43}'), Some('\u{1E42}')) => continue,
            (Some('\u{1E44}'), Some('\u{1E45}')) => continue,
            (Some('\u{1E45}'), Some('\u{1E44}')) => continue,
            (Some('\u{1E46}'), Some('\u{1E47}')) => continue,
            (Some('\u{1E47}'), Some('\u{1E46}')) => continue,
            (Some('\u{1E48}'), Some('\u{1E49}')) => continue,
            (Some('\u{1E49}'), Some('\u{1E48}')) => continue,
            (Some('\u{1E4A}'), Some('\u{1E4B}')) => continue,
            (Some('\u{1E4B}'), Some('\u{1E4A}')) => continue,
            (Some('\u{1E4C}'), Some('\u{1E4D}')) => continue,
            (Some('\u{1E4D}'), Some('\u{1E4C}')) => continue,
            (Some('\u{1E4E}'), Some('\u{1E4F}')) => continue,
            (Some('\u{1E4F}'), Some('\u{1E4E}')) => continue,
            (Some('\u{1E50}'), Some('\u{1E51}')) => continue,
            (Some('\u{1E51}'), Some('\u{1E50}')) => continue,
            (Some('\u{1E52}'), Some('\u{1E53}')) => continue,
            (Some('\u{1E53}'), Some('\u{1E52}')) => continue,
            (Some('\u{1E54}'), Some('\u{1E55}')) => continue,
            (Some('\u{1E55}'), Some('\u{1E54}')) => continue,
            (Some('\u{1E56}'), Some('\u{1E57}')) => continue,
            (Some('\u{1E57}'), Some('\u{1E56}')) => continue,
            (Some('\u{1E58}'), Some('\u{1E59}')) => continue,
            (Some('\u{1E59}'), Some('\u{1E58}')) => continue,
            (Some('\u{1E5A}'), Some('\u{1E5B}')) => continue,
            (Some('\u{1E5B}'), Some('\u{1E5A}')) => continue,
            (Some('\u{1E5C}'), Some('\u{1E5D}')) => continue,
            (Some('\u{1E5D}'), Some('\u{1E5C}')) => continue,
            (Some('\u{1E5E}'), Some('\u{1E5F}')) => continue,
            (Some('\u{1E5F}'), Some('\u{1E5E}')) => continue,
            (Some('\u{1E60}'), Some('\u{1E61}')) => continue,
            (Some('\u{1E61}'), Some('\u{1E60}')) => continue,
            (Some('\u{1E62}'), Some('\u{1E63}')) => continue,
            (Some('\u{1E63}'), Some('\u{1E62}')) => continue,
            (Some('\u{1E64}'), Some('\u{1E65}')) => continue,
            (Some('\u{1E65}'), Some('\u{1E64}')) => continue,
            (Some('\u{1E66}'), Some('\u{1E67}')) => continue,
            (Some('\u{1E67}'), Some('\u{1E66}')) => continue,
            (Some('\u{1E68}'), Some('\u{1E69}')) => continue,
            (Some('\u{1E69}'), Some('\u{1E68}')) => continue,
            (Some('\u{1E6A}'), Some('\u{1E6B}')) => continue,
            (Some('\u{1E6B}'), Some('\u{1E6A}')) => continue,
            (Some('\u{1E6C}'), Some('\u{1E6D}')) => continue,
            (Some('\u{1E6D}'), Some('\u{1E6C}')) => continue,
            (Some('\u{1E6E}'), Some('\u{1E6F}')) => continue,
            (Some('\u{1E6F}'), Some('\u{1E6E}')) => continue,
            (Some('\u{1E70}'), Some('\u{1E71}')) => continue,
            (Some('\u{1E71}'), Some('\u{1E70}')) => continue,
            (Some('\u{1E72}'), Some('\u{1E73}')) => continue,
            (Some('\u{1E73}'), Some('\u{1E72}')) => continue,
            (Some('\u{1E74}'), Some('\u{1E75}')) => continue,
            (Some('\u{1E75}'), Some('\u{1E74}')) => continue,
            (Some('\u{1E76}'), Some('\u{1E77}')) => continue,
            (Some('\u{1E77}'), Some('\u{1E76}')) => continue,
            (Some('\u{1E78}'), Some('\u{1E79}')) => continue,
            (Some('\u{1E79}'), Some('\u{1E78}')) => continue,
            (Some('\u{1E7A}'), Some('\u{1E7B}')) => continue,
            (Some('\u{1E7B}'), Some('\u{1E7A}')) => continue,
            (Some('\u{1E7C}'), Some('\u{1E7D}')) => continue,
            (Some('\u{1E7D}'), Some('\u{1E7C}')) => continue,
            (Some('\u{1E7E}'), Some('\u{1E7F}')) => continue,
            (Some('\u{1E7F}'), Some('\u{1E7E}')) => continue,
            (Some('\u{1E80}'), Some('\u{1E81}')) => continue,
            (Some('\u{1E81}'), Some('\u{1E80}')) => continue,
            (Some('\u{1E82}'), Some('\u{1E83}')) => continue,
            (Some('\u{1E83}'), Some('\u{1E82}')) => continue,
            (Some('\u{1E84}'), Some('\u{1E85}')) => continue,
            (Some('\u{1E85}'), Some('\u{1E84}')) => continue,
            (Some('\u{1E86}'), Some('\u{1E87}')) => continue,
            (Some('\u{1E87}'), Some('\u{1E86}')) => continue,
            (Some('\u{1E88}'), Some('\u{1E89}')) => continue,
            (Some('\u{1E89}'), Some('\u{1E88}')) => continue,
            (Some('\u{1E8A}'), Some('\u{1E8B}')) => continue,
            (Some('\u{1E8B}'), Some('\u{1E8A}')) => continue,
            (Some('\u{1E8C}'), Some('\u{1E8D}')) => continue,
            (Some('\u{1E8D}'), Some('\u{1E8C}')) => continue,
            (Some('\u{1E8E}'), Some('\u{1E8F}')) => continue,
            (Some('\u{1E8F}'), Some('\u{1E8E}')) => continue,
            (Some('\u{1E90}'), Some('\u{1E91}')) => continue,
            (Some('\u{1E91}'), Some('\u{1E90}')) => continue,
            (Some('\u{1E92}'), Some('\u{1E93}')) => continue,
            (Some('\u{1E93}'), Some('\u{1E92}')) => continue,
            (Some('\u{1E94}'), Some('\u{1E95}')) => continue,
            (Some('\u{1E95}'), Some('\u{1E94}')) => continue,
            (Some('\u{1E96}'), Some('\u{0068}')) if matches!(right.next(), Some('\u{0331}')) => {
                continue
            }
            (Some('\u{0068}'), Some('\u{1E96}')) if matches!(left.next(), Some('\u{0331}')) => {
                continue
            }
            (Some('\u{1E97}'), Some('\u{0074}')) if matches!(right.next(), Some('\u{0308}')) => {
                continue
            }
            (Some('\u{0074}'), Some('\u{1E97}')) if matches!(left.next(), Some('\u{0308}')) => {
                continue
            }
            (Some('\u{1E98}'), Some('\u{0077}')) if matches!(right.next(), Some('\u{030A}')) => {
                continue
            }
            (Some('\u{0077}'), Some('\u{1E98}')) if matches!(left.next(), Some('\u{030A}')) => {
                continue
            }
            (Some('\u{1E99}'), Some('\u{0079}')) if matches!(right.next(), Some('\u{030A}')) => {
                continue
            }
            (Some('\u{0079}'), Some('\u{1E99}')) if matches!(left.next(), Some('\u{030A}')) => {
                continue
            }
            (Some('\u{1E9A}'), Some('\u{0061}')) if matches!(right.next(), Some('\u{02BE}')) => {
                continue
            }
            (Some('\u{0061}'), Some('\u{1E9A}')) if matches!(left.next(), Some('\u{02BE}')) => {
                continue
            }
            (Some('\u{1E9B}'), Some('\u{1E61}')) => continue,
            (Some('\u{1E61}'), Some('\u{1E9B}')) => continue,
            (Some('\u{1E9E}'), Some('\u{0073}')) if matches!(right.next(), Some('\u{0073}')) => {
                continue
            }
            (Some('\u{0073}'), Some('\u{1E9E}')) if matches!(left.next(), Some('\u{0073}')) => {
                continue
            }
            (Some('\u{1EA0}'), Some('\u{1EA1}')) => continue,
            (Some('\u{1EA1}'), Some('\u{1EA0}')) => continue,
            (Some('\u{1EA2}'), Some('\u{1EA3}')) => continue,
            (Some('\u{1EA3}'), Some('\u{1EA2}')) => continue,
            (Some('\u{1EA4}'), Some('\u{1EA5}')) => continue,
            (Some('\u{1EA5}'), Some('\u{1EA4}')) => continue,
            (Some('\u{1EA6}'), Some('\u{1EA7}')) => continue,
            (Some('\u{1EA7}'), Some('\u{1EA6}')) => continue,
            (Some('\u{1EA8}'), Some('\u{1EA9}')) => continue,
            (Some('\u{1EA9}'), Some('\u{1EA8}')) => continue,
            (Some('\u{1EAA}'), Some('\u{1EAB}')) => continue,
            (Some('\u{1EAB}'), Some('\u{1EAA}')) => continue,
            (Some('\u{1EAC}'), Some('\u{1EAD}')) => continue,
            (Some('\u{1EAD}'), Some('\u{1EAC}')) => continue,
            (Some('\u{1EAE}'), Some('\u{1EAF}')) => continue,
            (Some('\u{1EAF}'), Some('\u{1EAE}')) => continue,
            (Some('\u{1EB0}'), Some('\u{1EB1}')) => continue,
            (Some('\u{1EB1}'), Some('\u{1EB0}')) => continue,
            (Some('\u{1EB2}'), Some('\u{1EB3}')) => continue,
            (Some('\u{1EB3}'), Some('\u{1EB2}')) => continue,
            (Some('\u{1EB4}'), Some('\u{1EB5}')) => continue,
            (Some('\u{1EB5}'), Some('\u{1EB4}')) => continue,
            (Some('\u{1EB6}'), Some('\u{1EB7}')) => continue,
            (Some('\u{1EB7}'), Some('\u{1EB6}')) => continue,
            (Some('\u{1EB8}'), Some('\u{1EB9}')) => continue,
            (Some('\u{1EB9}'), Some('\u{1EB8}')) => continue,
            (Some('\u{1EBA}'), Some('\u{1EBB}')) => continue,
            (Some('\u{1EBB}'), Some('\u{1EBA}')) => continue,
            (Some('\u{1EBC}'), Some('\u{1EBD}')) => continue,
            (Some('\u{1EBD}'), Some('\u{1EBC}')) => continue,
            (Some('\u{1EBE}'), Some('\u{1EBF}')) => continue,
            (Some('\u{1EBF}'), Some('\u{1EBE}')) => continue,
            (Some('\u{1EC0}'), Some('\u{1EC1}')) => continue,
            (Some('\u{1EC1}'), Some('\u{1EC0}')) => continue,
            (Some('\u{1EC2}'), Some('\u{1EC3}')) => continue,
            (Some('\u{1EC3}'), Some('\u{1EC2}')) => continue,
            (Some('\u{1EC4}'), Some('\u{1EC5}')) => continue,
            (Some('\u{1EC5}'), Some('\u{1EC4}')) => continue,
            (Some('\u{1EC6}'), Some('\u{1EC7}')) => continue,
            (Some('\u{1EC7}'), Some('\u{1EC6}')) => continue,
            (Some('\u{1EC8}'), Some('\u{1EC9}')) => continue,
            (Some('\u{1EC9}'), Some('\u{1EC8}')) => continue,
            (Some('\u{1ECA}'), Some('\u{1ECB}')) => continue,
            (Some('\u{1ECB}'), Some('\u{1ECA}')) => continue,
            (Some('\u{1ECC}'), Some('\u{1ECD}')) => continue,
            (Some('\u{1ECD}'), Some('\u{1ECC}')) => continue,
            (Some('\u{1ECE}'), Some('\u{1ECF}')) => continue,
            (Some('\u{1ECF}'), Some('\u{1ECE}')) => continue,
            (Some('\u{1ED0}'), Some('\u{1ED1}')) => continue,
            (Some('\u{1ED1}'), Some('\u{1ED0}')) => continue,
            (Some('\u{1ED2}'), Some('\u{1ED3}')) => continue,
            (Some('\u{1ED3}'), Some('\u{1ED2}')) => continue,
            (Some('\u{1ED4}'), Some('\u{1ED5}')) => continue,
            (Some('\u{1ED5}'), Some('\u{1ED4}')) => continue,
            (Some('\u{1ED6}'), Some('\u{1ED7}')) => continue,
            (Some('\u{1ED7}'), Some('\u{1ED6}')) => continue,
            (Some('\u{1ED8}'), Some('\u{1ED9}')) => continue,
            (Some('\u{1ED9}'), Some('\u{1ED8}')) => continue,
            (Some('\u{1EDA}'), Some('\u{1EDB}')) => continue,
            (Some('\u{1EDB}'), Some('\u{1EDA}')) => continue,
            (Some('\u{1EDC}'), Some('\u{1EDD}')) => continue,
            (Some('\u{1EDD}'), Some('\u{1EDC}')) => continue,
            (Some('\u{1EDE}'), Some('\u{1EDF}')) => continue,
            (Some('\u{1EDF}'), Some('\u{1EDE}')) => continue,
            (Some('\u{1EE0}'), Some('\u{1EE1}')) => continue,
            (Some('\u{1EE1}'), Some('\u{1EE0}')) => continue,
            (Some('\u{1EE2}'), Some('\u{1EE3}')) => continue,
            (Some('\u{1EE3}'), Some('\u{1EE2}')) => continue,
            (Some('\u{1EE4}'), Some('\u{1EE5}')) => continue,
            (Some('\u{1EE5}'), Some('\u{1EE4}')) => continue,
            (Some('\u{1EE6}'), Some('\u{1EE7}')) => continue,
            (Some('\u{1EE7}'), Some('\u{1EE6}')) => continue,
            (Some('\u{1EE8}'), Some('\u{1EE9}')) => continue,
            (Some('\u{1EE9}'), Some('\u{1EE8}')) => continue,
            (Some('\u{1EEA}'), Some('\u{1EEB}')) => continue,
            (Some('\u{1EEB}'), Some('\u{1EEA}')) => continue,
            (Some('\u{1EEC}'), Some('\u{1EED}')) => continue,
            (Some('\u{1EED}'), Some('\u{1EEC}')) => continue,
            (Some('\u{1EEE}'), Some('\u{1EEF}')) => continue,
            (Some('\u{1EEF}'), Some('\u{1EEE}')) => continue,
            (Some('\u{1EF0}'), Some('\u{1EF1}')) => continue,
            (Some('\u{1EF1}'), Some('\u{1EF0}')) => continue,
            (Some('\u{1EF2}'), Some('\u{1EF3}')) => continue,
            (Some('\u{1EF3}'), Some('\u{1EF2}')) => continue,
            (Some('\u{1EF4}'), Some('\u{1EF5}')) => continue,
            (Some('\u{1EF5}'), Some('\u{1EF4}')) => continue,
            (Some('\u{1EF6}'), Some('\u{1EF7}')) => continue,
            (Some('\u{1EF7}'), Some('\u{1EF6}')) => continue,
            (Some('\u{1EF8}'), Some('\u{1EF9}')) => continue,
            (Some('\u{1EF9}'), Some('\u{1EF8}')) => continue,
            (Some('\u{1EFA}'), Some('\u{1EFB}')) => continue,
            (Some('\u{1EFB}'), Some('\u{1EFA}')) => continue,
            (Some('\u{1EFC}'), Some('\u{1EFD}')) => continue,
            (Some('\u{1EFD}'), Some('\u{1EFC}')) => continue,
            (Some('\u{1EFE}'), Some('\u{1EFF}')) => continue,
            (Some('\u{1EFF}'), Some('\u{1EFE}')) => continue,
            (Some('\u{1F08}'), Some('\u{1F00}')) => continue,
            (Some('\u{1F00}'), Some('\u{1F08}')) => continue,
            (Some('\u{1F09}'), Some('\u{1F01}')) => continue,
            (Some('\u{1F01}'), Some('\u{1F09}')) => continue,
            (Some('\u{1F0A}'), Some('\u{1F02}')) => continue,
            (Some('\u{1F02}'), Some('\u{1F0A}')) => continue,
            (Some('\u{1F0B}'), Some('\u{1F03}')) => continue,
            (Some('\u{1F03}'), Some('\u{1F0B}')) => continue,
            (Some('\u{1F0C}'), Some('\u{1F04}')) => continue,
            (Some('\u{1F04}'), Some('\u{1F0C}')) => continue,
            (Some('\u{1F0D}'), Some('\u{1F05}')) => continue,
            (Some('\u{1F05}'), Some('\u{1F0D}')) => continue,
            (Some('\u{1F0E}'), Some('\u{1F06}')) => continue,
            (Some('\u{1F06}'), Some('\u{1F0E}')) => continue,
            (Some('\u{1F0F}'), Some('\u{1F07}')) => continue,
            (Some('\u{1F07}'), Some('\u{1F0F}')) => continue,
            (Some('\u{1F18}'), Some('\u{1F10}')) => continue,
            (Some('\u{1F10}'), Some('\u{1F18}')) => continue,
            (Some('\u{1F19}'), Some('\u{1F11}')) => continue,
            (Some('\u{1F11}'), Some('\u{1F19}')) => continue,
            (Some('\u{1F1A}'), Some('\u{1F12}')) => continue,
            (Some('\u{1F12}'), Some('\u{1F1A}')) => continue,
            (Some('\u{1F1B}'), Some('\u{1F13}')) => continue,
            (Some('\u{1F13}'), Some('\u{1F1B}')) => continue,
            (Some('\u{1F1C}'), Some('\u{1F14}')) => continue,
            (Some('\u{1F14}'), Some('\u{1F1C}')) => continue,
            (Some('\u{1F1D}'), Some('\u{1F15}')) => continue,
            (Some('\u{1F15}'), Some('\u{1F1D}')) => continue,
            (Some('\u{1F28}'), Some('\u{1F20}')) => continue,
            (Some('\u{1F20}'), Some('\u{1F28}')) => continue,
            (Some('\u{1F29}'), Some('\u{1F21}')) => continue,
            (Some('\u{1F21}'), Some('\u{1F29}')) => continue,
            (Some('\u{1F2A}'), Some('\u{1F22}')) => continue,
            (Some('\u{1F22}'), Some('\u{1F2A}')) => continue,
            (Some('\u{1F2B}'), Some('\u{1F23}')) => continue,
            (Some('\u{1F23}'), Some('\u{1F2B}')) => continue,
            (Some('\u{1F2C}'), Some('\u{1F24}')) => continue,
            (Some('\u{1F24}'), Some('\u{1F2C}')) => continue,
            (Some('\u{1F2D}'), Some('\u{1F25}')) => continue,
            (Some('\u{1F25}'), Some('\u{1F2D}')) => continue,
            (Some('\u{1F2E}'), Some('\u{1F26}')) => continue,
            (Some('\u{1F26}'), Some('\u{1F2E}')) => continue,
            (Some('\u{1F2F}'), Some('\u{1F27}')) => continue,
            (Some('\u{1F27}'), Some('\u{1F2F}')) => continue,
            (Some('\u{1F38}'), Some('\u{1F30}')) => continue,
            (Some('\u{1F30}'), Some('\u{1F38}')) => continue,
            (Some('\u{1F39}'), Some('\u{1F31}')) => continue,
            (Some('\u{1F31}'), Some('\u{1F39}')) => continue,
            (Some('\u{1F3A}'), Some('\u{1F32}')) => continue,
            (Some('\u{1F32}'), Some('\u{1F3A}')) => continue,
            (Some('\u{1F3B}'), Some('\u{1F33}')) => continue,
            (Some('\u{1F33}'), Some('\u{1F3B}')) => continue,
            (Some('\u{1F3C}'), Some('\u{1F34}')) => continue,
            (Some('\u{1F34}'), Some('\u{1F3C}')) => continue,
            (Some('\u{1F3D}'), Some('\u{1F35}')) => continue,
            (Some('\u{1F35}'), Some('\u{1F3D}')) => continue,
            (Some('\u{1F3E}'), Some('\u{1F36}')) => continue,
            (Some('\u{1F36}'), Some('\u{1F3E}')) => continue,
            (Some('\u{1F3F}'), Some('\u{1F37}')) => continue,
            (Some('\u{1F37}'), Some('\u{1F3F}')) => continue,
            (Some('\u{1F48}'), Some('\u{1F40}')) => continue,
            (Some('\u{1F40}'), Some('\u{1F48}')) => continue,
            (Some('\u{1F49}'), Some('\u{1F41}')) => continue,
            (Some('\u{1F41}'), Some('\u{1F49}')) => continue,
            (Some('\u{1F4A}'), Some('\u{1F42}')) => continue,
            (Some('\u{1F42}'), Some('\u{1F4A}')) => continue,
            (Some('\u{1F4B}'), Some('\u{1F43}')) => continue,
            (Some('\u{1F43}'), Some('\u{1F4B}')) => continue,
            (Some('\u{1F4C}'), Some('\u{1F44}')) => continue,
            (Some('\u{1F44}'), Some('\u{1F4C}')) => continue,
            (Some('\u{1F4D}'), Some('\u{1F45}')) => continue,
            (Some('\u{1F45}'), Some('\u{1F4D}')) => continue,
            (Some('\u{1F50}'), Some('\u{03C5}')) if matches!(right.next(), Some('\u{0313}')) => {
                continue
            }
            (Some('\u{03C5}'), Some('\u{1F50}')) if matches!(left.next(), Some('\u{0313}')) => {
                continue
            }
            (Some('\u{1F52}'), Some('\u{03C5}'))
                if matches!(right.next(), Some('\u{0313}'))
                    && matches!(right.next(), Some('\u{0300}')) =>
            {
                continue
            }
            (Some('\u{03C5}'), Some('\u{1F52}'))
                if matches!(left.next(), Some('\u{0313}'))
                    && matches!(left.next(), Some('\u{0300}')) =>
            {
                continue
            }
            (Some('\u{1F54}'), Some('\u{03C5}'))
                if matches!(right.next(), Some('\u{0313}'))
                    && matches!(right.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{03C5}'), Some('\u{1F54}'))
                if matches!(left.next(), Some('\u{0313}'))
                    && matches!(left.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{1F56}'), Some('\u{03C5}'))
                if matches!(right.next(), Some('\u{0313}'))
                    && matches!(right.next(), Some('\u{0342}')) =>
            {
                continue
            }
            (Some('\u{03C5}'), Some('\u{1F56}'))
                if matches!(left.next(), Some('\u{0313}'))
                    && matches!(left.next(), Some('\u{0342}')) =>
            {
                continue
            }
            (Some('\u{1F59}'), Some('\u{1F51}')) => continue,
            (Some('\u{1F51}'), Some('\u{1F59}')) => continue,
            (Some('\u{1F5B}'), Some('\u{1F53}')) => continue,
            (Some('\u{1F53}'), Some('\u{1F5B}')) => continue,
            (Some('\u{1F5D}'), Some('\u{1F55}')) => continue,
            (Some('\u{1F55}'), Some('\u{1F5D}')) => continue,
            (Some('\u{1F5F}'), Some('\u{1F57}')) => continue,
            (Some('\u{1F57}'), Some('\u{1F5F}')) => continue,
            (Some('\u{1F68}'), Some('\u{1F60}')) => continue,
            (Some('\u{1F60}'), Some('\u{1F68}')) => continue,
            (Some('\u{1F69}'), Some('\u{1F61}')) => continue,
            (Some('\u{1F61}'), Some('\u{1F69}')) => continue,
            (Some('\u{1F6A}'), Some('\u{1F62}')) => continue,
            (Some('\u{1F62}'), Some('\u{1F6A}')) => continue,
            (Some('\u{1F6B}'), Some('\u{1F63}')) => continue,
            (Some('\u{1F63}'), Some('\u{1F6B}')) => continue,
            (Some('\u{1F6C}'), Some('\u{1F64}')) => continue,
            (Some('\u{1F64}'), Some('\u{1F6C}')) => continue,
            (Some('\u{1F6D}'), Some('\u{1F65}')) => continue,
            (Some('\u{1F65}'), Some('\u{1F6D}')) => continue,
            (Some('\u{1F6E}'), Some('\u{1F66}')) => continue,
            (Some('\u{1F66}'), Some('\u{1F6E}')) => continue,
            (Some('\u{1F6F}'), Some('\u{1F67}')) => continue,
            (Some('\u{1F67}'), Some('\u{1F6F}')) => continue,
            (Some('\u{1F80}'), Some('\u{1F00}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F00}'), Some('\u{1F80}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F81}'), Some('\u{1F01}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F01}'), Some('\u{1F81}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F82}'), Some('\u{1F02}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F02}'), Some('\u{1F82}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F83}'), Some('\u{1F03}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F03}'), Some('\u{1F83}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F84}'), Some('\u{1F04}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F04}'), Some('\u{1F84}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F85}'), Some('\u{1F05}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F05}'), Some('\u{1F85}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F86}'), Some('\u{1F06}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F06}'), Some('\u{1F86}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F87}'), Some('\u{1F07}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F07}'), Some('\u{1F87}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F88}'), Some('\u{1F00}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F00}'), Some('\u{1F88}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F89}'), Some('\u{1F01}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F01}'), Some('\u{1F89}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F8A}'), Some('\u{1F02}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F02}'), Some('\u{1F8A}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F8B}'), Some('\u{1F03}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F03}'), Some('\u{1F8B}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F8C}'), Some('\u{1F04}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F04}'), Some('\u{1F8C}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F8D}'), Some('\u{1F05}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F05}'), Some('\u{1F8D}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F8E}'), Some('\u{1F06}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F06}'), Some('\u{1F8E}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F8F}'), Some('\u{1F07}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F07}'), Some('\u{1F8F}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F90}'), Some('\u{1F20}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F20}'), Some('\u{1F90}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F91}'), Some('\u{1F21}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F21}'), Some('\u{1F91}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F92}'), Some('\u{1F22}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F22}'), Some('\u{1F92}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F93}'), Some('\u{1F23}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F23}'), Some('\u{1F93}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F94}'), Some('\u{1F24}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F24}'), Some('\u{1F94}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F95}'), Some('\u{1F25}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F25}'), Some('\u{1F95}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F96}'), Some('\u{1F26}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F26}'), Some('\u{1F96}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F97}'), Some('\u{1F27}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F27}'), Some('\u{1F97}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F98}'), Some('\u{1F20}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F20}'), Some('\u{1F98}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F99}'), Some('\u{1F21}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F21}'), Some('\u{1F99}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F9A}'), Some('\u{1F22}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F22}'), Some('\u{1F9A}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F9B}'), Some('\u{1F23}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F23}'), Some('\u{1F9B}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F9C}'), Some('\u{1F24}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F24}'), Some('\u{1F9C}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F9D}'), Some('\u{1F25}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F25}'), Some('\u{1F9D}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F9E}'), Some('\u{1F26}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F26}'), Some('\u{1F9E}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F9F}'), Some('\u{1F27}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F27}'), Some('\u{1F9F}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA0}'), Some('\u{1F60}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F60}'), Some('\u{1FA0}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA1}'), Some('\u{1F61}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F61}'), Some('\u{1FA1}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA2}'), Some('\u{1F62}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F62}'), Some('\u{1FA2}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA3}'), Some('\u{1F63}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F63}'), Some('\u{1FA3}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA4}'), Some('\u{1F64}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F64}'), Some('\u{1FA4}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA5}'), Some('\u{1F65}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F65}'), Some('\u{1FA5}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA6}'), Some('\u{1F66}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F66}'), Some('\u{1FA6}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA7}'), Some('\u{1F67}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F67}'), Some('\u{1FA7}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA8}'), Some('\u{1F60}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F60}'), Some('\u{1FA8}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FA9}'), Some('\u{1F61}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F61}'), Some('\u{1FA9}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FAA}'), Some('\u{1F62}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F62}'), Some('\u{1FAA}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FAB}'), Some('\u{1F63}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F63}'), Some('\u{1FAB}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FAC}'), Some('\u{1F64}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F64}'), Some('\u{1FAC}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FAD}'), Some('\u{1F65}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F65}'), Some('\u{1FAD}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FAE}'), Some('\u{1F66}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F66}'), Some('\u{1FAE}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FAF}'), Some('\u{1F67}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F67}'), Some('\u{1FAF}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FB2}'), Some('\u{1F70}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F70}'), Some('\u{1FB2}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FB3}'), Some('\u{03B1}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03B1}'), Some('\u{1FB3}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FB4}'), Some('\u{03AC}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03AC}'), Some('\u{1FB4}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FB6}'), Some('\u{03B1}')) if matches!(right.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{03B1}'), Some('\u{1FB6}')) if matches!(left.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{1FB7}'), Some('\u{03B1}'))
                if matches!(right.next(), Some('\u{0342}'))
                    && matches!(right.next(), Some('\u{03B9}')) =>
            {
                continue
            }
            (Some('\u{03B1}'), Some('\u{1FB7}'))
                if matches!(left.next(), Some('\u{0342}'))
                    && matches!(left.next(), Some('\u{03B9}')) =>
            {
                continue
            }
            (Some('\u{1FB8}'), Some('\u{1FB0}')) => continue,
            (Some('\u{1FB0}'), Some('\u{1FB8}')) => continue,
            (Some('\u{1FB9}'), Some('\u{1FB1}')) => continue,
            (Some('\u{1FB1}'), Some('\u{1FB9}')) => continue,
            (Some('\u{1FBA}'), Some('\u{1F70}')) => continue,
            (Some('\u{1F70}'), Some('\u{1FBA}')) => continue,
            (Some('\u{1FBB}'), Some('\u{1F71}')) => continue,
            (Some('\u{1F71}'), Some('\u{1FBB}')) => continue,
            (Some('\u{1FBC}'), Some('\u{03B1}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03B1}'), Some('\u{1FBC}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FBE}'), Some('\u{03B9}')) => continue,
            (Some('\u{03B9}'), Some('\u{1FBE}')) => continue,
            (Some('\u{1FC2}'), Some('\u{1F74}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F74}'), Some('\u{1FC2}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FC3}'), Some('\u{03B7}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03B7}'), Some('\u{1FC3}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FC4}'), Some('\u{03AE}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03AE}'), Some('\u{1FC4}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FC6}'), Some('\u{03B7}')) if matches!(right.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{03B7}'), Some('\u{1FC6}')) if matches!(left.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{1FC7}'), Some('\u{03B7}'))
                if matches!(right.next(), Some('\u{0342}'))
                    && matches!(right.next(), Some('\u{03B9}')) =>
            {
                continue
            }
            (Some('\u{03B7}'), Some('\u{1FC7}'))
                if matches!(left.next(), Some('\u{0342}'))
                    && matches!(left.next(), Some('\u{03B9}')) =>
            {
                continue
            }
            (Some('\u{1FC8}'), Some('\u{1F72}')) => continue,
            (Some('\u{1F72}'), Some('\u{1FC8}')) => continue,
            (Some('\u{1FC9}'), Some('\u{1F73}')) => continue,
            (Some('\u{1F73}'), Some('\u{1FC9}')) => continue,
            (Some('\u{1FCA}'), Some('\u{1F74}')) => continue,
            (Some('\u{1F74}'), Some('\u{1FCA}')) => continue,
            (Some('\u{1FCB}'), Some('\u{1F75}')) => continue,
            (Some('\u{1F75}'), Some('\u{1FCB}')) => continue,
            (Some('\u{1FCC}'), Some('\u{03B7}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03B7}'), Some('\u{1FCC}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FD2}'), Some('\u{03B9}'))
                if matches!(right.next(), Some('\u{0308}'))
                    && matches!(right.next(), Some('\u{0300}')) =>
            {
                continue
            }
            (Some('\u{03B9}'), Some('\u{1FD2}'))
                if matches!(left.next(), Some('\u{0308}'))
                    && matches!(left.next(), Some('\u{0300}')) =>
            {
                continue
            }
            (Some('\u{1FD3}'), Some('\u{03B9}'))
                if matches!(right.next(), Some('\u{0308}'))
                    && matches!(right.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{03B9}'), Some('\u{1FD3}'))
                if matches!(left.next(), Some('\u{0308}'))
                    && matches!(left.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{1FD6}'), Some('\u{03B9}')) if matches!(right.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{03B9}'), Some('\u{1FD6}')) if matches!(left.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{1FD7}'), Some('\u{03B9}'))
                if matches!(right.next(), Some('\u{0308}'))
                    && matches!(right.next(), Some('\u{0342}')) =>
            {
                continue
            }
            (Some('\u{03B9}'), Some('\u{1FD7}'))
                if matches!(left.next(), Some('\u{0308}'))
                    && matches!(left.next(), Some('\u{0342}')) =>
            {
                continue
            }
            (Some('\u{1FD8}'), Some('\u{1FD0}')) => continue,
            (Some('\u{1FD0}'), Some('\u{1FD8}')) => continue,
            (Some('\u{1FD9}'), Some('\u{1FD1}')) => continue,
            (Some('\u{1FD1}'), Some('\u{1FD9}')) => continue,
            (Some('\u{1FDA}'), Some('\u{1F76}')) => continue,
            (Some('\u{1F76}'), Some('\u{1FDA}')) => continue,
            (Some('\u{1FDB}'), Some('\u{1F77}')) => continue,
            (Some('\u{1F77}'), Some('\u{1FDB}')) => continue,
            (Some('\u{1FE2}'), Some('\u{03C5}'))
                if matches!(right.next(), Some('\u{0308}'))
                    && matches!(right.next(), Some('\u{0300}')) =>
            {
                continue
            }
            (Some('\u{03C5}'), Some('\u{1FE2}'))
                if matches!(left.next(), Some('\u{0308}'))
                    && matches!(left.next(), Some('\u{0300}')) =>
            {
                continue
            }
            (Some('\u{1FE3}'), Some('\u{03C5}'))
                if matches!(right.next(), Some('\u{0308}'))
                    && matches!(right.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{03C5}'), Some('\u{1FE3}'))
                if matches!(left.next(), Some('\u{0308}'))
                    && matches!(left.next(), Some('\u{0301}')) =>
            {
                continue
            }
            (Some('\u{1FE4}'), Some('\u{03C1}')) if matches!(right.next(), Some('\u{0313}')) => {
                continue
            }
            (Some('\u{03C1}'), Some('\u{1FE4}')) if matches!(left.next(), Some('\u{0313}')) => {
                continue
            }
            (Some('\u{1FE6}'), Some('\u{03C5}')) if matches!(right.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{03C5}'), Some('\u{1FE6}')) if matches!(left.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{1FE7}'), Some('\u{03C5}'))
                if matches!(right.next(), Some('\u{0308}'))
                    && matches!(right.next(), Some('\u{0342}')) =>
            {
                continue
            }
            (Some('\u{03C5}'), Some('\u{1FE7}'))
                if matches!(left.next(), Some('\u{0308}'))
                    && matches!(left.next(), Some('\u{0342}')) =>
            {
                continue
            }
            (Some('\u{1FE8}'), Some('\u{1FE0}')) => continue,
            (Some('\u{1FE0}'), Some('\u{1FE8}')) => continue,
            (Some('\u{1FE9}'), Some('\u{1FE1}')) => continue,
            (Some('\u{1FE1}'), Some('\u{1FE9}')) => continue,
            (Some('\u{1FEA}'), Some('\u{1F7A}')) => continue,
            (Some('\u{1F7A}'), Some('\u{1FEA}')) => continue,
            (Some('\u{1FEB}'), Some('\u{1F7B}')) => continue,
            (Some('\u{1F7B}'), Some('\u{1FEB}')) => continue,
            (Some('\u{1FEC}'), Some('\u{1FE5}')) => continue,
            (Some('\u{1FE5}'), Some('\u{1FEC}')) => continue,
            (Some('\u{1FF2}'), Some('\u{1F7C}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1F7C}'), Some('\u{1FF2}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FF3}'), Some('\u{03C9}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03C9}'), Some('\u{1FF3}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FF4}'), Some('\u{03CE}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03CE}'), Some('\u{1FF4}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{1FF6}'), Some('\u{03C9}')) if matches!(right.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{03C9}'), Some('\u{1FF6}')) if matches!(left.next(), Some('\u{0342}')) => {
                continue
            }
            (Some('\u{1FF7}'), Some('\u{03C9}'))
                if matches!(right.next(), Some('\u{0342}'))
                    && matches!(right.next(), Some('\u{03B9}')) =>
            {
                continue
            }
            (Some('\u{03C9}'), Some('\u{1FF7}'))
                if matches!(left.next(), Some('\u{0342}'))
                    && matches!(left.next(), Some('\u{03B9}')) =>
            {
                continue
            }
            (Some('\u{1FF8}'), Some('\u{1F78}')) => continue,
            (Some('\u{1F78}'), Some('\u{1FF8}')) => continue,
            (Some('\u{1FF9}'), Some('\u{1F79}')) => continue,
            (Some('\u{1F79}'), Some('\u{1FF9}')) => continue,
            (Some('\u{1FFA}'), Some('\u{1F7C}')) => continue,
            (Some('\u{1F7C}'), Some('\u{1FFA}')) => continue,
            (Some('\u{1FFB}'), Some('\u{1F7D}')) => continue,
            (Some('\u{1F7D}'), Some('\u{1FFB}')) => continue,
            (Some('\u{1FFC}'), Some('\u{03C9}')) if matches!(right.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{03C9}'), Some('\u{1FFC}')) if matches!(left.next(), Some('\u{03B9}')) => {
                continue
            }
            (Some('\u{2126}'), Some('\u{03C9}')) => continue,
            (Some('\u{03C9}'), Some('\u{2126}')) => continue,
            (Some('\u{212A}'), Some('\u{006B}')) => continue,
            (Some('\u{006B}'), Some('\u{212A}')) => continue,
            (Some('\u{212B}'), Some('\u{00E5}')) => continue,
            (Some('\u{00E5}'), Some('\u{212B}')) => continue,
            (Some('\u{2132}'), Some('\u{214E}')) => continue,
            (Some('\u{214E}'), Some('\u{2132}')) => continue,
            (Some('\u{2160}'), Some('\u{2170}')) => continue,
            (Some('\u{2170}'), Some('\u{2160}')) => continue,
            (Some('\u{2161}'), Some('\u{2171}')) => continue,
            (Some('\u{2171}'), Some('\u{2161}')) => continue,
            (Some('\u{2162}'), Some('\u{2172}')) => continue,
            (Some('\u{2172}'), Some('\u{2162}')) => continue,
            (Some('\u{2163}'), Some('\u{2173}')) => continue,
            (Some('\u{2173}'), Some('\u{2163}')) => continue,
            (Some('\u{2164}'), Some('\u{2174}')) => continue,
            (Some('\u{2174}'), Some('\u{2164}')) => continue,
            (Some('\u{2165}'), Some('\u{2175}')) => continue,
            (Some('\u{2175}'), Some('\u{2165}')) => continue,
            (Some('\u{2166}'), Some('\u{2176}')) => continue,
            (Some('\u{2176}'), Some('\u{2166}')) => continue,
            (Some('\u{2167}'), Some('\u{2177}')) => continue,
            (Some('\u{2177}'), Some('\u{2167}')) => continue,
            (Some('\u{2168}'), Some('\u{2178}')) => continue,
            (Some('\u{2178}'), Some('\u{2168}')) => continue,
            (Some('\u{2169}'), Some('\u{2179}')) => continue,
            (Some('\u{2179}'), Some('\u{2169}')) => continue,
            (Some('\u{216A}'), Some('\u{217A}')) => continue,
            (Some('\u{217A}'), Some('\u{216A}')) => continue,
            (Some('\u{216B}'), Some('\u{217B}')) => continue,
            (Some('\u{217B}'), Some('\u{216B}')) => continue,
            (Some('\u{216C}'), Some('\u{217C}')) => continue,
            (Some('\u{217C}'), Some('\u{216C}')) => continue,
            (Some('\u{216D}'), Some('\u{217D}')) => continue,
            (Some('\u{217D}'), Some('\u{216D}')) => continue,
            (Some('\u{216E}'), Some('\u{217E}')) => continue,
            (Some('\u{217E}'), Some('\u{216E}')) => continue,
            (Some('\u{216F}'), Some('\u{217F}')) => continue,
            (Some('\u{217F}'), Some('\u{216F}')) => continue,
            (Some('\u{2183}'), Some('\u{2184}')) => continue,
            (Some('\u{2184}'), Some('\u{2183}')) => continue,
            (Some('\u{24B6}'), Some('\u{24D0}')) => continue,
            (Some('\u{24D0}'), Some('\u{24B6}')) => continue,
            (Some('\u{24B7}'), Some('\u{24D1}')) => continue,
            (Some('\u{24D1}'), Some('\u{24B7}')) => continue,
            (Some('\u{24B8}'), Some('\u{24D2}')) => continue,
            (Some('\u{24D2}'), Some('\u{24B8}')) => continue,
            (Some('\u{24B9}'), Some('\u{24D3}')) => continue,
            (Some('\u{24D3}'), Some('\u{24B9}')) => continue,
            (Some('\u{24BA}'), Some('\u{24D4}')) => continue,
            (Some('\u{24D4}'), Some('\u{24BA}')) => continue,
            (Some('\u{24BB}'), Some('\u{24D5}')) => continue,
            (Some('\u{24D5}'), Some('\u{24BB}')) => continue,
            (Some('\u{24BC}'), Some('\u{24D6}')) => continue,
            (Some('\u{24D6}'), Some('\u{24BC}')) => continue,
            (Some('\u{24BD}'), Some('\u{24D7}')) => continue,
            (Some('\u{24D7}'), Some('\u{24BD}')) => continue,
            (Some('\u{24BE}'), Some('\u{24D8}')) => continue,
            (Some('\u{24D8}'), Some('\u{24BE}')) => continue,
            (Some('\u{24BF}'), Some('\u{24D9}')) => continue,
            (Some('\u{24D9}'), Some('\u{24BF}')) => continue,
            (Some('\u{24C0}'), Some('\u{24DA}')) => continue,
            (Some('\u{24DA}'), Some('\u{24C0}')) => continue,
            (Some('\u{24C1}'), Some('\u{24DB}')) => continue,
            (Some('\u{24DB}'), Some('\u{24C1}')) => continue,
            (Some('\u{24C2}'), Some('\u{24DC}')) => continue,
            (Some('\u{24DC}'), Some('\u{24C2}')) => continue,
            (Some('\u{24C3}'), Some('\u{24DD}')) => continue,
            (Some('\u{24DD}'), Some('\u{24C3}')) => continue,
            (Some('\u{24C4}'), Some('\u{24DE}')) => continue,
            (Some('\u{24DE}'), Some('\u{24C4}')) => continue,
            (Some('\u{24C5}'), Some('\u{24DF}')) => continue,
            (Some('\u{24DF}'), Some('\u{24C5}')) => continue,
            (Some('\u{24C6}'), Some('\u{24E0}')) => continue,
            (Some('\u{24E0}'), Some('\u{24C6}')) => continue,
            (Some('\u{24C7}'), Some('\u{24E1}')) => continue,
            (Some('\u{24E1}'), Some('\u{24C7}')) => continue,
            (Some('\u{24C8}'), Some('\u{24E2}')) => continue,
            (Some('\u{24E2}'), Some('\u{24C8}')) => continue,
            (Some('\u{24C9}'), Some('\u{24E3}')) => continue,
            (Some('\u{24E3}'), Some('\u{24C9}')) => continue,
            (Some('\u{24CA}'), Some('\u{24E4}')) => continue,
            (Some('\u{24E4}'), Some('\u{24CA}')) => continue,
            (Some('\u{24CB}'), Some('\u{24E5}')) => continue,
            (Some('\u{24E5}'), Some('\u{24CB}')) => continue,
            (Some('\u{24CC}'), Some('\u{24E6}')) => continue,
            (Some('\u{24E6}'), Some('\u{24CC}')) => continue,
            (Some('\u{24CD}'), Some('\u{24E7}')) => continue,
            (Some('\u{24E7}'), Some('\u{24CD}')) => continue,
            (Some('\u{24CE}'), Some('\u{24E8}')) => continue,
            (Some('\u{24E8}'), Some('\u{24CE}')) => continue,
            (Some('\u{24CF}'), Some('\u{24E9}')) => continue,
            (Some('\u{24E9}'), Some('\u{24CF}')) => continue,
            (Some('\u{2C00}'), Some('\u{2C30}')) => continue,
            (Some('\u{2C30}'), Some('\u{2C00}')) => continue,
            (Some('\u{2C01}'), Some('\u{2C31}')) => continue,
            (Some('\u{2C31}'), Some('\u{2C01}')) => continue,
            (Some('\u{2C02}'), Some('\u{2C32}')) => continue,
            (Some('\u{2C32}'), Some('\u{2C02}')) => continue,
            (Some('\u{2C03}'), Some('\u{2C33}')) => continue,
            (Some('\u{2C33}'), Some('\u{2C03}')) => continue,
            (Some('\u{2C04}'), Some('\u{2C34}')) => continue,
            (Some('\u{2C34}'), Some('\u{2C04}')) => continue,
            (Some('\u{2C05}'), Some('\u{2C35}')) => continue,
            (Some('\u{2C35}'), Some('\u{2C05}')) => continue,
            (Some('\u{2C06}'), Some('\u{2C36}')) => continue,
            (Some('\u{2C36}'), Some('\u{2C06}')) => continue,
            (Some('\u{2C07}'), Some('\u{2C37}')) => continue,
            (Some('\u{2C37}'), Some('\u{2C07}')) => continue,
            (Some('\u{2C08}'), Some('\u{2C38}')) => continue,
            (Some('\u{2C38}'), Some('\u{2C08}')) => continue,
            (Some('\u{2C09}'), Some('\u{2C39}')) => continue,
            (Some('\u{2C39}'), Some('\u{2C09}')) => continue,
            (Some('\u{2C0A}'), Some('\u{2C3A}')) => continue,
            (Some('\u{2C3A}'), Some('\u{2C0A}')) => continue,
            (Some('\u{2C0B}'), Some('\u{2C3B}')) => continue,
            (Some('\u{2C3B}'), Some('\u{2C0B}')) => continue,
            (Some('\u{2C0C}'), Some('\u{2C3C}')) => continue,
            (Some('\u{2C3C}'), Some('\u{2C0C}')) => continue,
            (Some('\u{2C0D}'), Some('\u{2C3D}')) => continue,
            (Some('\u{2C3D}'), Some('\u{2C0D}')) => continue,
            (Some('\u{2C0E}'), Some('\u{2C3E}')) => continue,
            (Some('\u{2C3E}'), Some('\u{2C0E}')) => continue,
            (Some('\u{2C0F}'), Some('\u{2C3F}')) => continue,
            (Some('\u{2C3F}'), Some('\u{2C0F}')) => continue,
            (Some('\u{2C10}'), Some('\u{2C40}')) => continue,
            (Some('\u{2C40}'), Some('\u{2C10}')) => continue,
            (Some('\u{2C11}'), Some('\u{2C41}')) => continue,
            (Some('\u{2C41}'), Some('\u{2C11}')) => continue,
            (Some('\u{2C12}'), Some('\u{2C42}')) => continue,
            (Some('\u{2C42}'), Some('\u{2C12}')) => continue,
            (Some('\u{2C13}'), Some('\u{2C43}')) => continue,
            (Some('\u{2C43}'), Some('\u{2C13}')) => continue,
            (Some('\u{2C14}'), Some('\u{2C44}')) => continue,
            (Some('\u{2C44}'), Some('\u{2C14}')) => continue,
            (Some('\u{2C15}'), Some('\u{2C45}')) => continue,
            (Some('\u{2C45}'), Some('\u{2C15}')) => continue,
            (Some('\u{2C16}'), Some('\u{2C46}')) => continue,
            (Some('\u{2C46}'), Some('\u{2C16}')) => continue,
            (Some('\u{2C17}'), Some('\u{2C47}')) => continue,
            (Some('\u{2C47}'), Some('\u{2C17}')) => continue,
            (Some('\u{2C18}'), Some('\u{2C48}')) => continue,
            (Some('\u{2C48}'), Some('\u{2C18}')) => continue,
            (Some('\u{2C19}'), Some('\u{2C49}')) => continue,
            (Some('\u{2C49}'), Some('\u{2C19}')) => continue,
            (Some('\u{2C1A}'), Some('\u{2C4A}')) => continue,
            (Some('\u{2C4A}'), Some('\u{2C1A}')) => continue,
            (Some('\u{2C1B}'), Some('\u{2C4B}')) => continue,
            (Some('\u{2C4B}'), Some('\u{2C1B}')) => continue,
            (Some('\u{2C1C}'), Some('\u{2C4C}')) => continue,
            (Some('\u{2C4C}'), Some('\u{2C1C}')) => continue,
            (Some('\u{2C1D}'), Some('\u{2C4D}')) => continue,
            (Some('\u{2C4D}'), Some('\u{2C1D}')) => continue,
            (Some('\u{2C1E}'), Some('\u{2C4E}')) => continue,
            (Some('\u{2C4E}'), Some('\u{2C1E}')) => continue,
            (Some('\u{2C1F}'), Some('\u{2C4F}')) => continue,
            (Some('\u{2C4F}'), Some('\u{2C1F}')) => continue,
            (Some('\u{2C20}'), Some('\u{2C50}')) => continue,
            (Some('\u{2C50}'), Some('\u{2C20}')) => continue,
            (Some('\u{2C21}'), Some('\u{2C51}')) => continue,
            (Some('\u{2C51}'), Some('\u{2C21}')) => continue,
            (Some('\u{2C22}'), Some('\u{2C52}')) => continue,
            (Some('\u{2C52}'), Some('\u{2C22}')) => continue,
            (Some('\u{2C23}'), Some('\u{2C53}')) => continue,
            (Some('\u{2C53}'), Some('\u{2C23}')) => continue,
            (Some('\u{2C24}'), Some('\u{2C54}')) => continue,
            (Some('\u{2C54}'), Some('\u{2C24}')) => continue,
            (Some('\u{2C25}'), Some('\u{2C55}')) => continue,
            (Some('\u{2C55}'), Some('\u{2C25}')) => continue,
            (Some('\u{2C26}'), Some('\u{2C56}')) => continue,
            (Some('\u{2C56}'), Some('\u{2C26}')) => continue,
            (Some('\u{2C27}'), Some('\u{2C57}')) => continue,
            (Some('\u{2C57}'), Some('\u{2C27}')) => continue,
            (Some('\u{2C28}'), Some('\u{2C58}')) => continue,
            (Some('\u{2C58}'), Some('\u{2C28}')) => continue,
            (Some('\u{2C29}'), Some('\u{2C59}')) => continue,
            (Some('\u{2C59}'), Some('\u{2C29}')) => continue,
            (Some('\u{2C2A}'), Some('\u{2C5A}')) => continue,
            (Some('\u{2C5A}'), Some('\u{2C2A}')) => continue,
            (Some('\u{2C2B}'), Some('\u{2C5B}')) => continue,
            (Some('\u{2C5B}'), Some('\u{2C2B}')) => continue,
            (Some('\u{2C2C}'), Some('\u{2C5C}')) => continue,
            (Some('\u{2C5C}'), Some('\u{2C2C}')) => continue,
            (Some('\u{2C2D}'), Some('\u{2C5D}')) => continue,
            (Some('\u{2C5D}'), Some('\u{2C2D}')) => continue,
            (Some('\u{2C2E}'), Some('\u{2C5E}')) => continue,
            (Some('\u{2C5E}'), Some('\u{2C2E}')) => continue,
            (Some('\u{2C60}'), Some('\u{2C61}')) => continue,
            (Some('\u{2C61}'), Some('\u{2C60}')) => continue,
            (Some('\u{2C62}'), Some('\u{026B}')) => continue,
            (Some('\u{026B}'), Some('\u{2C62}')) => continue,
            (Some('\u{2C63}'), Some('\u{1D7D}')) => continue,
            (Some('\u{1D7D}'), Some('\u{2C63}')) => continue,
            (Some('\u{2C64}'), Some('\u{027D}')) => continue,
            (Some('\u{027D}'), Some('\u{2C64}')) => continue,
            (Some('\u{2C67}'), Some('\u{2C68}')) => continue,
            (Some('\u{2C68}'), Some('\u{2C67}')) => continue,
            (Some('\u{2C69}'), Some('\u{2C6A}')) => continue,
            (Some('\u{2C6A}'), Some('\u{2C69}')) => continue,
            (Some('\u{2C6B}'), Some('\u{2C6C}')) => continue,
            (Some('\u{2C6C}'), Some('\u{2C6B}')) => continue,
            (Some('\u{2C6D}'), Some('\u{0251}')) => continue,
            (Some('\u{0251}'), Some('\u{2C6D}')) => continue,
            (Some('\u{2C6E}'), Some('\u{0271}')) => continue,
            (Some('\u{0271}'), Some('\u{2C6E}')) => continue,
            (Some('\u{2C6F}'), Some('\u{0250}')) => continue,
            (Some('\u{0250}'), Some('\u{2C6F}')) => continue,
            (Some('\u{2C70}'), Some('\u{0252}')) => continue,
            (Some('\u{0252}'), Some('\u{2C70}')) => continue,
            (Some('\u{2C72}'), Some('\u{2C73}')) => continue,
            (Some('\u{2C73}'), Some('\u{2C72}')) => continue,
            (Some('\u{2C75}'), Some('\u{2C76}')) => continue,
            (Some('\u{2C76}'), Some('\u{2C75}')) => continue,
            (Some('\u{2C7E}'), Some('\u{023F}')) => continue,
            (Some('\u{023F}'), Some('\u{2C7E}')) => continue,
            (Some('\u{2C7F}'), Some('\u{0240}')) => continue,
            (Some('\u{0240}'), Some('\u{2C7F}')) => continue,
            (Some('\u{2C80}'), Some('\u{2C81}')) => continue,
            (Some('\u{2C81}'), Some('\u{2C80}')) => continue,
            (Some('\u{2C82}'), Some('\u{2C83}')) => continue,
            (Some('\u{2C83}'), Some('\u{2C82}')) => continue,
            (Some('\u{2C84}'), Some('\u{2C85}')) => continue,
            (Some('\u{2C85}'), Some('\u{2C84}')) => continue,
            (Some('\u{2C86}'), Some('\u{2C87}')) => continue,
            (Some('\u{2C87}'), Some('\u{2C86}')) => continue,
            (Some('\u{2C88}'), Some('\u{2C89}')) => continue,
            (Some('\u{2C89}'), Some('\u{2C88}')) => continue,
            (Some('\u{2C8A}'), Some('\u{2C8B}')) => continue,
            (Some('\u{2C8B}'), Some('\u{2C8A}')) => continue,
            (Some('\u{2C8C}'), Some('\u{2C8D}')) => continue,
            (Some('\u{2C8D}'), Some('\u{2C8C}')) => continue,
            (Some('\u{2C8E}'), Some('\u{2C8F}')) => continue,
            (Some('\u{2C8F}'), Some('\u{2C8E}')) => continue,
            (Some('\u{2C90}'), Some('\u{2C91}')) => continue,
            (Some('\u{2C91}'), Some('\u{2C90}')) => continue,
            (Some('\u{2C92}'), Some('\u{2C93}')) => continue,
            (Some('\u{2C93}'), Some('\u{2C92}')) => continue,
            (Some('\u{2C94}'), Some('\u{2C95}')) => continue,
            (Some('\u{2C95}'), Some('\u{2C94}')) => continue,
            (Some('\u{2C96}'), Some('\u{2C97}')) => continue,
            (Some('\u{2C97}'), Some('\u{2C96}')) => continue,
            (Some('\u{2C98}'), Some('\u{2C99}')) => continue,
            (Some('\u{2C99}'), Some('\u{2C98}')) => continue,
            (Some('\u{2C9A}'), Some('\u{2C9B}')) => continue,
            (Some('\u{2C9B}'), Some('\u{2C9A}')) => continue,
            (Some('\u{2C9C}'), Some('\u{2C9D}')) => continue,
            (Some('\u{2C9D}'), Some('\u{2C9C}')) => continue,
            (Some('\u{2C9E}'), Some('\u{2C9F}')) => continue,
            (Some('\u{2C9F}'), Some('\u{2C9E}')) => continue,
            (Some('\u{2CA0}'), Some('\u{2CA1}')) => continue,
            (Some('\u{2CA1}'), Some('\u{2CA0}')) => continue,
            (Some('\u{2CA2}'), Some('\u{2CA3}')) => continue,
            (Some('\u{2CA3}'), Some('\u{2CA2}')) => continue,
            (Some('\u{2CA4}'), Some('\u{2CA5}')) => continue,
            (Some('\u{2CA5}'), Some('\u{2CA4}')) => continue,
            (Some('\u{2CA6}'), Some('\u{2CA7}')) => continue,
            (Some('\u{2CA7}'), Some('\u{2CA6}')) => continue,
            (Some('\u{2CA8}'), Some('\u{2CA9}')) => continue,
            (Some('\u{2CA9}'), Some('\u{2CA8}')) => continue,
            (Some('\u{2CAA}'), Some('\u{2CAB}')) => continue,
            (Some('\u{2CAB}'), Some('\u{2CAA}')) => continue,
            (Some('\u{2CAC}'), Some('\u{2CAD}')) => continue,
            (Some('\u{2CAD}'), Some('\u{2CAC}')) => continue,
            (Some('\u{2CAE}'), Some('\u{2CAF}')) => continue,
            (Some('\u{2CAF}'), Some('\u{2CAE}')) => continue,
            (Some('\u{2CB0}'), Some('\u{2CB1}')) => continue,
            (Some('\u{2CB1}'), Some('\u{2CB0}')) => continue,
            (Some('\u{2CB2}'), Some('\u{2CB3}')) => continue,
            (Some('\u{2CB3}'), Some('\u{2CB2}')) => continue,
            (Some('\u{2CB4}'), Some('\u{2CB5}')) => continue,
            (Some('\u{2CB5}'), Some('\u{2CB4}')) => continue,
            (Some('\u{2CB6}'), Some('\u{2CB7}')) => continue,
            (Some('\u{2CB7}'), Some('\u{2CB6}')) => continue,
            (Some('\u{2CB8}'), Some('\u{2CB9}')) => continue,
            (Some('\u{2CB9}'), Some('\u{2CB8}')) => continue,
            (Some('\u{2CBA}'), Some('\u{2CBB}')) => continue,
            (Some('\u{2CBB}'), Some('\u{2CBA}')) => continue,
            (Some('\u{2CBC}'), Some('\u{2CBD}')) => continue,
            (Some('\u{2CBD}'), Some('\u{2CBC}')) => continue,
            (Some('\u{2CBE}'), Some('\u{2CBF}')) => continue,
            (Some('\u{2CBF}'), Some('\u{2CBE}')) => continue,
            (Some('\u{2CC0}'), Some('\u{2CC1}')) => continue,
            (Some('\u{2CC1}'), Some('\u{2CC0}')) => continue,
            (Some('\u{2CC2}'), Some('\u{2CC3}')) => continue,
            (Some('\u{2CC3}'), Some('\u{2CC2}')) => continue,
            (Some('\u{2CC4}'), Some('\u{2CC5}')) => continue,
            (Some('\u{2CC5}'), Some('\u{2CC4}')) => continue,
            (Some('\u{2CC6}'), Some('\u{2CC7}')) => continue,
            (Some('\u{2CC7}'), Some('\u{2CC6}')) => continue,
            (Some('\u{2CC8}'), Some('\u{2CC9}')) => continue,
            (Some('\u{2CC9}'), Some('\u{2CC8}')) => continue,
            (Some('\u{2CCA}'), Some('\u{2CCB}')) => continue,
            (Some('\u{2CCB}'), Some('\u{2CCA}')) => continue,
            (Some('\u{2CCC}'), Some('\u{2CCD}')) => continue,
            (Some('\u{2CCD}'), Some('\u{2CCC}')) => continue,
            (Some('\u{2CCE}'), Some('\u{2CCF}')) => continue,
            (Some('\u{2CCF}'), Some('\u{2CCE}')) => continue,
            (Some('\u{2CD0}'), Some('\u{2CD1}')) => continue,
            (Some('\u{2CD1}'), Some('\u{2CD0}')) => continue,
            (Some('\u{2CD2}'), Some('\u{2CD3}')) => continue,
            (Some('\u{2CD3}'), Some('\u{2CD2}')) => continue,
            (Some('\u{2CD4}'), Some('\u{2CD5}')) => continue,
            (Some('\u{2CD5}'), Some('\u{2CD4}')) => continue,
            (Some('\u{2CD6}'), Some('\u{2CD7}')) => continue,
            (Some('\u{2CD7}'), Some('\u{2CD6}')) => continue,
            (Some('\u{2CD8}'), Some('\u{2CD9}')) => continue,
            (Some('\u{2CD9}'), Some('\u{2CD8}')) => continue,
            (Some('\u{2CDA}'), Some('\u{2CDB}')) => continue,
            (Some('\u{2CDB}'), Some('\u{2CDA}')) => continue,
            (Some('\u{2CDC}'), Some('\u{2CDD}')) => continue,
            (Some('\u{2CDD}'), Some('\u{2CDC}')) => continue,
            (Some('\u{2CDE}'), Some('\u{2CDF}')) => continue,
            (Some('\u{2CDF}'), Some('\u{2CDE}')) => continue,
            (Some('\u{2CE0}'), Some('\u{2CE1}')) => continue,
            (Some('\u{2CE1}'), Some('\u{2CE0}')) => continue,
            (Some('\u{2CE2}'), Some('\u{2CE3}')) => continue,
            (Some('\u{2CE3}'), Some('\u{2CE2}')) => continue,
            (Some('\u{2CEB}'), Some('\u{2CEC}')) => continue,
            (Some('\u{2CEC}'), Some('\u{2CEB}')) => continue,
            (Some('\u{2CED}'), Some('\u{2CEE}')) => continue,
            (Some('\u{2CEE}'), Some('\u{2CED}')) => continue,
            (Some('\u{2CF2}'), Some('\u{2CF3}')) => continue,
            (Some('\u{2CF3}'), Some('\u{2CF2}')) => continue,
            (Some('\u{A640}'), Some('\u{A641}')) => continue,
            (Some('\u{A641}'), Some('\u{A640}')) => continue,
            (Some('\u{A642}'), Some('\u{A643}')) => continue,
            (Some('\u{A643}'), Some('\u{A642}')) => continue,
            (Some('\u{A644}'), Some('\u{A645}')) => continue,
            (Some('\u{A645}'), Some('\u{A644}')) => continue,
            (Some('\u{A646}'), Some('\u{A647}')) => continue,
            (Some('\u{A647}'), Some('\u{A646}')) => continue,
            (Some('\u{A648}'), Some('\u{A649}')) => continue,
            (Some('\u{A649}'), Some('\u{A648}')) => continue,
            (Some('\u{A64A}'), Some('\u{A64B}')) => continue,
            (Some('\u{A64B}'), Some('\u{A64A}')) => continue,
            (Some('\u{A64C}'), Some('\u{A64D}')) => continue,
            (Some('\u{A64D}'), Some('\u{A64C}')) => continue,
            (Some('\u{A64E}'), Some('\u{A64F}')) => continue,
            (Some('\u{A64F}'), Some('\u{A64E}')) => continue,
            (Some('\u{A650}'), Some('\u{A651}')) => continue,
            (Some('\u{A651}'), Some('\u{A650}')) => continue,
            (Some('\u{A652}'), Some('\u{A653}')) => continue,
            (Some('\u{A653}'), Some('\u{A652}')) => continue,
            (Some('\u{A654}'), Some('\u{A655}')) => continue,
            (Some('\u{A655}'), Some('\u{A654}')) => continue,
            (Some('\u{A656}'), Some('\u{A657}')) => continue,
            (Some('\u{A657}'), Some('\u{A656}')) => continue,
            (Some('\u{A658}'), Some('\u{A659}')) => continue,
            (Some('\u{A659}'), Some('\u{A658}')) => continue,
            (Some('\u{A65A}'), Some('\u{A65B}')) => continue,
            (Some('\u{A65B}'), Some('\u{A65A}')) => continue,
            (Some('\u{A65C}'), Some('\u{A65D}')) => continue,
            (Some('\u{A65D}'), Some('\u{A65C}')) => continue,
            (Some('\u{A65E}'), Some('\u{A65F}')) => continue,
            (Some('\u{A65F}'), Some('\u{A65E}')) => continue,
            (Some('\u{A660}'), Some('\u{A661}')) => continue,
            (Some('\u{A661}'), Some('\u{A660}')) => continue,
            (Some('\u{A662}'), Some('\u{A663}')) => continue,
            (Some('\u{A663}'), Some('\u{A662}')) => continue,
            (Some('\u{A664}'), Some('\u{A665}')) => continue,
            (Some('\u{A665}'), Some('\u{A664}')) => continue,
            (Some('\u{A666}'), Some('\u{A667}')) => continue,
            (Some('\u{A667}'), Some('\u{A666}')) => continue,
            (Some('\u{A668}'), Some('\u{A669}')) => continue,
            (Some('\u{A669}'), Some('\u{A668}')) => continue,
            (Some('\u{A66A}'), Some('\u{A66B}')) => continue,
            (Some('\u{A66B}'), Some('\u{A66A}')) => continue,
            (Some('\u{A66C}'), Some('\u{A66D}')) => continue,
            (Some('\u{A66D}'), Some('\u{A66C}')) => continue,
            (Some('\u{A680}'), Some('\u{A681}')) => continue,
            (Some('\u{A681}'), Some('\u{A680}')) => continue,
            (Some('\u{A682}'), Some('\u{A683}')) => continue,
            (Some('\u{A683}'), Some('\u{A682}')) => continue,
            (Some('\u{A684}'), Some('\u{A685}')) => continue,
            (Some('\u{A685}'), Some('\u{A684}')) => continue,
            (Some('\u{A686}'), Some('\u{A687}')) => continue,
            (Some('\u{A687}'), Some('\u{A686}')) => continue,
            (Some('\u{A688}'), Some('\u{A689}')) => continue,
            (Some('\u{A689}'), Some('\u{A688}')) => continue,
            (Some('\u{A68A}'), Some('\u{A68B}')) => continue,
            (Some('\u{A68B}'), Some('\u{A68A}')) => continue,
            (Some('\u{A68C}'), Some('\u{A68D}')) => continue,
            (Some('\u{A68D}'), Some('\u{A68C}')) => continue,
            (Some('\u{A68E}'), Some('\u{A68F}')) => continue,
            (Some('\u{A68F}'), Some('\u{A68E}')) => continue,
            (Some('\u{A690}'), Some('\u{A691}')) => continue,
            (Some('\u{A691}'), Some('\u{A690}')) => continue,
            (Some('\u{A692}'), Some('\u{A693}')) => continue,
            (Some('\u{A693}'), Some('\u{A692}')) => continue,
            (Some('\u{A694}'), Some('\u{A695}')) => continue,
            (Some('\u{A695}'), Some('\u{A694}')) => continue,
            (Some('\u{A696}'), Some('\u{A697}')) => continue,
            (Some('\u{A697}'), Some('\u{A696}')) => continue,
            (Some('\u{A698}'), Some('\u{A699}')) => continue,
            (Some('\u{A699}'), Some('\u{A698}')) => continue,
            (Some('\u{A69A}'), Some('\u{A69B}')) => continue,
            (Some('\u{A69B}'), Some('\u{A69A}')) => continue,
            (Some('\u{A722}'), Some('\u{A723}')) => continue,
            (Some('\u{A723}'), Some('\u{A722}')) => continue,
            (Some('\u{A724}'), Some('\u{A725}')) => continue,
            (Some('\u{A725}'), Some('\u{A724}')) => continue,
            (Some('\u{A726}'), Some('\u{A727}')) => continue,
            (Some('\u{A727}'), Some('\u{A726}')) => continue,
            (Some('\u{A728}'), Some('\u{A729}')) => continue,
            (Some('\u{A729}'), Some('\u{A728}')) => continue,
            (Some('\u{A72A}'), Some('\u{A72B}')) => continue,
            (Some('\u{A72B}'), Some('\u{A72A}')) => continue,
            (Some('\u{A72C}'), Some('\u{A72D}')) => continue,
            (Some('\u{A72D}'), Some('\u{A72C}')) => continue,
            (Some('\u{A72E}'), Some('\u{A72F}')) => continue,
            (Some('\u{A72F}'), Some('\u{A72E}')) => continue,
            (Some('\u{A732}'), Some('\u{A733}')) => continue,
            (Some('\u{A733}'), Some('\u{A732}')) => continue,
            (Some('\u{A734}'), Some('\u{A735}')) => continue,
            (Some('\u{A735}'), Some('\u{A734}')) => continue,
            (Some('\u{A736}'), Some('\u{A737}')) => continue,
            (Some('\u{A737}'), Some('\u{A736}')) => continue,
            (Some('\u{A738}'), Some('\u{A739}')) => continue,
            (Some('\u{A739}'), Some('\u{A738}')) => continue,
            (Some('\u{A73A}'), Some('\u{A73B}')) => continue,
            (Some('\u{A73B}'), Some('\u{A73A}')) => continue,
            (Some('\u{A73C}'), Some('\u{A73D}')) => continue,
            (Some('\u{A73D}'), Some('\u{A73C}')) => continue,
            (Some('\u{A73E}'), Some('\u{A73F}')) => continue,
            (Some('\u{A73F}'), Some('\u{A73E}')) => continue,
            (Some('\u{A740}'), Some('\u{A741}')) => continue,
            (Some('\u{A741}'), Some('\u{A740}')) => continue,
            (Some('\u{A742}'), Some('\u{A743}')) => continue,
            (Some('\u{A743}'), Some('\u{A742}')) => continue,
            (Some('\u{A744}'), Some('\u{A745}')) => continue,
            (Some('\u{A745}'), Some('\u{A744}')) => continue,
            (Some('\u{A746}'), Some('\u{A747}')) => continue,
            (Some('\u{A747}'), Some('\u{A746}')) => continue,
            (Some('\u{A748}'), Some('\u{A749}')) => continue,
            (Some('\u{A749}'), Some('\u{A748}')) => continue,
            (Some('\u{A74A}'), Some('\u{A74B}')) => continue,
            (Some('\u{A74B}'), Some('\u{A74A}')) => continue,
            (Some('\u{A74C}'), Some('\u{A74D}')) => continue,
            (Some('\u{A74D}'), Some('\u{A74C}')) => continue,
            (Some('\u{A74E}'), Some('\u{A74F}')) => continue,
            (Some('\u{A74F}'), Some('\u{A74E}')) => continue,
            (Some('\u{A750}'), Some('\u{A751}')) => continue,
            (Some('\u{A751}'), Some('\u{A750}')) => continue,
            (Some('\u{A752}'), Some('\u{A753}')) => continue,
            (Some('\u{A753}'), Some('\u{A752}')) => continue,
            (Some('\u{A754}'), Some('\u{A755}')) => continue,
            (Some('\u{A755}'), Some('\u{A754}')) => continue,
            (Some('\u{A756}'), Some('\u{A757}')) => continue,
            (Some('\u{A757}'), Some('\u{A756}')) => continue,
            (Some('\u{A758}'), Some('\u{A759}')) => continue,
            (Some('\u{A759}'), Some('\u{A758}')) => continue,
            (Some('\u{A75A}'), Some('\u{A75B}')) => continue,
            (Some('\u{A75B}'), Some('\u{A75A}')) => continue,
            (Some('\u{A75C}'), Some('\u{A75D}')) => continue,
            (Some('\u{A75D}'), Some('\u{A75C}')) => continue,
            (Some('\u{A75E}'), Some('\u{A75F}')) => continue,
            (Some('\u{A75F}'), Some('\u{A75E}')) => continue,
            (Some('\u{A760}'), Some('\u{A761}')) => continue,
            (Some('\u{A761}'), Some('\u{A760}')) => continue,
            (Some('\u{A762}'), Some('\u{A763}')) => continue,
            (Some('\u{A763}'), Some('\u{A762}')) => continue,
            (Some('\u{A764}'), Some('\u{A765}')) => continue,
            (Some('\u{A765}'), Some('\u{A764}')) => continue,
            (Some('\u{A766}'), Some('\u{A767}')) => continue,
            (Some('\u{A767}'), Some('\u{A766}')) => continue,
            (Some('\u{A768}'), Some('\u{A769}')) => continue,
            (Some('\u{A769}'), Some('\u{A768}')) => continue,
            (Some('\u{A76A}'), Some('\u{A76B}')) => continue,
            (Some('\u{A76B}'), Some('\u{A76A}')) => continue,
            (Some('\u{A76C}'), Some('\u{A76D}')) => continue,
            (Some('\u{A76D}'), Some('\u{A76C}')) => continue,
            (Some('\u{A76E}'), Some('\u{A76F}')) => continue,
            (Some('\u{A76F}'), Some('\u{A76E}')) => continue,
            (Some('\u{A779}'), Some('\u{A77A}')) => continue,
            (Some('\u{A77A}'), Some('\u{A779}')) => continue,
            (Some('\u{A77B}'), Some('\u{A77C}')) => continue,
            (Some('\u{A77C}'), Some('\u{A77B}')) => continue,
            (Some('\u{A77D}'), Some('\u{1D79}')) => continue,
            (Some('\u{1D79}'), Some('\u{A77D}')) => continue,
            (Some('\u{A77E}'), Some('\u{A77F}')) => continue,
            (Some('\u{A77F}'), Some('\u{A77E}')) => continue,
            (Some('\u{A780}'), Some('\u{A781}')) => continue,
            (Some('\u{A781}'), Some('\u{A780}')) => continue,
            (Some('\u{A782}'), Some('\u{A783}')) => continue,
            (Some('\u{A783}'), Some('\u{A782}')) => continue,
            (Some('\u{A784}'), Some('\u{A785}')) => continue,
            (Some('\u{A785}'), Some('\u{A784}')) => continue,
            (Some('\u{A786}'), Some('\u{A787}')) => continue,
            (Some('\u{A787}'), Some('\u{A786}')) => continue,
            (Some('\u{A78B}'), Some('\u{A78C}')) => continue,
            (Some('\u{A78C}'), Some('\u{A78B}')) => continue,
            (Some('\u{A78D}'), Some('\u{0265}')) => continue,
            (Some('\u{0265}'), Some('\u{A78D}')) => continue,
            (Some('\u{A790}'), Some('\u{A791}')) => continue,
            (Some('\u{A791}'), Some('\u{A790}')) => continue,
            (Some('\u{A792}'), Some('\u{A793}')) => continue,
            (Some('\u{A793}'), Some('\u{A792}')) => continue,
            (Some('\u{A796}'), Some('\u{A797}')) => continue,
            (Some('\u{A797}'), Some('\u{A796}')) => continue,
            (Some('\u{A798}'), Some('\u{A799}')) => continue,
            (Some('\u{A799}'), Some('\u{A798}')) => continue,
            (Some('\u{A79A}'), Some('\u{A79B}')) => continue,
            (Some('\u{A79B}'), Some('\u{A79A}')) => continue,
            (Some('\u{A79C}'), Some('\u{A79D}')) => continue,
            (Some('\u{A79D}'), Some('\u{A79C}')) => continue,
            (Some('\u{A79E}'), Some('\u{A79F}')) => continue,
            (Some('\u{A79F}'), Some('\u{A79E}')) => continue,
            (Some('\u{A7A0}'), Some('\u{A7A1}')) => continue,
            (Some('\u{A7A1}'), Some('\u{A7A0}')) => continue,
            (Some('\u{A7A2}'), Some('\u{A7A3}')) => continue,
            (Some('\u{A7A3}'), Some('\u{A7A2}')) => continue,
            (Some('\u{A7A4}'), Some('\u{A7A5}')) => continue,
            (Some('\u{A7A5}'), Some('\u{A7A4}')) => continue,
            (Some('\u{A7A6}'), Some('\u{A7A7}')) => continue,
            (Some('\u{A7A7}'), Some('\u{A7A6}')) => continue,
            (Some('\u{A7A8}'), Some('\u{A7A9}')) => continue,
            (Some('\u{A7A9}'), Some('\u{A7A8}')) => continue,
            (Some('\u{A7AA}'), Some('\u{0266}')) => continue,
            (Some('\u{0266}'), Some('\u{A7AA}')) => continue,
            (Some('\u{A7AB}'), Some('\u{025C}')) => continue,
            (Some('\u{025C}'), Some('\u{A7AB}')) => continue,
            (Some('\u{A7AC}'), Some('\u{0261}')) => continue,
            (Some('\u{0261}'), Some('\u{A7AC}')) => continue,
            (Some('\u{A7AD}'), Some('\u{026C}')) => continue,
            (Some('\u{026C}'), Some('\u{A7AD}')) => continue,
            (Some('\u{A7AE}'), Some('\u{026A}')) => continue,
            (Some('\u{026A}'), Some('\u{A7AE}')) => continue,
            (Some('\u{A7B0}'), Some('\u{029E}')) => continue,
            (Some('\u{029E}'), Some('\u{A7B0}')) => continue,
            (Some('\u{A7B1}'), Some('\u{0287}')) => continue,
            (Some('\u{0287}'), Some('\u{A7B1}')) => continue,
            (Some('\u{A7B2}'), Some('\u{029D}')) => continue,
            (Some('\u{029D}'), Some('\u{A7B2}')) => continue,
            (Some('\u{A7B3}'), Some('\u{AB53}')) => continue,
            (Some('\u{AB53}'), Some('\u{A7B3}')) => continue,
            (Some('\u{A7B4}'), Some('\u{A7B5}')) => continue,
            (Some('\u{A7B5}'), Some('\u{A7B4}')) => continue,
            (Some('\u{A7B6}'), Some('\u{A7B7}')) => continue,
            (Some('\u{A7B7}'), Some('\u{A7B6}')) => continue,
            (Some('\u{A7B8}'), Some('\u{A7B9}')) => continue,
            (Some('\u{A7B9}'), Some('\u{A7B8}')) => continue,
            (Some('\u{A7BA}'), Some('\u{A7BB}')) => continue,
            (Some('\u{A7BB}'), Some('\u{A7BA}')) => continue,
            (Some('\u{A7BC}'), Some('\u{A7BD}')) => continue,
            (Some('\u{A7BD}'), Some('\u{A7BC}')) => continue,
            (Some('\u{A7BE}'), Some('\u{A7BF}')) => continue,
            (Some('\u{A7BF}'), Some('\u{A7BE}')) => continue,
            (Some('\u{A7C2}'), Some('\u{A7C3}')) => continue,
            (Some('\u{A7C3}'), Some('\u{A7C2}')) => continue,
            (Some('\u{A7C4}'), Some('\u{A794}')) => continue,
            (Some('\u{A794}'), Some('\u{A7C4}')) => continue,
            (Some('\u{A7C5}'), Some('\u{0282}')) => continue,
            (Some('\u{0282}'), Some('\u{A7C5}')) => continue,
            (Some('\u{A7C6}'), Some('\u{1D8E}')) => continue,
            (Some('\u{1D8E}'), Some('\u{A7C6}')) => continue,
            (Some('\u{A7C7}'), Some('\u{A7C8}')) => continue,
            (Some('\u{A7C8}'), Some('\u{A7C7}')) => continue,
            (Some('\u{A7C9}'), Some('\u{A7CA}')) => continue,
            (Some('\u{A7CA}'), Some('\u{A7C9}')) => continue,
            (Some('\u{A7F5}'), Some('\u{A7F6}')) => continue,
            (Some('\u{A7F6}'), Some('\u{A7F5}')) => continue,
            (Some('\u{AB70}'), Some('\u{13A0}')) => continue,
            (Some('\u{13A0}'), Some('\u{AB70}')) => continue,
            (Some('\u{AB71}'), Some('\u{13A1}')) => continue,
            (Some('\u{13A1}'), Some('\u{AB71}')) => continue,
            (Some('\u{AB72}'), Some('\u{13A2}')) => continue,
            (Some('\u{13A2}'), Some('\u{AB72}')) => continue,
            (Some('\u{AB73}'), Some('\u{13A3}')) => continue,
            (Some('\u{13A3}'), Some('\u{AB73}')) => continue,
            (Some('\u{AB74}'), Some('\u{13A4}')) => continue,
            (Some('\u{13A4}'), Some('\u{AB74}')) => continue,
            (Some('\u{AB75}'), Some('\u{13A5}')) => continue,
            (Some('\u{13A5}'), Some('\u{AB75}')) => continue,
            (Some('\u{AB76}'), Some('\u{13A6}')) => continue,
            (Some('\u{13A6}'), Some('\u{AB76}')) => continue,
            (Some('\u{AB77}'), Some('\u{13A7}')) => continue,
            (Some('\u{13A7}'), Some('\u{AB77}')) => continue,
            (Some('\u{AB78}'), Some('\u{13A8}')) => continue,
            (Some('\u{13A8}'), Some('\u{AB78}')) => continue,
            (Some('\u{AB79}'), Some('\u{13A9}')) => continue,
            (Some('\u{13A9}'), Some('\u{AB79}')) => continue,
            (Some('\u{AB7A}'), Some('\u{13AA}')) => continue,
            (Some('\u{13AA}'), Some('\u{AB7A}')) => continue,
            (Some('\u{AB7B}'), Some('\u{13AB}')) => continue,
            (Some('\u{13AB}'), Some('\u{AB7B}')) => continue,
            (Some('\u{AB7C}'), Some('\u{13AC}')) => continue,
            (Some('\u{13AC}'), Some('\u{AB7C}')) => continue,
            (Some('\u{AB7D}'), Some('\u{13AD}')) => continue,
            (Some('\u{13AD}'), Some('\u{AB7D}')) => continue,
            (Some('\u{AB7E}'), Some('\u{13AE}')) => continue,
            (Some('\u{13AE}'), Some('\u{AB7E}')) => continue,
            (Some('\u{AB7F}'), Some('\u{13AF}')) => continue,
            (Some('\u{13AF}'), Some('\u{AB7F}')) => continue,
            (Some('\u{AB80}'), Some('\u{13B0}')) => continue,
            (Some('\u{13B0}'), Some('\u{AB80}')) => continue,
            (Some('\u{AB81}'), Some('\u{13B1}')) => continue,
            (Some('\u{13B1}'), Some('\u{AB81}')) => continue,
            (Some('\u{AB82}'), Some('\u{13B2}')) => continue,
            (Some('\u{13B2}'), Some('\u{AB82}')) => continue,
            (Some('\u{AB83}'), Some('\u{13B3}')) => continue,
            (Some('\u{13B3}'), Some('\u{AB83}')) => continue,
            (Some('\u{AB84}'), Some('\u{13B4}')) => continue,
            (Some('\u{13B4}'), Some('\u{AB84}')) => continue,
            (Some('\u{AB85}'), Some('\u{13B5}')) => continue,
            (Some('\u{13B5}'), Some('\u{AB85}')) => continue,
            (Some('\u{AB86}'), Some('\u{13B6}')) => continue,
            (Some('\u{13B6}'), Some('\u{AB86}')) => continue,
            (Some('\u{AB87}'), Some('\u{13B7}')) => continue,
            (Some('\u{13B7}'), Some('\u{AB87}')) => continue,
            (Some('\u{AB88}'), Some('\u{13B8}')) => continue,
            (Some('\u{13B8}'), Some('\u{AB88}')) => continue,
            (Some('\u{AB89}'), Some('\u{13B9}')) => continue,
            (Some('\u{13B9}'), Some('\u{AB89}')) => continue,
            (Some('\u{AB8A}'), Some('\u{13BA}')) => continue,
            (Some('\u{13BA}'), Some('\u{AB8A}')) => continue,
            (Some('\u{AB8B}'), Some('\u{13BB}')) => continue,
            (Some('\u{13BB}'), Some('\u{AB8B}')) => continue,
            (Some('\u{AB8C}'), Some('\u{13BC}')) => continue,
            (Some('\u{13BC}'), Some('\u{AB8C}')) => continue,
            (Some('\u{AB8D}'), Some('\u{13BD}')) => continue,
            (Some('\u{13BD}'), Some('\u{AB8D}')) => continue,
            (Some('\u{AB8E}'), Some('\u{13BE}')) => continue,
            (Some('\u{13BE}'), Some('\u{AB8E}')) => continue,
            (Some('\u{AB8F}'), Some('\u{13BF}')) => continue,
            (Some('\u{13BF}'), Some('\u{AB8F}')) => continue,
            (Some('\u{AB90}'), Some('\u{13C0}')) => continue,
            (Some('\u{13C0}'), Some('\u{AB90}')) => continue,
            (Some('\u{AB91}'), Some('\u{13C1}')) => continue,
            (Some('\u{13C1}'), Some('\u{AB91}')) => continue,
            (Some('\u{AB92}'), Some('\u{13C2}')) => continue,
            (Some('\u{13C2}'), Some('\u{AB92}')) => continue,
            (Some('\u{AB93}'), Some('\u{13C3}')) => continue,
            (Some('\u{13C3}'), Some('\u{AB93}')) => continue,
            (Some('\u{AB94}'), Some('\u{13C4}')) => continue,
            (Some('\u{13C4}'), Some('\u{AB94}')) => continue,
            (Some('\u{AB95}'), Some('\u{13C5}')) => continue,
            (Some('\u{13C5}'), Some('\u{AB95}')) => continue,
            (Some('\u{AB96}'), Some('\u{13C6}')) => continue,
            (Some('\u{13C6}'), Some('\u{AB96}')) => continue,
            (Some('\u{AB97}'), Some('\u{13C7}')) => continue,
            (Some('\u{13C7}'), Some('\u{AB97}')) => continue,
            (Some('\u{AB98}'), Some('\u{13C8}')) => continue,
            (Some('\u{13C8}'), Some('\u{AB98}')) => continue,
            (Some('\u{AB99}'), Some('\u{13C9}')) => continue,
            (Some('\u{13C9}'), Some('\u{AB99}')) => continue,
            (Some('\u{AB9A}'), Some('\u{13CA}')) => continue,
            (Some('\u{13CA}'), Some('\u{AB9A}')) => continue,
            (Some('\u{AB9B}'), Some('\u{13CB}')) => continue,
            (Some('\u{13CB}'), Some('\u{AB9B}')) => continue,
            (Some('\u{AB9C}'), Some('\u{13CC}')) => continue,
            (Some('\u{13CC}'), Some('\u{AB9C}')) => continue,
            (Some('\u{AB9D}'), Some('\u{13CD}')) => continue,
            (Some('\u{13CD}'), Some('\u{AB9D}')) => continue,
            (Some('\u{AB9E}'), Some('\u{13CE}')) => continue,
            (Some('\u{13CE}'), Some('\u{AB9E}')) => continue,
            (Some('\u{AB9F}'), Some('\u{13CF}')) => continue,
            (Some('\u{13CF}'), Some('\u{AB9F}')) => continue,
            (Some('\u{ABA0}'), Some('\u{13D0}')) => continue,
            (Some('\u{13D0}'), Some('\u{ABA0}')) => continue,
            (Some('\u{ABA1}'), Some('\u{13D1}')) => continue,
            (Some('\u{13D1}'), Some('\u{ABA1}')) => continue,
            (Some('\u{ABA2}'), Some('\u{13D2}')) => continue,
            (Some('\u{13D2}'), Some('\u{ABA2}')) => continue,
            (Some('\u{ABA3}'), Some('\u{13D3}')) => continue,
            (Some('\u{13D3}'), Some('\u{ABA3}')) => continue,
            (Some('\u{ABA4}'), Some('\u{13D4}')) => continue,
            (Some('\u{13D4}'), Some('\u{ABA4}')) => continue,
            (Some('\u{ABA5}'), Some('\u{13D5}')) => continue,
            (Some('\u{13D5}'), Some('\u{ABA5}')) => continue,
            (Some('\u{ABA6}'), Some('\u{13D6}')) => continue,
            (Some('\u{13D6}'), Some('\u{ABA6}')) => continue,
            (Some('\u{ABA7}'), Some('\u{13D7}')) => continue,
            (Some('\u{13D7}'), Some('\u{ABA7}')) => continue,
            (Some('\u{ABA8}'), Some('\u{13D8}')) => continue,
            (Some('\u{13D8}'), Some('\u{ABA8}')) => continue,
            (Some('\u{ABA9}'), Some('\u{13D9}')) => continue,
            (Some('\u{13D9}'), Some('\u{ABA9}')) => continue,
            (Some('\u{ABAA}'), Some('\u{13DA}')) => continue,
            (Some('\u{13DA}'), Some('\u{ABAA}')) => continue,
            (Some('\u{ABAB}'), Some('\u{13DB}')) => continue,
            (Some('\u{13DB}'), Some('\u{ABAB}')) => continue,
            (Some('\u{ABAC}'), Some('\u{13DC}')) => continue,
            (Some('\u{13DC}'), Some('\u{ABAC}')) => continue,
            (Some('\u{ABAD}'), Some('\u{13DD}')) => continue,
            (Some('\u{13DD}'), Some('\u{ABAD}')) => continue,
            (Some('\u{ABAE}'), Some('\u{13DE}')) => continue,
            (Some('\u{13DE}'), Some('\u{ABAE}')) => continue,
            (Some('\u{ABAF}'), Some('\u{13DF}')) => continue,
            (Some('\u{13DF}'), Some('\u{ABAF}')) => continue,
            (Some('\u{ABB0}'), Some('\u{13E0}')) => continue,
            (Some('\u{13E0}'), Some('\u{ABB0}')) => continue,
            (Some('\u{ABB1}'), Some('\u{13E1}')) => continue,
            (Some('\u{13E1}'), Some('\u{ABB1}')) => continue,
            (Some('\u{ABB2}'), Some('\u{13E2}')) => continue,
            (Some('\u{13E2}'), Some('\u{ABB2}')) => continue,
            (Some('\u{ABB3}'), Some('\u{13E3}')) => continue,
            (Some('\u{13E3}'), Some('\u{ABB3}')) => continue,
            (Some('\u{ABB4}'), Some('\u{13E4}')) => continue,
            (Some('\u{13E4}'), Some('\u{ABB4}')) => continue,
            (Some('\u{ABB5}'), Some('\u{13E5}')) => continue,
            (Some('\u{13E5}'), Some('\u{ABB5}')) => continue,
            (Some('\u{ABB6}'), Some('\u{13E6}')) => continue,
            (Some('\u{13E6}'), Some('\u{ABB6}')) => continue,
            (Some('\u{ABB7}'), Some('\u{13E7}')) => continue,
            (Some('\u{13E7}'), Some('\u{ABB7}')) => continue,
            (Some('\u{ABB8}'), Some('\u{13E8}')) => continue,
            (Some('\u{13E8}'), Some('\u{ABB8}')) => continue,
            (Some('\u{ABB9}'), Some('\u{13E9}')) => continue,
            (Some('\u{13E9}'), Some('\u{ABB9}')) => continue,
            (Some('\u{ABBA}'), Some('\u{13EA}')) => continue,
            (Some('\u{13EA}'), Some('\u{ABBA}')) => continue,
            (Some('\u{ABBB}'), Some('\u{13EB}')) => continue,
            (Some('\u{13EB}'), Some('\u{ABBB}')) => continue,
            (Some('\u{ABBC}'), Some('\u{13EC}')) => continue,
            (Some('\u{13EC}'), Some('\u{ABBC}')) => continue,
            (Some('\u{ABBD}'), Some('\u{13ED}')) => continue,
            (Some('\u{13ED}'), Some('\u{ABBD}')) => continue,
            (Some('\u{ABBE}'), Some('\u{13EE}')) => continue,
            (Some('\u{13EE}'), Some('\u{ABBE}')) => continue,
            (Some('\u{ABBF}'), Some('\u{13EF}')) => continue,
            (Some('\u{13EF}'), Some('\u{ABBF}')) => continue,
            (Some('\u{FB00}'), Some('\u{0066}')) if matches!(right.next(), Some('\u{0066}')) => {
                continue
            }
            (Some('\u{0066}'), Some('\u{FB00}')) if matches!(left.next(), Some('\u{0066}')) => {
                continue
            }
            (Some('\u{FB01}'), Some('\u{0066}')) if matches!(right.next(), Some('\u{0069}')) => {
                continue
            }
            (Some('\u{0066}'), Some('\u{FB01}')) if matches!(left.next(), Some('\u{0069}')) => {
                continue
            }
            (Some('\u{FB02}'), Some('\u{0066}')) if matches!(right.next(), Some('\u{006C}')) => {
                continue
            }
            (Some('\u{0066}'), Some('\u{FB02}')) if matches!(left.next(), Some('\u{006C}')) => {
                continue
            }
            (Some('\u{FB03}'), Some('\u{0066}'))
                if matches!(right.next(), Some('\u{0066}'))
                    && matches!(right.next(), Some('\u{0069}')) =>
            {
                continue
            }
            (Some('\u{0066}'), Some('\u{FB03}'))
                if matches!(left.next(), Some('\u{0066}'))
                    && matches!(left.next(), Some('\u{0069}')) =>
            {
                continue
            }
            (Some('\u{FB04}'), Some('\u{0066}'))
                if matches!(right.next(), Some('\u{0066}'))
                    && matches!(right.next(), Some('\u{006C}')) =>
            {
                continue
            }
            (Some('\u{0066}'), Some('\u{FB04}'))
                if matches!(left.next(), Some('\u{0066}'))
                    && matches!(left.next(), Some('\u{006C}')) =>
            {
                continue
            }
            (Some('\u{FB05}'), Some('\u{0073}')) if matches!(right.next(), Some('\u{0074}')) => {
                continue
            }
            (Some('\u{0073}'), Some('\u{FB05}')) if matches!(left.next(), Some('\u{0074}')) => {
                continue
            }
            (Some('\u{FB06}'), Some('\u{0073}')) if matches!(right.next(), Some('\u{0074}')) => {
                continue
            }
            (Some('\u{0073}'), Some('\u{FB06}')) if matches!(left.next(), Some('\u{0074}')) => {
                continue
            }
            (Some('\u{FB13}'), Some('\u{0574}')) if matches!(right.next(), Some('\u{0576}')) => {
                continue
            }
            (Some('\u{0574}'), Some('\u{FB13}')) if matches!(left.next(), Some('\u{0576}')) => {
                continue
            }
            (Some('\u{FB14}'), Some('\u{0574}')) if matches!(right.next(), Some('\u{0565}')) => {
                continue
            }
            (Some('\u{0574}'), Some('\u{FB14}')) if matches!(left.next(), Some('\u{0565}')) => {
                continue
            }
            (Some('\u{FB15}'), Some('\u{0574}')) if matches!(right.next(), Some('\u{056B}')) => {
                continue
            }
            (Some('\u{0574}'), Some('\u{FB15}')) if matches!(left.next(), Some('\u{056B}')) => {
                continue
            }
            (Some('\u{FB16}'), Some('\u{057E}')) if matches!(right.next(), Some('\u{0576}')) => {
                continue
            }
            (Some('\u{057E}'), Some('\u{FB16}')) if matches!(left.next(), Some('\u{0576}')) => {
                continue
            }
            (Some('\u{FB17}'), Some('\u{0574}')) if matches!(right.next(), Some('\u{056D}')) => {
                continue
            }
            (Some('\u{0574}'), Some('\u{FB17}')) if matches!(left.next(), Some('\u{056D}')) => {
                continue
            }
            (Some('\u{FF21}'), Some('\u{FF41}')) => continue,
            (Some('\u{FF41}'), Some('\u{FF21}')) => continue,
            (Some('\u{FF22}'), Some('\u{FF42}')) => continue,
            (Some('\u{FF42}'), Some('\u{FF22}')) => continue,
            (Some('\u{FF23}'), Some('\u{FF43}')) => continue,
            (Some('\u{FF43}'), Some('\u{FF23}')) => continue,
            (Some('\u{FF24}'), Some('\u{FF44}')) => continue,
            (Some('\u{FF44}'), Some('\u{FF24}')) => continue,
            (Some('\u{FF25}'), Some('\u{FF45}')) => continue,
            (Some('\u{FF45}'), Some('\u{FF25}')) => continue,
            (Some('\u{FF26}'), Some('\u{FF46}')) => continue,
            (Some('\u{FF46}'), Some('\u{FF26}')) => continue,
            (Some('\u{FF27}'), Some('\u{FF47}')) => continue,
            (Some('\u{FF47}'), Some('\u{FF27}')) => continue,
            (Some('\u{FF28}'), Some('\u{FF48}')) => continue,
            (Some('\u{FF48}'), Some('\u{FF28}')) => continue,
            (Some('\u{FF29}'), Some('\u{FF49}')) => continue,
            (Some('\u{FF49}'), Some('\u{FF29}')) => continue,
            (Some('\u{FF2A}'), Some('\u{FF4A}')) => continue,
            (Some('\u{FF4A}'), Some('\u{FF2A}')) => continue,
            (Some('\u{FF2B}'), Some('\u{FF4B}')) => continue,
            (Some('\u{FF4B}'), Some('\u{FF2B}')) => continue,
            (Some('\u{FF2C}'), Some('\u{FF4C}')) => continue,
            (Some('\u{FF4C}'), Some('\u{FF2C}')) => continue,
            (Some('\u{FF2D}'), Some('\u{FF4D}')) => continue,
            (Some('\u{FF4D}'), Some('\u{FF2D}')) => continue,
            (Some('\u{FF2E}'), Some('\u{FF4E}')) => continue,
            (Some('\u{FF4E}'), Some('\u{FF2E}')) => continue,
            (Some('\u{FF2F}'), Some('\u{FF4F}')) => continue,
            (Some('\u{FF4F}'), Some('\u{FF2F}')) => continue,
            (Some('\u{FF30}'), Some('\u{FF50}')) => continue,
            (Some('\u{FF50}'), Some('\u{FF30}')) => continue,
            (Some('\u{FF31}'), Some('\u{FF51}')) => continue,
            (Some('\u{FF51}'), Some('\u{FF31}')) => continue,
            (Some('\u{FF32}'), Some('\u{FF52}')) => continue,
            (Some('\u{FF52}'), Some('\u{FF32}')) => continue,
            (Some('\u{FF33}'), Some('\u{FF53}')) => continue,
            (Some('\u{FF53}'), Some('\u{FF33}')) => continue,
            (Some('\u{FF34}'), Some('\u{FF54}')) => continue,
            (Some('\u{FF54}'), Some('\u{FF34}')) => continue,
            (Some('\u{FF35}'), Some('\u{FF55}')) => continue,
            (Some('\u{FF55}'), Some('\u{FF35}')) => continue,
            (Some('\u{FF36}'), Some('\u{FF56}')) => continue,
            (Some('\u{FF56}'), Some('\u{FF36}')) => continue,
            (Some('\u{FF37}'), Some('\u{FF57}')) => continue,
            (Some('\u{FF57}'), Some('\u{FF37}')) => continue,
            (Some('\u{FF38}'), Some('\u{FF58}')) => continue,
            (Some('\u{FF58}'), Some('\u{FF38}')) => continue,
            (Some('\u{FF39}'), Some('\u{FF59}')) => continue,
            (Some('\u{FF59}'), Some('\u{FF39}')) => continue,
            (Some('\u{FF3A}'), Some('\u{FF5A}')) => continue,
            (Some('\u{FF5A}'), Some('\u{FF3A}')) => continue,
            (Some('\u{10400}'), Some('\u{10428}')) => continue,
            (Some('\u{10428}'), Some('\u{10400}')) => continue,
            (Some('\u{10401}'), Some('\u{10429}')) => continue,
            (Some('\u{10429}'), Some('\u{10401}')) => continue,
            (Some('\u{10402}'), Some('\u{1042A}')) => continue,
            (Some('\u{1042A}'), Some('\u{10402}')) => continue,
            (Some('\u{10403}'), Some('\u{1042B}')) => continue,
            (Some('\u{1042B}'), Some('\u{10403}')) => continue,
            (Some('\u{10404}'), Some('\u{1042C}')) => continue,
            (Some('\u{1042C}'), Some('\u{10404}')) => continue,
            (Some('\u{10405}'), Some('\u{1042D}')) => continue,
            (Some('\u{1042D}'), Some('\u{10405}')) => continue,
            (Some('\u{10406}'), Some('\u{1042E}')) => continue,
            (Some('\u{1042E}'), Some('\u{10406}')) => continue,
            (Some('\u{10407}'), Some('\u{1042F}')) => continue,
            (Some('\u{1042F}'), Some('\u{10407}')) => continue,
            (Some('\u{10408}'), Some('\u{10430}')) => continue,
            (Some('\u{10430}'), Some('\u{10408}')) => continue,
            (Some('\u{10409}'), Some('\u{10431}')) => continue,
            (Some('\u{10431}'), Some('\u{10409}')) => continue,
            (Some('\u{1040A}'), Some('\u{10432}')) => continue,
            (Some('\u{10432}'), Some('\u{1040A}')) => continue,
            (Some('\u{1040B}'), Some('\u{10433}')) => continue,
            (Some('\u{10433}'), Some('\u{1040B}')) => continue,
            (Some('\u{1040C}'), Some('\u{10434}')) => continue,
            (Some('\u{10434}'), Some('\u{1040C}')) => continue,
            (Some('\u{1040D}'), Some('\u{10435}')) => continue,
            (Some('\u{10435}'), Some('\u{1040D}')) => continue,
            (Some('\u{1040E}'), Some('\u{10436}')) => continue,
            (Some('\u{10436}'), Some('\u{1040E}')) => continue,
            (Some('\u{1040F}'), Some('\u{10437}')) => continue,
            (Some('\u{10437}'), Some('\u{1040F}')) => continue,
            (Some('\u{10410}'), Some('\u{10438}')) => continue,
            (Some('\u{10438}'), Some('\u{10410}')) => continue,
            (Some('\u{10411}'), Some('\u{10439}')) => continue,
            (Some('\u{10439}'), Some('\u{10411}')) => continue,
            (Some('\u{10412}'), Some('\u{1043A}')) => continue,
            (Some('\u{1043A}'), Some('\u{10412}')) => continue,
            (Some('\u{10413}'), Some('\u{1043B}')) => continue,
            (Some('\u{1043B}'), Some('\u{10413}')) => continue,
            (Some('\u{10414}'), Some('\u{1043C}')) => continue,
            (Some('\u{1043C}'), Some('\u{10414}')) => continue,
            (Some('\u{10415}'), Some('\u{1043D}')) => continue,
            (Some('\u{1043D}'), Some('\u{10415}')) => continue,
            (Some('\u{10416}'), Some('\u{1043E}')) => continue,
            (Some('\u{1043E}'), Some('\u{10416}')) => continue,
            (Some('\u{10417}'), Some('\u{1043F}')) => continue,
            (Some('\u{1043F}'), Some('\u{10417}')) => continue,
            (Some('\u{10418}'), Some('\u{10440}')) => continue,
            (Some('\u{10440}'), Some('\u{10418}')) => continue,
            (Some('\u{10419}'), Some('\u{10441}')) => continue,
            (Some('\u{10441}'), Some('\u{10419}')) => continue,
            (Some('\u{1041A}'), Some('\u{10442}')) => continue,
            (Some('\u{10442}'), Some('\u{1041A}')) => continue,
            (Some('\u{1041B}'), Some('\u{10443}')) => continue,
            (Some('\u{10443}'), Some('\u{1041B}')) => continue,
            (Some('\u{1041C}'), Some('\u{10444}')) => continue,
            (Some('\u{10444}'), Some('\u{1041C}')) => continue,
            (Some('\u{1041D}'), Some('\u{10445}')) => continue,
            (Some('\u{10445}'), Some('\u{1041D}')) => continue,
            (Some('\u{1041E}'), Some('\u{10446}')) => continue,
            (Some('\u{10446}'), Some('\u{1041E}')) => continue,
            (Some('\u{1041F}'), Some('\u{10447}')) => continue,
            (Some('\u{10447}'), Some('\u{1041F}')) => continue,
            (Some('\u{10420}'), Some('\u{10448}')) => continue,
            (Some('\u{10448}'), Some('\u{10420}')) => continue,
            (Some('\u{10421}'), Some('\u{10449}')) => continue,
            (Some('\u{10449}'), Some('\u{10421}')) => continue,
            (Some('\u{10422}'), Some('\u{1044A}')) => continue,
            (Some('\u{1044A}'), Some('\u{10422}')) => continue,
            (Some('\u{10423}'), Some('\u{1044B}')) => continue,
            (Some('\u{1044B}'), Some('\u{10423}')) => continue,
            (Some('\u{10424}'), Some('\u{1044C}')) => continue,
            (Some('\u{1044C}'), Some('\u{10424}')) => continue,
            (Some('\u{10425}'), Some('\u{1044D}')) => continue,
            (Some('\u{1044D}'), Some('\u{10425}')) => continue,
            (Some('\u{10426}'), Some('\u{1044E}')) => continue,
            (Some('\u{1044E}'), Some('\u{10426}')) => continue,
            (Some('\u{10427}'), Some('\u{1044F}')) => continue,
            (Some('\u{1044F}'), Some('\u{10427}')) => continue,
            (Some('\u{104B0}'), Some('\u{104D8}')) => continue,
            (Some('\u{104D8}'), Some('\u{104B0}')) => continue,
            (Some('\u{104B1}'), Some('\u{104D9}')) => continue,
            (Some('\u{104D9}'), Some('\u{104B1}')) => continue,
            (Some('\u{104B2}'), Some('\u{104DA}')) => continue,
            (Some('\u{104DA}'), Some('\u{104B2}')) => continue,
            (Some('\u{104B3}'), Some('\u{104DB}')) => continue,
            (Some('\u{104DB}'), Some('\u{104B3}')) => continue,
            (Some('\u{104B4}'), Some('\u{104DC}')) => continue,
            (Some('\u{104DC}'), Some('\u{104B4}')) => continue,
            (Some('\u{104B5}'), Some('\u{104DD}')) => continue,
            (Some('\u{104DD}'), Some('\u{104B5}')) => continue,
            (Some('\u{104B6}'), Some('\u{104DE}')) => continue,
            (Some('\u{104DE}'), Some('\u{104B6}')) => continue,
            (Some('\u{104B7}'), Some('\u{104DF}')) => continue,
            (Some('\u{104DF}'), Some('\u{104B7}')) => continue,
            (Some('\u{104B8}'), Some('\u{104E0}')) => continue,
            (Some('\u{104E0}'), Some('\u{104B8}')) => continue,
            (Some('\u{104B9}'), Some('\u{104E1}')) => continue,
            (Some('\u{104E1}'), Some('\u{104B9}')) => continue,
            (Some('\u{104BA}'), Some('\u{104E2}')) => continue,
            (Some('\u{104E2}'), Some('\u{104BA}')) => continue,
            (Some('\u{104BB}'), Some('\u{104E3}')) => continue,
            (Some('\u{104E3}'), Some('\u{104BB}')) => continue,
            (Some('\u{104BC}'), Some('\u{104E4}')) => continue,
            (Some('\u{104E4}'), Some('\u{104BC}')) => continue,
            (Some('\u{104BD}'), Some('\u{104E5}')) => continue,
            (Some('\u{104E5}'), Some('\u{104BD}')) => continue,
            (Some('\u{104BE}'), Some('\u{104E6}')) => continue,
            (Some('\u{104E6}'), Some('\u{104BE}')) => continue,
            (Some('\u{104BF}'), Some('\u{104E7}')) => continue,
            (Some('\u{104E7}'), Some('\u{104BF}')) => continue,
            (Some('\u{104C0}'), Some('\u{104E8}')) => continue,
            (Some('\u{104E8}'), Some('\u{104C0}')) => continue,
            (Some('\u{104C1}'), Some('\u{104E9}')) => continue,
            (Some('\u{104E9}'), Some('\u{104C1}')) => continue,
            (Some('\u{104C2}'), Some('\u{104EA}')) => continue,
            (Some('\u{104EA}'), Some('\u{104C2}')) => continue,
            (Some('\u{104C3}'), Some('\u{104EB}')) => continue,
            (Some('\u{104EB}'), Some('\u{104C3}')) => continue,
            (Some('\u{104C4}'), Some('\u{104EC}')) => continue,
            (Some('\u{104EC}'), Some('\u{104C4}')) => continue,
            (Some('\u{104C5}'), Some('\u{104ED}')) => continue,
            (Some('\u{104ED}'), Some('\u{104C5}')) => continue,
            (Some('\u{104C6}'), Some('\u{104EE}')) => continue,
            (Some('\u{104EE}'), Some('\u{104C6}')) => continue,
            (Some('\u{104C7}'), Some('\u{104EF}')) => continue,
            (Some('\u{104EF}'), Some('\u{104C7}')) => continue,
            (Some('\u{104C8}'), Some('\u{104F0}')) => continue,
            (Some('\u{104F0}'), Some('\u{104C8}')) => continue,
            (Some('\u{104C9}'), Some('\u{104F1}')) => continue,
            (Some('\u{104F1}'), Some('\u{104C9}')) => continue,
            (Some('\u{104CA}'), Some('\u{104F2}')) => continue,
            (Some('\u{104F2}'), Some('\u{104CA}')) => continue,
            (Some('\u{104CB}'), Some('\u{104F3}')) => continue,
            (Some('\u{104F3}'), Some('\u{104CB}')) => continue,
            (Some('\u{104CC}'), Some('\u{104F4}')) => continue,
            (Some('\u{104F4}'), Some('\u{104CC}')) => continue,
            (Some('\u{104CD}'), Some('\u{104F5}')) => continue,
            (Some('\u{104F5}'), Some('\u{104CD}')) => continue,
            (Some('\u{104CE}'), Some('\u{104F6}')) => continue,
            (Some('\u{104F6}'), Some('\u{104CE}')) => continue,
            (Some('\u{104CF}'), Some('\u{104F7}')) => continue,
            (Some('\u{104F7}'), Some('\u{104CF}')) => continue,
            (Some('\u{104D0}'), Some('\u{104F8}')) => continue,
            (Some('\u{104F8}'), Some('\u{104D0}')) => continue,
            (Some('\u{104D1}'), Some('\u{104F9}')) => continue,
            (Some('\u{104F9}'), Some('\u{104D1}')) => continue,
            (Some('\u{104D2}'), Some('\u{104FA}')) => continue,
            (Some('\u{104FA}'), Some('\u{104D2}')) => continue,
            (Some('\u{104D3}'), Some('\u{104FB}')) => continue,
            (Some('\u{104FB}'), Some('\u{104D3}')) => continue,
            (Some('\u{10C80}'), Some('\u{10CC0}')) => continue,
            (Some('\u{10CC0}'), Some('\u{10C80}')) => continue,
            (Some('\u{10C81}'), Some('\u{10CC1}')) => continue,
            (Some('\u{10CC1}'), Some('\u{10C81}')) => continue,
            (Some('\u{10C82}'), Some('\u{10CC2}')) => continue,
            (Some('\u{10CC2}'), Some('\u{10C82}')) => continue,
            (Some('\u{10C83}'), Some('\u{10CC3}')) => continue,
            (Some('\u{10CC3}'), Some('\u{10C83}')) => continue,
            (Some('\u{10C84}'), Some('\u{10CC4}')) => continue,
            (Some('\u{10CC4}'), Some('\u{10C84}')) => continue,
            (Some('\u{10C85}'), Some('\u{10CC5}')) => continue,
            (Some('\u{10CC5}'), Some('\u{10C85}')) => continue,
            (Some('\u{10C86}'), Some('\u{10CC6}')) => continue,
            (Some('\u{10CC6}'), Some('\u{10C86}')) => continue,
            (Some('\u{10C87}'), Some('\u{10CC7}')) => continue,
            (Some('\u{10CC7}'), Some('\u{10C87}')) => continue,
            (Some('\u{10C88}'), Some('\u{10CC8}')) => continue,
            (Some('\u{10CC8}'), Some('\u{10C88}')) => continue,
            (Some('\u{10C89}'), Some('\u{10CC9}')) => continue,
            (Some('\u{10CC9}'), Some('\u{10C89}')) => continue,
            (Some('\u{10C8A}'), Some('\u{10CCA}')) => continue,
            (Some('\u{10CCA}'), Some('\u{10C8A}')) => continue,
            (Some('\u{10C8B}'), Some('\u{10CCB}')) => continue,
            (Some('\u{10CCB}'), Some('\u{10C8B}')) => continue,
            (Some('\u{10C8C}'), Some('\u{10CCC}')) => continue,
            (Some('\u{10CCC}'), Some('\u{10C8C}')) => continue,
            (Some('\u{10C8D}'), Some('\u{10CCD}')) => continue,
            (Some('\u{10CCD}'), Some('\u{10C8D}')) => continue,
            (Some('\u{10C8E}'), Some('\u{10CCE}')) => continue,
            (Some('\u{10CCE}'), Some('\u{10C8E}')) => continue,
            (Some('\u{10C8F}'), Some('\u{10CCF}')) => continue,
            (Some('\u{10CCF}'), Some('\u{10C8F}')) => continue,
            (Some('\u{10C90}'), Some('\u{10CD0}')) => continue,
            (Some('\u{10CD0}'), Some('\u{10C90}')) => continue,
            (Some('\u{10C91}'), Some('\u{10CD1}')) => continue,
            (Some('\u{10CD1}'), Some('\u{10C91}')) => continue,
            (Some('\u{10C92}'), Some('\u{10CD2}')) => continue,
            (Some('\u{10CD2}'), Some('\u{10C92}')) => continue,
            (Some('\u{10C93}'), Some('\u{10CD3}')) => continue,
            (Some('\u{10CD3}'), Some('\u{10C93}')) => continue,
            (Some('\u{10C94}'), Some('\u{10CD4}')) => continue,
            (Some('\u{10CD4}'), Some('\u{10C94}')) => continue,
            (Some('\u{10C95}'), Some('\u{10CD5}')) => continue,
            (Some('\u{10CD5}'), Some('\u{10C95}')) => continue,
            (Some('\u{10C96}'), Some('\u{10CD6}')) => continue,
            (Some('\u{10CD6}'), Some('\u{10C96}')) => continue,
            (Some('\u{10C97}'), Some('\u{10CD7}')) => continue,
            (Some('\u{10CD7}'), Some('\u{10C97}')) => continue,
            (Some('\u{10C98}'), Some('\u{10CD8}')) => continue,
            (Some('\u{10CD8}'), Some('\u{10C98}')) => continue,
            (Some('\u{10C99}'), Some('\u{10CD9}')) => continue,
            (Some('\u{10CD9}'), Some('\u{10C99}')) => continue,
            (Some('\u{10C9A}'), Some('\u{10CDA}')) => continue,
            (Some('\u{10CDA}'), Some('\u{10C9A}')) => continue,
            (Some('\u{10C9B}'), Some('\u{10CDB}')) => continue,
            (Some('\u{10CDB}'), Some('\u{10C9B}')) => continue,
            (Some('\u{10C9C}'), Some('\u{10CDC}')) => continue,
            (Some('\u{10CDC}'), Some('\u{10C9C}')) => continue,
            (Some('\u{10C9D}'), Some('\u{10CDD}')) => continue,
            (Some('\u{10CDD}'), Some('\u{10C9D}')) => continue,
            (Some('\u{10C9E}'), Some('\u{10CDE}')) => continue,
            (Some('\u{10CDE}'), Some('\u{10C9E}')) => continue,
            (Some('\u{10C9F}'), Some('\u{10CDF}')) => continue,
            (Some('\u{10CDF}'), Some('\u{10C9F}')) => continue,
            (Some('\u{10CA0}'), Some('\u{10CE0}')) => continue,
            (Some('\u{10CE0}'), Some('\u{10CA0}')) => continue,
            (Some('\u{10CA1}'), Some('\u{10CE1}')) => continue,
            (Some('\u{10CE1}'), Some('\u{10CA1}')) => continue,
            (Some('\u{10CA2}'), Some('\u{10CE2}')) => continue,
            (Some('\u{10CE2}'), Some('\u{10CA2}')) => continue,
            (Some('\u{10CA3}'), Some('\u{10CE3}')) => continue,
            (Some('\u{10CE3}'), Some('\u{10CA3}')) => continue,
            (Some('\u{10CA4}'), Some('\u{10CE4}')) => continue,
            (Some('\u{10CE4}'), Some('\u{10CA4}')) => continue,
            (Some('\u{10CA5}'), Some('\u{10CE5}')) => continue,
            (Some('\u{10CE5}'), Some('\u{10CA5}')) => continue,
            (Some('\u{10CA6}'), Some('\u{10CE6}')) => continue,
            (Some('\u{10CE6}'), Some('\u{10CA6}')) => continue,
            (Some('\u{10CA7}'), Some('\u{10CE7}')) => continue,
            (Some('\u{10CE7}'), Some('\u{10CA7}')) => continue,
            (Some('\u{10CA8}'), Some('\u{10CE8}')) => continue,
            (Some('\u{10CE8}'), Some('\u{10CA8}')) => continue,
            (Some('\u{10CA9}'), Some('\u{10CE9}')) => continue,
            (Some('\u{10CE9}'), Some('\u{10CA9}')) => continue,
            (Some('\u{10CAA}'), Some('\u{10CEA}')) => continue,
            (Some('\u{10CEA}'), Some('\u{10CAA}')) => continue,
            (Some('\u{10CAB}'), Some('\u{10CEB}')) => continue,
            (Some('\u{10CEB}'), Some('\u{10CAB}')) => continue,
            (Some('\u{10CAC}'), Some('\u{10CEC}')) => continue,
            (Some('\u{10CEC}'), Some('\u{10CAC}')) => continue,
            (Some('\u{10CAD}'), Some('\u{10CED}')) => continue,
            (Some('\u{10CED}'), Some('\u{10CAD}')) => continue,
            (Some('\u{10CAE}'), Some('\u{10CEE}')) => continue,
            (Some('\u{10CEE}'), Some('\u{10CAE}')) => continue,
            (Some('\u{10CAF}'), Some('\u{10CEF}')) => continue,
            (Some('\u{10CEF}'), Some('\u{10CAF}')) => continue,
            (Some('\u{10CB0}'), Some('\u{10CF0}')) => continue,
            (Some('\u{10CF0}'), Some('\u{10CB0}')) => continue,
            (Some('\u{10CB1}'), Some('\u{10CF1}')) => continue,
            (Some('\u{10CF1}'), Some('\u{10CB1}')) => continue,
            (Some('\u{10CB2}'), Some('\u{10CF2}')) => continue,
            (Some('\u{10CF2}'), Some('\u{10CB2}')) => continue,
            (Some('\u{118A0}'), Some('\u{118C0}')) => continue,
            (Some('\u{118C0}'), Some('\u{118A0}')) => continue,
            (Some('\u{118A1}'), Some('\u{118C1}')) => continue,
            (Some('\u{118C1}'), Some('\u{118A1}')) => continue,
            (Some('\u{118A2}'), Some('\u{118C2}')) => continue,
            (Some('\u{118C2}'), Some('\u{118A2}')) => continue,
            (Some('\u{118A3}'), Some('\u{118C3}')) => continue,
            (Some('\u{118C3}'), Some('\u{118A3}')) => continue,
            (Some('\u{118A4}'), Some('\u{118C4}')) => continue,
            (Some('\u{118C4}'), Some('\u{118A4}')) => continue,
            (Some('\u{118A5}'), Some('\u{118C5}')) => continue,
            (Some('\u{118C5}'), Some('\u{118A5}')) => continue,
            (Some('\u{118A6}'), Some('\u{118C6}')) => continue,
            (Some('\u{118C6}'), Some('\u{118A6}')) => continue,
            (Some('\u{118A7}'), Some('\u{118C7}')) => continue,
            (Some('\u{118C7}'), Some('\u{118A7}')) => continue,
            (Some('\u{118A8}'), Some('\u{118C8}')) => continue,
            (Some('\u{118C8}'), Some('\u{118A8}')) => continue,
            (Some('\u{118A9}'), Some('\u{118C9}')) => continue,
            (Some('\u{118C9}'), Some('\u{118A9}')) => continue,
            (Some('\u{118AA}'), Some('\u{118CA}')) => continue,
            (Some('\u{118CA}'), Some('\u{118AA}')) => continue,
            (Some('\u{118AB}'), Some('\u{118CB}')) => continue,
            (Some('\u{118CB}'), Some('\u{118AB}')) => continue,
            (Some('\u{118AC}'), Some('\u{118CC}')) => continue,
            (Some('\u{118CC}'), Some('\u{118AC}')) => continue,
            (Some('\u{118AD}'), Some('\u{118CD}')) => continue,
            (Some('\u{118CD}'), Some('\u{118AD}')) => continue,
            (Some('\u{118AE}'), Some('\u{118CE}')) => continue,
            (Some('\u{118CE}'), Some('\u{118AE}')) => continue,
            (Some('\u{118AF}'), Some('\u{118CF}')) => continue,
            (Some('\u{118CF}'), Some('\u{118AF}')) => continue,
            (Some('\u{118B0}'), Some('\u{118D0}')) => continue,
            (Some('\u{118D0}'), Some('\u{118B0}')) => continue,
            (Some('\u{118B1}'), Some('\u{118D1}')) => continue,
            (Some('\u{118D1}'), Some('\u{118B1}')) => continue,
            (Some('\u{118B2}'), Some('\u{118D2}')) => continue,
            (Some('\u{118D2}'), Some('\u{118B2}')) => continue,
            (Some('\u{118B3}'), Some('\u{118D3}')) => continue,
            (Some('\u{118D3}'), Some('\u{118B3}')) => continue,
            (Some('\u{118B4}'), Some('\u{118D4}')) => continue,
            (Some('\u{118D4}'), Some('\u{118B4}')) => continue,
            (Some('\u{118B5}'), Some('\u{118D5}')) => continue,
            (Some('\u{118D5}'), Some('\u{118B5}')) => continue,
            (Some('\u{118B6}'), Some('\u{118D6}')) => continue,
            (Some('\u{118D6}'), Some('\u{118B6}')) => continue,
            (Some('\u{118B7}'), Some('\u{118D7}')) => continue,
            (Some('\u{118D7}'), Some('\u{118B7}')) => continue,
            (Some('\u{118B8}'), Some('\u{118D8}')) => continue,
            (Some('\u{118D8}'), Some('\u{118B8}')) => continue,
            (Some('\u{118B9}'), Some('\u{118D9}')) => continue,
            (Some('\u{118D9}'), Some('\u{118B9}')) => continue,
            (Some('\u{118BA}'), Some('\u{118DA}')) => continue,
            (Some('\u{118DA}'), Some('\u{118BA}')) => continue,
            (Some('\u{118BB}'), Some('\u{118DB}')) => continue,
            (Some('\u{118DB}'), Some('\u{118BB}')) => continue,
            (Some('\u{118BC}'), Some('\u{118DC}')) => continue,
            (Some('\u{118DC}'), Some('\u{118BC}')) => continue,
            (Some('\u{118BD}'), Some('\u{118DD}')) => continue,
            (Some('\u{118DD}'), Some('\u{118BD}')) => continue,
            (Some('\u{118BE}'), Some('\u{118DE}')) => continue,
            (Some('\u{118DE}'), Some('\u{118BE}')) => continue,
            (Some('\u{118BF}'), Some('\u{118DF}')) => continue,
            (Some('\u{118DF}'), Some('\u{118BF}')) => continue,
            (Some('\u{16E40}'), Some('\u{16E60}')) => continue,
            (Some('\u{16E60}'), Some('\u{16E40}')) => continue,
            (Some('\u{16E41}'), Some('\u{16E61}')) => continue,
            (Some('\u{16E61}'), Some('\u{16E41}')) => continue,
            (Some('\u{16E42}'), Some('\u{16E62}')) => continue,
            (Some('\u{16E62}'), Some('\u{16E42}')) => continue,
            (Some('\u{16E43}'), Some('\u{16E63}')) => continue,
            (Some('\u{16E63}'), Some('\u{16E43}')) => continue,
            (Some('\u{16E44}'), Some('\u{16E64}')) => continue,
            (Some('\u{16E64}'), Some('\u{16E44}')) => continue,
            (Some('\u{16E45}'), Some('\u{16E65}')) => continue,
            (Some('\u{16E65}'), Some('\u{16E45}')) => continue,
            (Some('\u{16E46}'), Some('\u{16E66}')) => continue,
            (Some('\u{16E66}'), Some('\u{16E46}')) => continue,
            (Some('\u{16E47}'), Some('\u{16E67}')) => continue,
            (Some('\u{16E67}'), Some('\u{16E47}')) => continue,
            (Some('\u{16E48}'), Some('\u{16E68}')) => continue,
            (Some('\u{16E68}'), Some('\u{16E48}')) => continue,
            (Some('\u{16E49}'), Some('\u{16E69}')) => continue,
            (Some('\u{16E69}'), Some('\u{16E49}')) => continue,
            (Some('\u{16E4A}'), Some('\u{16E6A}')) => continue,
            (Some('\u{16E6A}'), Some('\u{16E4A}')) => continue,
            (Some('\u{16E4B}'), Some('\u{16E6B}')) => continue,
            (Some('\u{16E6B}'), Some('\u{16E4B}')) => continue,
            (Some('\u{16E4C}'), Some('\u{16E6C}')) => continue,
            (Some('\u{16E6C}'), Some('\u{16E4C}')) => continue,
            (Some('\u{16E4D}'), Some('\u{16E6D}')) => continue,
            (Some('\u{16E6D}'), Some('\u{16E4D}')) => continue,
            (Some('\u{16E4E}'), Some('\u{16E6E}')) => continue,
            (Some('\u{16E6E}'), Some('\u{16E4E}')) => continue,
            (Some('\u{16E4F}'), Some('\u{16E6F}')) => continue,
            (Some('\u{16E6F}'), Some('\u{16E4F}')) => continue,
            (Some('\u{16E50}'), Some('\u{16E70}')) => continue,
            (Some('\u{16E70}'), Some('\u{16E50}')) => continue,
            (Some('\u{16E51}'), Some('\u{16E71}')) => continue,
            (Some('\u{16E71}'), Some('\u{16E51}')) => continue,
            (Some('\u{16E52}'), Some('\u{16E72}')) => continue,
            (Some('\u{16E72}'), Some('\u{16E52}')) => continue,
            (Some('\u{16E53}'), Some('\u{16E73}')) => continue,
            (Some('\u{16E73}'), Some('\u{16E53}')) => continue,
            (Some('\u{16E54}'), Some('\u{16E74}')) => continue,
            (Some('\u{16E74}'), Some('\u{16E54}')) => continue,
            (Some('\u{16E55}'), Some('\u{16E75}')) => continue,
            (Some('\u{16E75}'), Some('\u{16E55}')) => continue,
            (Some('\u{16E56}'), Some('\u{16E76}')) => continue,
            (Some('\u{16E76}'), Some('\u{16E56}')) => continue,
            (Some('\u{16E57}'), Some('\u{16E77}')) => continue,
            (Some('\u{16E77}'), Some('\u{16E57}')) => continue,
            (Some('\u{16E58}'), Some('\u{16E78}')) => continue,
            (Some('\u{16E78}'), Some('\u{16E58}')) => continue,
            (Some('\u{16E59}'), Some('\u{16E79}')) => continue,
            (Some('\u{16E79}'), Some('\u{16E59}')) => continue,
            (Some('\u{16E5A}'), Some('\u{16E7A}')) => continue,
            (Some('\u{16E7A}'), Some('\u{16E5A}')) => continue,
            (Some('\u{16E5B}'), Some('\u{16E7B}')) => continue,
            (Some('\u{16E7B}'), Some('\u{16E5B}')) => continue,
            (Some('\u{16E5C}'), Some('\u{16E7C}')) => continue,
            (Some('\u{16E7C}'), Some('\u{16E5C}')) => continue,
            (Some('\u{16E5D}'), Some('\u{16E7D}')) => continue,
            (Some('\u{16E7D}'), Some('\u{16E5D}')) => continue,
            (Some('\u{16E5E}'), Some('\u{16E7E}')) => continue,
            (Some('\u{16E7E}'), Some('\u{16E5E}')) => continue,
            (Some('\u{16E5F}'), Some('\u{16E7F}')) => continue,
            (Some('\u{16E7F}'), Some('\u{16E5F}')) => continue,
            (Some('\u{1E900}'), Some('\u{1E922}')) => continue,
            (Some('\u{1E922}'), Some('\u{1E900}')) => continue,
            (Some('\u{1E901}'), Some('\u{1E923}')) => continue,
            (Some('\u{1E923}'), Some('\u{1E901}')) => continue,
            (Some('\u{1E902}'), Some('\u{1E924}')) => continue,
            (Some('\u{1E924}'), Some('\u{1E902}')) => continue,
            (Some('\u{1E903}'), Some('\u{1E925}')) => continue,
            (Some('\u{1E925}'), Some('\u{1E903}')) => continue,
            (Some('\u{1E904}'), Some('\u{1E926}')) => continue,
            (Some('\u{1E926}'), Some('\u{1E904}')) => continue,
            (Some('\u{1E905}'), Some('\u{1E927}')) => continue,
            (Some('\u{1E927}'), Some('\u{1E905}')) => continue,
            (Some('\u{1E906}'), Some('\u{1E928}')) => continue,
            (Some('\u{1E928}'), Some('\u{1E906}')) => continue,
            (Some('\u{1E907}'), Some('\u{1E929}')) => continue,
            (Some('\u{1E929}'), Some('\u{1E907}')) => continue,
            (Some('\u{1E908}'), Some('\u{1E92A}')) => continue,
            (Some('\u{1E92A}'), Some('\u{1E908}')) => continue,
            (Some('\u{1E909}'), Some('\u{1E92B}')) => continue,
            (Some('\u{1E92B}'), Some('\u{1E909}')) => continue,
            (Some('\u{1E90A}'), Some('\u{1E92C}')) => continue,
            (Some('\u{1E92C}'), Some('\u{1E90A}')) => continue,
            (Some('\u{1E90B}'), Some('\u{1E92D}')) => continue,
            (Some('\u{1E92D}'), Some('\u{1E90B}')) => continue,
            (Some('\u{1E90C}'), Some('\u{1E92E}')) => continue,
            (Some('\u{1E92E}'), Some('\u{1E90C}')) => continue,
            (Some('\u{1E90D}'), Some('\u{1E92F}')) => continue,
            (Some('\u{1E92F}'), Some('\u{1E90D}')) => continue,
            (Some('\u{1E90E}'), Some('\u{1E930}')) => continue,
            (Some('\u{1E930}'), Some('\u{1E90E}')) => continue,
            (Some('\u{1E90F}'), Some('\u{1E931}')) => continue,
            (Some('\u{1E931}'), Some('\u{1E90F}')) => continue,
            (Some('\u{1E910}'), Some('\u{1E932}')) => continue,
            (Some('\u{1E932}'), Some('\u{1E910}')) => continue,
            (Some('\u{1E911}'), Some('\u{1E933}')) => continue,
            (Some('\u{1E933}'), Some('\u{1E911}')) => continue,
            (Some('\u{1E912}'), Some('\u{1E934}')) => continue,
            (Some('\u{1E934}'), Some('\u{1E912}')) => continue,
            (Some('\u{1E913}'), Some('\u{1E935}')) => continue,
            (Some('\u{1E935}'), Some('\u{1E913}')) => continue,
            (Some('\u{1E914}'), Some('\u{1E936}')) => continue,
            (Some('\u{1E936}'), Some('\u{1E914}')) => continue,
            (Some('\u{1E915}'), Some('\u{1E937}')) => continue,
            (Some('\u{1E937}'), Some('\u{1E915}')) => continue,
            (Some('\u{1E916}'), Some('\u{1E938}')) => continue,
            (Some('\u{1E938}'), Some('\u{1E916}')) => continue,
            (Some('\u{1E917}'), Some('\u{1E939}')) => continue,
            (Some('\u{1E939}'), Some('\u{1E917}')) => continue,
            (Some('\u{1E918}'), Some('\u{1E93A}')) => continue,
            (Some('\u{1E93A}'), Some('\u{1E918}')) => continue,
            (Some('\u{1E919}'), Some('\u{1E93B}')) => continue,
            (Some('\u{1E93B}'), Some('\u{1E919}')) => continue,
            (Some('\u{1E91A}'), Some('\u{1E93C}')) => continue,
            (Some('\u{1E93C}'), Some('\u{1E91A}')) => continue,
            (Some('\u{1E91B}'), Some('\u{1E93D}')) => continue,
            (Some('\u{1E93D}'), Some('\u{1E91B}')) => continue,
            (Some('\u{1E91C}'), Some('\u{1E93E}')) => continue,
            (Some('\u{1E93E}'), Some('\u{1E91C}')) => continue,
            (Some('\u{1E91D}'), Some('\u{1E93F}')) => continue,
            (Some('\u{1E93F}'), Some('\u{1E91D}')) => continue,
            (Some('\u{1E91E}'), Some('\u{1E940}')) => continue,
            (Some('\u{1E940}'), Some('\u{1E91E}')) => continue,
            (Some('\u{1E91F}'), Some('\u{1E941}')) => continue,
            (Some('\u{1E941}'), Some('\u{1E91F}')) => continue,
            (Some('\u{1E920}'), Some('\u{1E942}')) => continue,
            (Some('\u{1E942}'), Some('\u{1E920}')) => continue,
            (Some('\u{1E921}'), Some('\u{1E943}')) => continue,
            (Some('\u{1E943}'), Some('\u{1E921}')) => continue,
            _ => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::casecmp;

    #[test]
    fn compares_symbols_without_regard_to_case() {
        assert!(!casecmp("abcdef", "abcde"));
        assert!(casecmp("aBcDeF", "abcdef"));
        assert!(!casecmp("abcdef", "abcdefg"));
        assert!(casecmp("abcdef", "ABCDEF"));
    }

    #[test]
    fn doesent_consider_non_ascii_chars_equal_that_arent() {
        // -- UTF-8 --
        let upper_a_tilde = "Ã";
        let lower_a_tilde = "ã";
        let upper_a_umlaut = "Ä";
        let lower_a_umlaut = "ä";

        // From `spec/core/symbol/casecmp_spec.rb`:
        //
        // ```ruby
        // lower_a_tilde.casecmp?(lower_a_umlaut).should_not == true
        // lower_a_umlaut.casecmp?(lower_a_tilde).should_not == true
        // upper_a_tilde.casecmp?(upper_a_umlaut).should_not == true
        // upper_a_umlaut.casecmp?(upper_a_tilde).should_not == true
        // ```
        assert!(!casecmp(lower_a_tilde, lower_a_umlaut));
        assert!(!casecmp(lower_a_umlaut, lower_a_tilde));
        assert!(!casecmp(upper_a_tilde, upper_a_umlaut));
        assert!(!casecmp(upper_a_umlaut, upper_a_tilde));
    }

    #[test]
    fn does_case_mapping_for_unicode_chars() {
        // -- UTF-8 --
        let upper_a_tilde = "Ã";
        let lower_a_tilde = "ã";
        let upper_a_umlaut = "Ä";
        let lower_a_umlaut = "ä";

        // From `spec/core/symbol/casecmp_spec.rb`:
        //
        // ```ruby
        // upper_a_tilde.casecmp?(lower_a_tilde).should == true
        // upper_a_umlaut.casecmp?(lower_a_umlaut).should == true
        // lower_a_tilde.casecmp?(upper_a_tilde).should == true
        // lower_a_umlaut.casecmp?(upper_a_umlaut).should == true
        // ```
        assert!(casecmp(upper_a_tilde, lower_a_tilde));
        assert!(casecmp(upper_a_umlaut, lower_a_umlaut));
        assert!(casecmp(lower_a_tilde, upper_a_tilde));
        assert!(casecmp(lower_a_umlaut, upper_a_umlaut));
    }

    #[test]
    fn does_case_mapping_for_turkic_unicode_chars() {
        // -- UTF-8 --
        let upper_dotless_i = "I";
        let lower_dotless_i = "ı";
        let upper_dotted_i = "İ";
        let lower_dotted_i = "i";

        assert!(casecmp(upper_dotless_i, lower_dotless_i));
        assert!(casecmp(upper_dotted_i, lower_dotted_i));
        assert!(casecmp(lower_dotless_i, upper_dotless_i));
        assert!(casecmp(lower_dotted_i, upper_dotted_i));

        assert!(!casecmp(upper_dotless_i, upper_dotted_i));
        assert!(!casecmp(upper_dotless_i, lower_dotted_i));
        assert!(!casecmp(lower_dotless_i, upper_dotted_i));
        assert!(!casecmp(lower_dotless_i, lower_dotted_i));
        assert!(!casecmp(upper_dotted_i, upper_dotless_i));
        assert!(!casecmp(upper_dotted_i, lower_dotless_i));
        assert!(!casecmp(lower_dotted_i, upper_dotless_i));
        assert!(!casecmp(lower_dotted_i, lower_dotless_i));
    }
}
