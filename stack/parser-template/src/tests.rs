use crate::prelude::*;

#[test]
fn it_works() {
    let test_cases = vec!["SELECT 1", "SELECT 2+1", "SELECT 1,2,3"];

    for case in test_cases {
        match QueryParser::new(case).run() {
            Ok(result) => println!("{}: {:?}", case, result),
            Err(err) => println!("Failed to parse: {case}. Reason: {err}"),
        }
    }
}
