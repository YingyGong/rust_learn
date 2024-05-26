use crate::earley_parse::CFG;
pub use crate::earley_parse::nt;
pub use crate::earley_parse::tr;

pub fn cfg_for_regular_expression() -> CFG {
    let mut cfg = CFG::new("RE");

    // union
    cfg.add_rule("RE", vec![nt("RE"), tr('|'), nt("RE")]);

    // concatenation 
    cfg.add_rule("RE", vec![nt("RE"), nt("RE")]);

    // Kleene star
    cfg.add_rule("RE", vec![nt("RE"), tr('*')]);

    // pluss
    cfg.add_rule("RE", vec![nt("RE"), tr('+')]);

    // question mark
    cfg.add_rule("RE", vec![nt("RE"), tr('?')]);

    // parentheses
    cfg.add_rule("RE", vec![tr('('), nt("RE"), tr(')')]);


    // Tab (0x09) and all characters between space (0x20) and tilde (0x7E), 
    // except { |, *, (, ), ., +, ?, \} are regular expressions (literals).
    for c in 0x20u8..=0x7E {
        let ch = c as char;
        if !"{|*()+?\\}".contains(ch) {
            cfg.add_rule("RE", vec![tr(ch)])
        }
    }

    // escaped special characters
    for &c in &['|', '*', '(', ')', '+', '?', '\\'] {
        cfg.add_rule("RE", vec![tr('\\'), tr(c)]);
    }

    // dot (any character)
    cfg.add_rule("RE", vec![tr('.')]);

    // character classes
    cfg.add_rule("RE", vec![tr('\\'), tr('s')]); // whitespace
    cfg.add_rule("RE", vec![tr('\\'), tr('S')]); // non-whitespace
    cfg.add_rule("RE", vec![tr('\\'), tr('d')]); // digit
    cfg.add_rule("RE", vec![tr('\\'), tr('D')]); // non-digit
    cfg.add_rule("RE", vec![tr('\\'), tr('w')]); // word character (alphanumeric + underscore)
    cfg.add_rule("RE", vec![tr('\\'), tr('W')]); // non-word character

    cfg
}