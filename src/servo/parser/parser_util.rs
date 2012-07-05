import dom::style::*;
import str::{pop_char, from_chars};
import float::from_str;
import option::map;

fn parse_unit(str : str) -> option<Unit> {
    alt str {
      s if s.ends_with("%")  { from_str(str.substr(0, str.len() - 1)).map(|f| Percent(f)) }
      s if s.ends_with("in") { from_str(str.substr(0, str.len() - 2)).map(|f| In(f)) }
      s if s.ends_with("cm") { from_str(str.substr(0, str.len() - 2)).map(|f| Cm(f)) }
      s if s.ends_with("mm") { from_str(str.substr(0, str.len() - 2)).map(|f| Mm(f)) }
      s if s.ends_with("pt") { from_str(str.substr(0, str.len() - 2)).map(|f| Pt(f)) }
      s if s.ends_with("pc") { from_str(str.substr(0, str.len() - 2)).map(|f| Pc(f)) }
      s if s.ends_with("px") { from_str(str.substr(0, str.len() - 2)).map(|f| Px(f)) }
      s if s.ends_with("em") { from_str(str.substr(0, str.len() - 2)).map(|f| Em(f)) }
      s if s.ends_with("ex") { from_str(str.substr(0, str.len() - 2)).map(|f| Ex(f)) }
      _    { none }
    }
}

fn parse_font_size(str : str) -> option<Unit> {
    // The default pixel size, not sure if this is accurate.
    let default = 16.0;

    alt str {
      "xx-small"  { some(Px(0.6*default)) }
      "x-small"  { some(Px(0.75*default)) }
      "small"  { some(Px(8.0/9.0*default)) }
      "medium"  { some(Px(default)) }
      "large"  { some(Px(1.2*default)) }
      "x-large"  { some(Px(1.5*default)) }
      "xx-large"  { some(Px(2.0*default)) }
      "smaller"  { some(Em(0.8)) }
      "larger"  { some(Em(1.25)) }
      "inherit"  { some(Em(1.0)) }
      _  { parse_unit(str) }
    }
}
