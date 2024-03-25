mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn main()
{
    let args: Vec<String> = std::env::args().collect();

    let mut is_link = false;
    let mut is_release = false;
    let mut code = vec![];

    for arg in &args[1..]
    {
        if is_link
        {
            // Link Library
            is_link = false;
        }
        if arg == "-L" || arg == "-l" { is_link = true; }
        else if arg == "--release" { is_release = true; }
        else
        {
            let mut input = vec![];
            let mut file = std::fs::File::open(arg).expect("Failed to open the file.");
            std::io::Read::read_to_end(&mut file, &mut input).expect("Failed to read the file.");
            let text = String::from_utf8(input).expect("Failed to read as UTF-8");
            code.append(&mut lexer(&text));
        }
    }
    let lines : Vec<&[Token]> = code.split(|token| *token == Token::Str(String::from(""))).collect();
    for line in lines
    {
        // parse
    }
    if is_release
    {
        // Compile in Release Profile
    }
}
