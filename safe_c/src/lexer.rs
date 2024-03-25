#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Type
{
    Auto,
    Signed,
    Unsigned,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Bool,
    Void,
    Enum,
    Union,
    Struct,
    RawPointer,
    SafePointer
}
impl Type
{
    fn from(input : &str) -> Option<Vec<Token>>
    {
        let input_vec;
        let pointer = if input.contains("*")
        {
            input_vec = input.split("*").collect();
            Some(Type::RawPointer)
        }
        else if input.contains("&")
        {
            input_vec = input.split("&").collect();
            Some(Type::SafePointer)
        }
        else
        {
            input_vec = vec![input];
            None
        };

        let c_type = if input_vec[0] == "auto" { Some(Type::Auto) }
        else if input_vec[0] == "signed" { Some(Type::Signed) }
        else if input_vec[0] == "unsigned" { Some(Type::Unsigned) }
        else if input_vec[0] == "char" { Some(Type::Char) }
        else if input_vec[0] == "short" { Some(Type::Short) }
        else if input_vec[0] == "int" { Some(Type::Int) }
        else if input_vec[0] == "long" { Some(Type::Long) }
        else if input_vec[0] == "float" { Some(Type::Float) }
        else if input_vec[0] == "double" { Some(Type::Double) }
        else if input_vec[0] == "bool" { Some(Type::Bool) }
        else if input_vec[0] == "void" { Some(Type::Void) }
        else if input_vec[0] == "enum" { Some(Type::Enum) }
        else if input_vec[0] == "union" { Some(Type::Union) }
        else if input_vec[0] == "struct" { Some(Type::Struct) }
        else { None };

        if c_type.is_none() { None }
        else
        {
            let mut vector = vec![Token::Type(c_type.unwrap())];
            if pointer.is_some() { vector.push(Token::Type(pointer.unwrap())); }
            if input_vec.len() > 0
            {
                for index in 1..input_vec.len() { vector.append(&mut Token::from(input_vec[index])); }
            }

            return Some(vector)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Operator
{
    Add,
    Subtract,
    Multiply,
    Divide,
    Remain,
    Reference,
    Dereference,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Not,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitShiftRight,
    BitShiftLeft,
    Move,
    AccessStruct,
    AccessStructPointer,
    AccessEnum
}
impl Operator
{
    fn from(input : &str) -> Option<Vec<Token>>
    {
        if input.starts_with("&")
        {
            let mut vector = vec![Token::Operator(Operator::Reference)];
            vector.append(&mut Token::from(&input[1..]));
            Some(vector)
        }
        else if input.starts_with("*")
        {
            let mut vector = vec![Token::Operator(Operator::Dereference)];
            vector.append(&mut Token::from(&input[1..]));
            Some(vector)
        }
        else if input.contains("!=")
        {
            let strings : Vec<&str> = input.split("!=").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::NotEqual)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::NotEqual));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.starts_with("!")
        {
            let mut vector = vec![Token::Operator(Operator::Not)];
            vector.append(&mut Token::from(&input[1..]));
            Some(vector)
        }
        else if input.contains(".")
        {
            let strings : Vec<&str> = input.split(".").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::AccessStruct)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::AccessStruct));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("->")
        {
            let strings : Vec<&str> = input.split("->").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::AccessStructPointer)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::AccessStructPointer));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("::")
        {
            let strings : Vec<&str> = input.split("::").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::AccessEnum)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::AccessEnum));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("+")
        {
            let strings : Vec<&str> = input.split("+").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Add)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Add));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("-")
        {
            let strings : Vec<&str> = input.split("-").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Subtract)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Subtract));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("*")
        {
            let strings : Vec<&str> = input.split("*").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Multiply)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Multiply));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("/")
        {
            let strings : Vec<&str> = input.split("/").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Divide)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Divide));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("%")
        {
            let strings : Vec<&str> = input.split("%").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Remain)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Remain));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("&&")
        {
            let strings : Vec<&str> = input.split("&&").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::And)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::And));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("||")
        {
            let strings : Vec<&str> = input.split("||").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Or)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Or));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input == "&"
        {
            let strings : Vec<&str> = input.split("&").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::BitwiseAnd)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::BitwiseAnd));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input == "|"
        {
            let strings : Vec<&str> = input.split("|").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::BitwiseOr)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::BitwiseOr));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input == "^"
        {
            let strings : Vec<&str> = input.split("^").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::BitwiseXor)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::BitwiseXor));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains(">>")
        {
            let strings : Vec<&str> = input.split(">>").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::BitShiftRight)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::BitShiftRight));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("<<")
        {
            let strings : Vec<&str> = input.split("<<").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::BitShiftLeft)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::BitShiftLeft));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("==")
        {
            let strings : Vec<&str> = input.split("==").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Equal)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Equal));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains(">=")
        {
            let strings : Vec<&str> = input.split(">=").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::GreaterOrEqual)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::GreaterOrEqual));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains(">")
        {
            let strings : Vec<&str> = input.split(">").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Greater)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Greater));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("<=")
        {
            let strings : Vec<&str> = input.split("<=").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::LessOrEqual)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::LessOrEqual));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("<")
        {
            let strings : Vec<&str> = input.split("<").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Less)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Less));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains("=")
        {
            let strings : Vec<&str> = input.split("=").collect();
            if strings.is_empty() { return Some(vec![Token::Operator(Operator::Move)]) }
            let mut vector = Token::from(strings[0]);
            vector.push(Token::Operator(Operator::Move));
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else if input.contains(",")
        {
            let strings : Vec<&str> = input.split(",").collect();
            if strings.is_empty() { return None }
            let mut vector = Token::from(strings[0]);
            vector.append(&mut Token::from(strings[1]));
            Some(vector)
        }
        else { None }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Preprocessor
{
    If,
    Elif,
    Else,
    Endif,
    Ifdef,
    Ifndef,
    Elifdef,
    Elifndef,
    Define,
    Undef,
    Include,
    Error,
    Warning,
    Pragma,
    Defined
}
impl Preprocessor
{
    fn from(word : &str) -> Option<Preprocessor>
    {
        match word
        {
            "if" => Some(Preprocessor::If),
            "elif" => Some(Preprocessor::Elif),
            "else" => Some(Preprocessor::Else),
            "endif" => Some(Preprocessor::Endif),
            "ifdef" => Some(Preprocessor::Ifdef),
            "ifndef" => Some(Preprocessor::Ifndef),
            "elifdef" => Some(Preprocessor::Elifdef),
            "elifndef" => Some(Preprocessor::Elifndef),
            "define" => Some(Preprocessor::Define),
            "undef" => Some(Preprocessor::Undef),
            "include" => Some(Preprocessor::Include),
            "error" => Some(Preprocessor::Error),
            "warning" => Some(Preprocessor::Warning),
            "pragma" => Some(Preprocessor::Pragma),
            "defined" => Some(Preprocessor::Defined),
            _ => None
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum Token
{
    Return,
    LineEnd,
    Preprocessor(Preprocessor),
    Operator(Operator),
    Type(Type),
    BlockStart,
    BlockEnd,
    ParenthesisStart,
    ParenthesisEnd,
    BracketStart,
    BracketEnd,
    Static,
    Const,
    If,
    Else,
    Switch,
    Case,
    Default,
    For,
    Do,
    While,
    Goto,
    Continue,
    Break,
    Typedef,
    Inline,
    Extern,
    Int(isize),
    UInt(usize),
    Float(f64),
    Str(String),
}
impl Token
{
    fn from(word : &str) -> Vec<Token>
    {
        if word.ends_with(";")
        {
            let mut tokens = Token::from(&word[..word.len() - 1]);
            tokens.append(&mut vec![Token::LineEnd]);
            tokens
        } 
        else if word.starts_with("#") { vec![Token::Preprocessor(Preprocessor::from(&word[1..]).expect("Wrong Preprocessor Token!"))] }
        else if Operator::from(word).is_some() { Operator::from(word).unwrap() }
        else if Type::from(word).is_some() { Type::from(word).unwrap() }
        else if word.contains("{")
        {
            let words : Vec<&str> = word.split("{").collect();
            let mut tokens = vec![];
            if words.len() > 0
            {
                for word in 0..words.len()
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::BlockStart);
                }
                tokens.pop();
            }
            else { tokens.push(Token::BlockStart); }
            tokens
        }
        else if word.contains("}")
        {
            let words : Vec<&str> = word.split("}").collect();
            let mut tokens = vec![];
            if words.len() > 0
            {
                for word in 0..words.len()
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::BlockEnd);
                }
                tokens.pop();
            }
            else { tokens.push(Token::BlockEnd); }
            tokens
        }
        else if word.contains("(")
        {
            let words : Vec<&str> = word.split("(").collect();
            let mut tokens = vec![];
            if words.len() > 0
            {
                for word in 0..words.len()
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::ParenthesisStart);
                }
                tokens.pop();
            }
            else { tokens.push(Token::ParenthesisStart); }
            tokens
        }
        else if word.contains(")")
        {
            let words : Vec<&str> = word.split(")").collect();
            let mut tokens = vec![];
            if words.len() > 0
            {
                for word in 0..words.len()
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::ParenthesisEnd);
                }
                tokens.pop();
            }
            else { tokens.push(Token::ParenthesisEnd); }
            tokens
        }
        else if word.contains("[")
        {
            let words : Vec<&str> = word.split("[").collect();
            let mut tokens = vec![];
            if words.len() > 0
            {
                for word in 0..words.len()
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::BracketStart);
                }
                tokens.pop();
            }
            else { tokens.push(Token::BracketStart); }
            tokens
        }
        else if word.contains("]")
        {
            let words : Vec<&str> = word.split("]").collect();
            let mut tokens = vec![];
            if words.len() > 0
            {
                for word in 0..words.len()
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::BracketEnd);
                }
                tokens.pop();
            }
            else { tokens.push(Token::BracketEnd); }
            tokens
        }
        else if word == "return" { vec![Token::Return] }
        else if word == "static" { vec![Token::Static] }
        else if word == "const" { vec![Token::Const] }
        else if word == "if" { vec![Token::If] }
        else if word == "else" { vec![Token::Else] }
        else if word == "switch" { vec![Token::Switch] }
        else if word == "case" { vec![Token::Case] }
        else if word == "default" { vec![Token::Default] }
        else if word == "for" { vec![Token::For] }
        else if word == "do" { vec![Token::Do] }
        else if word == "while" { vec![Token::While] }
        else if word == "goto" { vec![Token::Goto] }
        else if word == "continue"{ vec![Token::Continue] }
        else if word == "break" { vec![Token::Break] }
        else if word == "typedef" { vec![Token::Typedef] }
        else if word == "inline" { vec![Token::Inline] }
        else if word == "extern" { vec![Token::Extern] }
        else if word.parse::<isize>().is_ok() { vec![Token::Int(word.parse::<isize>().unwrap())] }
        else if word.parse::<usize>().is_ok() { vec![Token::UInt(word.parse::<usize>().unwrap())] }
        else if word.parse::<f64>().is_ok() { vec![Token::Float(word.parse::<f64>().unwrap())] }
        else { vec![Token::Str(String::from(word))] }
    }
}

