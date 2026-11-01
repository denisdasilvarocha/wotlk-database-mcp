use std::fmt::Write;

/// Numeric spell fields used to expand description tokens.
#[derive(Debug, Clone, Default)]
pub(crate) struct SpellNumbers {
    pub(crate) base_points: [i32; 3],
    pub(crate) die_sides: [i32; 3],
    pub(crate) period: [i32; 3],
    pub(crate) radius: [Option<f64>; 3],
    pub(crate) chain_targets: [i32; 3],
    pub(crate) duration_ms: i32,
    pub(crate) range_max: f64,
}

/// Expands supported spell tokens, preserving formulas and unresolved tokens.
pub(crate) fn resolve<'a>(
    text: &str,
    spell: &SpellNumbers,
    lookup: &dyn Fn(i32) -> Option<&'a SpellNumbers>,
) -> String {
    let bytes = text.as_bytes();
    let mut result = String::with_capacity(text.len());
    let mut cursor = 0;
    while let Some(relative) = text[cursor..].find('$') {
        let start = cursor + relative;
        result.push_str(&text[cursor..start]);
        let mut end = start + 1;
        if bytes.get(end) == Some(&b'{') {
            end += 1;
            let mut depth = 1usize;
            while end < bytes.len() && depth != 0 {
                match bytes[end] {
                    b'{' => depth += 1,
                    b'}' => depth -= 1,
                    _ => {}
                }
                end += 1;
            }
            result.push_str(&text[start..end]);
            cursor = end;
            continue;
        }

        let id_start = end;
        while bytes.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        let id_end = end;
        let Some(&kind) = bytes.get(end) else {
            result.push_str(&text[start..end]);
            cursor = end;
            continue;
        };
        let kind = kind.to_ascii_lowercase();
        if !matches!(kind, b's' | b'm' | b'd' | b'o' | b't' | b'a' | b'x' | b'r') {
            // Conditionals are opaque, including any tokens in their branches.
            if matches!(kind, b'l' | b'g') {
                if let Some(relative_end) = text[end..].find(';') {
                    end += relative_end + 1;
                } else {
                    end = text.len();
                }
            }
            result.push_str(&text[start..end]);
            cursor = end;
            continue;
        }
        end += 1;
        let index_start = end;
        while bytes.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        let index = if index_start == end {
            Some(0)
        } else {
            match &text[index_start..end] {
                "1" => Some(0),
                "2" => Some(1),
                "3" => Some(2),
                _ => None,
            }
        };
        let target = if id_start == id_end {
            Some(spell)
        } else {
            text[id_start..id_end].parse::<i32>().ok().and_then(lookup)
        };
        if !index
            .zip(target)
            .is_some_and(|(index, target)| append_value(&mut result, kind, index, target))
        {
            result.push_str(&text[start..end]);
        }
        cursor = end;
    }
    result.push_str(&text[cursor..]);
    result
}

fn append_value(output: &mut String, kind: u8, index: usize, spell: &SpellNumbers) -> bool {
    let base = i64::from(spell.base_points[index]);
    let sides = i64::from(spell.die_sides[index]);
    let low = (base + sides.min(1)).abs();
    let high = (base + sides).abs();
    match kind {
        b's' => {
            if sides <= 1 {
                let _ = write!(output, "{high}");
            } else {
                let _ = write!(output, "{low} to {high}");
            }
        }
        b'm' => {
            let _ = write!(output, "{low}");
        }
        b'd' if spell.duration_ms > 0 => {
            let ms = spell.duration_ms;
            if ms % 3_600_000 == 0 {
                let hours = ms / 3_600_000;
                let _ = write!(output, "{hours} hour{}", if hours == 1 { "" } else { "s" });
            } else if ms % 60_000 == 0 {
                let minutes = ms / 60_000;
                let _ = write!(output, "{minutes} min");
            } else {
                let seconds = f64::from(ms) / 1000.0;
                let _ = write!(output, "{seconds} sec");
            }
        }
        b'o' if spell.duration_ms > 0 && spell.period[index] > 0 => {
            let ticks = i64::from(spell.duration_ms / spell.period[index]);
            if sides <= 1 {
                let _ = write!(output, "{}", high * ticks);
            } else {
                let _ = write!(output, "{} to {}", low * ticks, high * ticks);
            }
        }
        b't' if spell.period[index] != 0 => {
            let _ = write!(output, "{}", f64::from(spell.period[index]) / 1000.0);
        }
        b'a' => {
            let Some(radius) = spell.radius[index] else {
                return false;
            };
            let _ = write!(output, "{radius}");
        }
        b'x' => {
            let _ = write!(output, "{}", spell.chain_targets[index]);
        }
        b'r' => {
            let _ = write!(output, "{}", spell.range_max);
        }
        _ => return false,
    }
    true
}
