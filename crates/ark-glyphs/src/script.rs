//! What Thai and Arabic need beyond one glyph per character, kept small
//! enough to run without a shaper. The bake tool compiles this same file,
//! so the clusters and letter forms it bakes are the ones the game asks for.
//!
//! - **Thai** stacks vowel and tone marks on a consonant. A consonant and
//!   its marks form one cluster, baked as one image with the font's own
//!   mark positioning.
//! - **Arabic** letters join their neighbours and change shape. Each letter
//!   becomes its Unicode presentation form (isolated, final, initial or
//!   medial), lam-alef becomes its ligature, and each form is baked from the
//!   font's own shaping. A line is then laid out right to left, with runs of
//!   Latin letters and numbers kept left to right inside it.

/// The longest Thai cluster: a consonant, an upper or lower vowel, a tone
/// mark and SARA AM.
pub const CLUSTER: usize = 4;

/// Thai characters that attach to the cluster before them: the combining
/// vowels and tone marks, and SARA AM, which carries its own NIKHAHIT.
pub const fn is_thai_mark(c: char) -> bool {
    matches!(c as u32, 0x0E31 | 0x0E33..=0x0E3A | 0x0E47..=0x0E4E)
}

/// Splits Thai text into clusters, calling `out` with each one. Characters
/// outside Thai come out alone.
pub fn thai_clusters(text: &str, mut out: impl FnMut(&[char])) {
    let mut cluster = ['\0'; CLUSTER];
    let mut len = 0;
    for c in text.chars() {
        if is_thai_mark(c) && len > 0 && len < CLUSTER {
            cluster[len] = c;
            len += 1;
            continue;
        }
        if len > 0 {
            out(&cluster[..len]);
        }
        cluster[0] = c;
        len = 1;
    }
    if len > 0 {
        out(&cluster[..len]);
    }
}

const TATWEEL: char = '\u{0640}';
const LAM: char = '\u{0644}';

/// How an Arabic letter joins (Unicode `ArabicShaping.txt`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Joining {
    /// Joins neither side (hamza).
    None,
    /// Joins only the letter before it (alef, dal, ra, waw …).
    Right,
    /// Joins both sides.
    Dual,
}

/// The four contextual shapes of a letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    Isolated,
    Final,
    Initial,
    Medial,
}

/// Each Arabic letter: how it joins, and its presentation forms in the
/// order isolated, final, initial, medial; zero where it has none.
const LETTERS: [(char, Joining, [u32; 4]); 36] = {
    use Joining::*;
    [
        ('\u{0621}', None, [0xFE80, 0, 0, 0]),
        ('\u{0622}', Right, [0xFE81, 0xFE82, 0, 0]),
        ('\u{0623}', Right, [0xFE83, 0xFE84, 0, 0]),
        ('\u{0624}', Right, [0xFE85, 0xFE86, 0, 0]),
        ('\u{0625}', Right, [0xFE87, 0xFE88, 0, 0]),
        ('\u{0626}', Dual, [0xFE89, 0xFE8A, 0xFE8B, 0xFE8C]),
        ('\u{0627}', Right, [0xFE8D, 0xFE8E, 0, 0]),
        ('\u{0628}', Dual, [0xFE8F, 0xFE90, 0xFE91, 0xFE92]),
        ('\u{0629}', Right, [0xFE93, 0xFE94, 0, 0]),
        ('\u{062A}', Dual, [0xFE95, 0xFE96, 0xFE97, 0xFE98]),
        ('\u{062B}', Dual, [0xFE99, 0xFE9A, 0xFE9B, 0xFE9C]),
        ('\u{062C}', Dual, [0xFE9D, 0xFE9E, 0xFE9F, 0xFEA0]),
        ('\u{062D}', Dual, [0xFEA1, 0xFEA2, 0xFEA3, 0xFEA4]),
        ('\u{062E}', Dual, [0xFEA5, 0xFEA6, 0xFEA7, 0xFEA8]),
        ('\u{062F}', Right, [0xFEA9, 0xFEAA, 0, 0]),
        ('\u{0630}', Right, [0xFEAB, 0xFEAC, 0, 0]),
        ('\u{0631}', Right, [0xFEAD, 0xFEAE, 0, 0]),
        ('\u{0632}', Right, [0xFEAF, 0xFEB0, 0, 0]),
        ('\u{0633}', Dual, [0xFEB1, 0xFEB2, 0xFEB3, 0xFEB4]),
        ('\u{0634}', Dual, [0xFEB5, 0xFEB6, 0xFEB7, 0xFEB8]),
        ('\u{0635}', Dual, [0xFEB9, 0xFEBA, 0xFEBB, 0xFEBC]),
        ('\u{0636}', Dual, [0xFEBD, 0xFEBE, 0xFEBF, 0xFEC0]),
        ('\u{0637}', Dual, [0xFEC1, 0xFEC2, 0xFEC3, 0xFEC4]),
        ('\u{0638}', Dual, [0xFEC5, 0xFEC6, 0xFEC7, 0xFEC8]),
        ('\u{0639}', Dual, [0xFEC9, 0xFECA, 0xFECB, 0xFECC]),
        ('\u{063A}', Dual, [0xFECD, 0xFECE, 0xFECF, 0xFED0]),
        ('\u{0641}', Dual, [0xFED1, 0xFED2, 0xFED3, 0xFED4]),
        ('\u{0642}', Dual, [0xFED5, 0xFED6, 0xFED7, 0xFED8]),
        ('\u{0643}', Dual, [0xFED9, 0xFEDA, 0xFEDB, 0xFEDC]),
        ('\u{0644}', Dual, [0xFEDD, 0xFEDE, 0xFEDF, 0xFEE0]),
        ('\u{0645}', Dual, [0xFEE1, 0xFEE2, 0xFEE3, 0xFEE4]),
        ('\u{0646}', Dual, [0xFEE5, 0xFEE6, 0xFEE7, 0xFEE8]),
        ('\u{0647}', Dual, [0xFEE9, 0xFEEA, 0xFEEB, 0xFEEC]),
        ('\u{0648}', Right, [0xFEED, 0xFEEE, 0, 0]),
        // Alef maksura is dual-joining; its initial and medial forms sit in
        // Presentation Forms-A.
        ('\u{0649}', Dual, [0xFEEF, 0xFEF0, 0xFBE8, 0xFBE9]),
        ('\u{064A}', Dual, [0xFEF1, 0xFEF2, 0xFEF3, 0xFEF4]),
    ]
};

