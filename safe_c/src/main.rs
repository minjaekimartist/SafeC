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

    for arg in &args
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
            std::io::Read::read(&mut std::fs::File::open(arg).expect("Failed to open the file."), &mut input).expect("Failed to read the file.");
            code.append(&mut lexer(&input));
        }
    }
    let lines : Vec<&[Token]> = code.split(|token| *token == Token::LineEnd).collect();
    for line in lines
    {
        println!("{:?}", line);
    }
    if is_release
    {
        // Compile in Release Profile
    }
}
