import dom::style::{Unit, In, Cm, Mm, Percent, Em, Ex, Pt, Pc, Px};
import str::{pop_char, from_chars};
import float::from_str;
import option::map;

fn parse_unit(-str : str) -> option<Unit> {
    let mut str = str;
    if str.len() < 1 { ret none }
    let last_char = pop_char(str);
    if last_char == '%' {
        ret from_str(str).map(|f| Percent(f));
    }

    if str.len() < 1 { ret none; }
    let second_last = pop_char(str);

    alt from_chars([second_last, last_char]/2) {
      "in" { from_str(str).map(|f| In(f)) }
      "cm" { from_str(str).map(|f| Cm(f)) }
      "mm" { from_str(str).map(|f| Mm(f)) }
      "pt" { from_str(str).map(|f| Pt(f)) }
      "pc" { from_str(str).map(|f| Pc(f)) }
      "px" { from_str(str).map(|f| Px(f)) }
      "em" { from_str(str).map(|f| Em(f)) }
      "ex" { from_str(str).map(|f| Ex(f)) }
      _    { none }
    }
}

