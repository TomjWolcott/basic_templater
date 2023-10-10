extern crate basic_templater;

use basic_templater::template;

#[test]
fn test() {
    let hello = 4;

    println!("{}", template!("{hello+3}, {2 * hello + 8 - if hello > 2 {3} else {5} }"))
}