extern crate basic_templater;

use basic_templater::template;

#[test]
fn test() {
    let hello = 4;

    println!("{}", template!("{hello+3 (:?)}, {f(2.0)}, {f(2.0) (:?)}, {2 * hello + 8 - if hello > 2 {3} else {5} }"))
}

fn f(x: f32) -> f32 {
    x * 2.0
}