//use interpreter::Interpreter;
//use lexer::Lexer;

use runtime::Interpreter;

fn main() {
    /*let lexer = Lexer::from_file("./test-script/test.ttay").unwrap();

    for token in lexer {
        match token {
            Ok(t) => println!("{:?}", t),
            Err(e) => println!("Error! {}", e),
        }
    }*/

    let interpreter = Interpreter::new("./test-script/test.ttay");
    let resp = interpreter.run();

    print!("\n");
    match resp {
        Some(r) => println!("{:?}", r),
        None => println!("No response")
    }
}
