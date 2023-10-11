extern crate basic_templater;

use basic_templater::template;

#[test]
fn test() {
    let hello = 4.0;

    println!("{}", template!("{ format!(\"{hello:?}\") (:?)} skdjbc if  sudc: sdicu hello { hello + 4.0 }"));
}

fn f(x: f32) -> f32 {
    x * 2.0
}