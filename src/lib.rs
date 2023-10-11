extern crate proc_macro;
use proc_macro::TokenStream;
use regex::Regex;

#[proc_macro]
pub fn template(input: TokenStream) -> TokenStream {
    let mut rust_expressions: Vec<String> = Vec::new();
    let mut template = input.to_string();

    let regex = Regex::new(r"\{\{|\}\}|\{|\(:[^)]+\)\}|\}").unwrap();
    let mut paren_level = 0;
    let mut expr_ranges = Vec::new();

    for capture in regex.captures_iter(template.as_str()) {
        let (mut str, []) = capture.extract::<0>();

        match str {
            "{" => {
                if paren_level == 0 {
                    expr_ranges.push((
                        capture.get(0).unwrap().end(),
                        capture.get(0).unwrap().end(),
                        capture.get(0).unwrap().end(),
                    ))
                }

                paren_level += 1;
            },
            "}" => {
                paren_level -= 1;

                if paren_level == 0 {
                    expr_ranges.last_mut().unwrap().1 = capture.get(0).unwrap().start();
                    expr_ranges.last_mut().unwrap().2 = capture.get(0).unwrap().start();
                }
            },
            "{{" | "}}" => {},
            formatter => {
                paren_level -= 1;

                if paren_level == 0 {
                    expr_ranges.last_mut().unwrap().1 = capture.get(0).unwrap().start();
                    expr_ranges.last_mut().unwrap().2 = capture.get(0).unwrap().start() + formatter.len();
                }
            }
        };
    }

    if paren_level > 0 {
        panic!("{}", "Found unmatched curly braces, make sure if you have nested braces you space them out: \"{if x {1} else {2}}\" -> \"{ if x {1} else {2} }\"")
    }

    for &(start, end1, end2) in expr_ranges.iter().rev() {
        rust_expressions.push(template[start..end1].to_string().replace("\\\"", "\""));
        template = if end1 == end2 {
            format!("{}{}", &template[..start], &template[end1..])
        } else {
            format!("{}{}{}", &template[..start], &template[end1+1..end2-2], &template[end2-1..])
        };
    }

    rust_expressions = rust_expressions.into_iter().rev().collect();\

    format!(
        "format!({}, {})",
        template,
        rust_expressions.join(", ")
    ).parse().unwrap()
}