pub(crate) fn lexer(text : &str) -> Vec<Vec<Token>>
{
    let mut result = vec![];
    let lines: Vec<&str> = text.lines().collect();
    let mut line = 0;
    loop
    {
        if line >= lines.len() { break; }
        let words: Vec<&str> = lines[line].split_whitespace().collect();
        let mut index = 0;
        let mut output = vec![];
        loop
        {
            if index >= words.len() { break; }
            if words[index].contains("/*")
            {
                if lines[line].contains("*/") { break; }
                line += 1;
            }
            if words[index] == "//" { break; }
            if words[index].contains("\"")
            {
                let mut string = String::new();
                let splitted_words : Vec<&str> = words[index].split("\"").collect();
                let mut token = Token::from(splitted_words[0]);
                token.pop();
                output.append(&mut token);
                string.push_str(splitted_words[1]);
                index += 1;
                if splitted_words.len() < 3
                {
                    loop
                    {
                        if words[index].contains("\"")
                        {
                            let splitted_words : Vec<&str> = words[index].split("\"").collect();
                            string.push_str(splitted_words[0]);
                            output.push(Token::Str(string));
                            output.append(&mut Token::from(splitted_words[1]));
                            index += 1;
                            break;
                        }
                        string.push_str(&words[index]);
                        index += 1;
                    }
                }
                else if splitted_words.len() > 3
                {
                    loop
                    {
                        if words[index].contains("\"")
                        {
                            let splitted_words : Vec<&str> = words[index].split("\"").collect();
                            string.push_str(splitted_words[0]);
                            output.push(Token::Str(string));
                            output.append(&mut Token::from(splitted_words[1]));
                            index += 1;
                            break;
                        }
                        string.push_str(&words[index]);
                        index += 1;
                    }
                    for word in index..splitted_words.len()
                    {
                        output.append(&mut Token::from(splitted_words[word]));
                    }
                }
            }
            if words[index].contains("\'")
            {
                let mut string = String::new();
                let splitted_words : Vec<&str> = words[index].split("\'").collect();
                let mut token = Token::from(splitted_words[0]);
                token.pop();
                output.append(&mut token);
                string.push_str(splitted_words[1]);
                index += 1;
                if splitted_words.len() < 3
                {
                    loop
                    {
                        if words[index].contains("\'")
                        {
                            let splitted_words : Vec<&str> = words[index].split("\'").collect();
                            string.push_str(splitted_words[0]);
                            output.push(Token::Str(string));
                            output.append(&mut Token::from(splitted_words[1]));
                            index += 1;
                            break;
                        }
                        string.push_str(&words[index]);
                        index += 1;
                    }
                }
                else if splitted_words.len() > 3
                {
                    loop
                    {
                        if words[index].contains("\'")
                        {
                            let splitted_words : Vec<&str> = words[index].split("\'").collect();
                            string.push_str(splitted_words[0]);
                            output.push(Token::Str(string));
                            output.append(&mut Token::from(splitted_words[1]));
                            index += 1;
                            break;
                        }
                        string.push_str(&words[index]);
                        index += 1;
                    }
                    for word in index..splitted_words.len()
                    {
                        output.append(&mut Token::from(splitted_words[word]));
                    }
                }
            }
            if words[index].contains("`")
            {
                let mut string = String::new();
                let splitted_words : Vec<&str> = words[index].split("`").collect();
                let mut token = Token::from(splitted_words[0]);
                token.pop();
                output.append(&mut token);
                string.push_str(splitted_words[1]);
                index += 1;
                if splitted_words.len() < 3
                {
                    loop
                    {
                        if words[index].contains("`")
                        {
                            let splitted_words : Vec<&str> = words[index].split("`").collect();
                            string.push_str(splitted_words[0]);
                            output.push(Token::Str(string));
                            output.append(&mut Token::from(splitted_words[1]));
                            index += 1;
                            break;
                        }
                        string.push_str(&words[index]);
                        index += 1;
                    }
                }
                else if splitted_words.len() > 3
                {
                    loop
                    {
                        if words[index].contains("`")
                        {
                            let splitted_words : Vec<&str> = words[index].split("`").collect();
                            string.push_str(splitted_words[0]);
                            output.push(Token::Str(string));
                            output.append(&mut Token::from(splitted_words[1]));
                            index += 1;
                            break;
                        }
                        string.push_str(&words[index]);
                        index += 1;
                    }
                    for word in index..splitted_words.len()
                    {
                        output.append(&mut Token::from(splitted_words[word]));
                    }
                }
            }
            if index >= words.len() { break; }
            let mut token = Token::from(words[index]);
            match token[0]
            {
                Token::Preprocessor(preprocessor) =>
                {
                    output.push(Token::Preprocessor(preprocessor));
                    token.remove(0);
                    output.append(&mut token);
                    index += 1;
                    let mut string = String::new();
                    while words[index] != ""
                    {
                        string.push_str(&words[index]);
                        index += 1;
                        if index >= words.len() { break; }
                    }
                    output.push(Token::Str(string));
                },
                _ => { output.append(&mut token); }
            }
            index += 1;
        }
        let mut index = 0;
        while index < output.len()
        {
            if output[index] == Token::Str(String::from("")) { output.remove(index); }
            index += 1;
        }
        let output: Vec<&[Token]> = output.split(|token| *token == Token::LineEnd).collect();
        for array in output { result.push(array.to_owned()); }
        line += 1;
    }
    let mut index = 0;
    while index < result.len()
    {
        if result[index] == vec![] { result.remove(index); }
            index += 1;
    }
    return result;
}