/// Lam followed by these alefs is written as one ligature: (alef,
/// isolated ligature); the final form follows it.
const LAM_ALEF: [(char, u32); 4] = [
    ('\u{0622}', 0xFEF5),
    ('\u{0623}', 0xFEF7),
    ('\u{0625}', 0xFEF9),
    ('\u{0627}', 0xFEFB),
];

fn letter(c: char) -> Option<(Joining, [u32; 4])> {
    LETTERS.iter().find(|l| l.0 == c).map(|l| (l.1, l.2))
}

/// Whether `c` joins the letter after it.
fn joins_forward(c: char) -> bool {
    c == TATWEEL || letter(c).is_some_and(|(j, _)| j == Joining::Dual)
}

/// Whether `c` joins the letter before it.
fn joins_back(c: char) -> bool {
    c == TATWEEL || letter(c).is_some_and(|(j, _)| j != Joining::None)
}

/// Rewrites Arabic letters as their contextual presentation forms, in
/// logical order, calling `out` with each character. Everything else
/// passes through unchanged.
pub fn arabic_forms(text: &str, mut out: impl FnMut(char)) {
    let mut chars = text.chars().peekable();
    let mut previous_joins = false;
    while let Some(c) = chars.next() {
        if c == LAM
            && let Some(&next) = chars.peek()
            && let Some(&(_, ligature)) = LAM_ALEF.iter().find(|l| l.0 == next)
        {
            chars.next();
            let form = ligature + u32::from(previous_joins);
            out(char::from_u32(form).unwrap_or(c));
            // The ligature ends in an alef, which never joins forward.
            previous_joins = false;
            continue;
        }
        let Some((joining, forms)) = letter(c) else {
            out(c);
            previous_joins = c == TATWEEL;
            continue;
        };
        let back = previous_joins && joining != Joining::None;
        let forward = joining == Joining::Dual && chars.peek().is_some_and(|&n| joins_back(n));
        let index = match (back, forward) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        };
        let form = match forms[index] {
            0 => forms[0],
            f => f,
        };
        out(char::from_u32(form).unwrap_or(c));
        previous_joins = joins_forward(c);
    }
}

/// For a presentation form, the text that spells it and which shape it
/// takes: the bake tool shapes that text in context to find the glyph.
pub fn arabic_source(form: char) -> Option<([char; 2], usize, Form)> {
    let code = form as u32;
    for &(alef, ligature) in &LAM_ALEF {
        if code == ligature || code == ligature + 1 {
            let shape = if code == ligature {
                Form::Isolated
            } else {
                Form::Final
            };
            return Some(([LAM, alef], 2, shape));
        }
    }
    for &(base, _, forms) in &LETTERS {
        if let Some(i) = forms.iter().position(|&f| f != 0 && f == code) {
            let shape = [Form::Isolated, Form::Final, Form::Initial, Form::Medial][i];
            return Some(([base, '\0'], 1, shape));
        }
    }
    None
}

