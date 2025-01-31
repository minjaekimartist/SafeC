use crate::lexer::*;

pub(crate) fn parser<S: ToString>(include_path : &[S], tokens : &[Token]) -> String
{
    let mut ir = String::new();
    println!("{:?}", tokens);
    if tokens.len() == 0 { return ir }
    match tokens[0]
    {
        Token::Return => todo!(),
        Token::Preprocessor(preprocessor) =>
        {
            match preprocessor
            {
                Preprocessor::If => todo!(),
                Preprocessor::Elif => todo!(),
                Preprocessor::Else => todo!(),
                Preprocessor::Endif => todo!(),
                Preprocessor::Ifdef => todo!(),
                Preprocessor::Ifndef => todo!(),
                Preprocessor::Elifdef => todo!(),
                Preprocessor::Elifndef => todo!(),
                Preprocessor::Define => todo!(),
                Preprocessor::Undef => todo!(),
                Preprocessor::Include =>
                {
                    let mut input = vec![];
                    if let Token::Str(string) = &tokens[1]
                    {
                        for path in include_path
                        {
                            let mut path = path.to_string();
                            path.push_str(string);
                            if let Ok(mut file) = std::fs::File::open(path)
                            {
                                std::io::Read::read_to_end(&mut file, &mut input).expect("Failed to read the file.");
                                let text = String::from_utf8(input).expect("Failed to read as UTF-8");
                                for line in lexer(&text)
                                {
                                    ir.push_str(&parser(include_path, &line));
                                }
                                break;
                            }
                        }
                    }
                },
                Preprocessor::Error => todo!(),
                Preprocessor::Warning => todo!(),
                Preprocessor::Pragma => todo!(),
                Preprocessor::Defined => todo!(),
            }
        },
        Token::Type(syntax) => todo!(),
        Token::BlockStart => todo!(),
        Token::BlockEnd => todo!(),
        Token::ParenthesisStart => todo!(),
        Token::ParenthesisEnd => todo!(),
        Token::BracketStart => todo!(),
        Token::BracketEnd => todo!(),
        Token::Static => todo!(),
        Token::Const => todo!(),
        Token::If => todo!(),
        Token::Else => todo!(),
        Token::Switch => todo!(),
        Token::Case => todo!(),
        Token::Default => todo!(),
        Token::For => todo!(),
        Token::Do => todo!(),
        Token::While => todo!(),
        Token::Goto => todo!(),
        Token::Continue => todo!(),
        Token::Break => todo!(),
        Token::Typedef => todo!(),
        Token::Inline => todo!(),
        Token::Extern => todo!(),
        _ =>
        {
            panic!("Unexpected syntax: {:?}", tokens[0]);
        }
    }
    ir
}