/// Whether `c` reads right to left: Arabic letters, their presentation
/// forms and Arabic punctuation.
pub const fn is_rtl(c: char) -> bool {
    matches!(c as u32, 0x0600..=0x06FF | 0xFB50..=0xFDFF | 0xFE70..=0xFEFF)
}

/// Text that is not RTL but has a direction of its own: letters and digits
/// of other scripts. Spaces, punctuation and icons take their neighbours'.
fn is_ltr(c: char) -> bool {
    !is_rtl(c) && c.is_alphanumeric()
}

/// The glyph a mirrored bracket shows in right-to-left text.
fn mirror(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '«' => '»',
        '»' => '«',
        '<' => '>',
        '>' => '<',
        other => other,
    }
}

/// Reorders one line of a right-to-left paragraph from logical to visual
/// order, left to right. A simplified bidi algorithm: runs of left-to-right
/// letters and digits, with the neutrals between them, keep their order;
/// everything else reverses, with brackets mirrored. `out` must be as long
/// as `line`.
pub fn visual(line: &[char], out: &mut [char]) {
    let n = line.len().min(out.len());
    let ltr = |i: usize| -> bool {
        if is_ltr(line[i]) {
            return true;
        }
        if is_rtl(line[i]) {
            return false;
        }
        // A neutral joins a left-to-right run only from inside it.
        let before = line[..i].iter().rev().find(|c| is_ltr(**c) || is_rtl(**c));
        let after = line[i + 1..n].iter().find(|c| is_ltr(**c) || is_rtl(**c));
        before.is_some_and(|c| is_ltr(*c)) && after.is_some_and(|c| is_ltr(*c))
    };
    let mut i = 0;
    while i < n {
        let start = i;
        let run_ltr = ltr(i);
        while i < n && ltr(i) == run_ltr {
            i += 1;
        }
        // The run lands mirrored across the line: [start, i) fills
        // [n - i, n - start), forward if left-to-right, reversed if not.
        for (k, &c) in line[start..i].iter().enumerate() {
            if run_ltr {
                out[n - i + k] = c;
            } else {
                out[n - 1 - start - k] = mirror(c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::{string::String, vec::Vec};

    fn forms(s: &str) -> Vec<u32> {
        let mut out = Vec::new();
        arabic_forms(s, |c| out.push(c as u32));
        out
    }

    #[test]
    fn letters_take_their_contextual_forms() {
        // بيت: beh initial, yeh medial, teh final.
        assert_eq!(forms("بيت"), [0xFE91, 0xFEF4, 0xFE96]);
        // دار: dal and alef never join forward, so all three stand isolated.
        assert_eq!(forms("دار"), [0xFEA9, 0xFE8D, 0xFEAD]);
        // بر: ra joins back to beh.
        assert_eq!(forms("بر"), [0xFE91, 0xFEAE]);
        // A space breaks joining.
        assert_eq!(forms("ب ب"), [0xFE8F, 0x20, 0xFE8F]);
        // Lam-alef is one ligature, final after a joining letter.
        assert_eq!(forms("لا"), [0xFEFB]);
        assert_eq!(forms("سلام"), [0xFEB3, 0xFEFC, 0xFEE1]);
    }

    #[test]
    fn every_form_traces_back_to_its_letters() {
        for c in "ءآأؤإئابةتثجحخدذرزسشصضطظعغفقكلمنهوىي".chars()
        {
            for f in forms(&[c, c, c].iter().collect::<String>()) {
                let (text, len, _) = arabic_source(char::from_u32(f).unwrap()).unwrap();
                assert_eq!(&text[..len], &[c]);
            }
        }
        let (text, len, form) = arabic_source('\u{FEFC}').unwrap();
        assert_eq!(
            (&text[..len], form),
            (&['\u{0644}', '\u{0627}'][..], Form::Final)
        );
    }

    fn reorder(s: &str) -> String {
        let line: Vec<char> = s.chars().collect();
        let mut out = std::vec!['?'; line.len()];
        visual(&line, &mut out);
        out.into_iter().collect()
    }

    #[test]
    fn lines_read_right_to_left_with_latin_runs_intact() {
        assert_eq!(reorder("ابت"), "تبا");
        assert_eq!(reorder("Esc للرجوع"), "عوجرلل Esc");
        assert_eq!(reorder("القطاع 03 · اسم"), "مسا · 03 عاطقلا");
        assert_eq!(reorder("أ 1 / 12 ب"), "ب 1 / 12 أ");
        assert_eq!(reorder("(ا)"), "(ا)");
        assert_eq!(reorder(""), "");
    }

    #[test]
    fn thai_marks_stay_with_their_consonant() {
        let mut out: Vec<String> = Vec::new();
        thai_clusters("ที่น้ำ a", |c| out.push(c.iter().collect()));
        assert_eq!(out, ["ที่", "น้ำ", " ", "a"]);
    }
}
