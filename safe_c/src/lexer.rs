#[derive(PartialEq, Debug)]
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
    fn from(input : &str) -> Option<Type>
    {
        match input
        {
            "auto" => Some(Type::Auto),
            "signed" => Some(Type::Signed),
            "unsigned" => Some(Type::Unsigned),
            "char" => Some(Type::Char),
            "short" => Some(Type::Short),
            "int" => Some(Type::Int),
            "long" => Some(Type::Long),
            "float" => Some(Type::Float),
            "double" => Some(Type::Double),
            "bool" => Some(Type::Bool),
            "void" => Some(Type::Void),
            "enum" => Some(Type::Enum),
            "union" => Some(Type::Union),
            "struct" => Some(Type::Struct),
            "*" => Some(Type::RawPointer),
            "&" => Some(Type::SafePointer),
            _ => None
        }
    }
}

#[derive(Clone)]
enum ContainsOperator
{
    True(Operator),
    False(String)
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
    fn from(input : &str) -> Vec<ContainsOperator>
    {
        if input.starts_with("&") { return [ContainsOperator::True(Operator::Reference), ContainsOperator::False(String::from(&input[1..]))].to_vec() }
        else if input.starts_with("*") { return [ContainsOperator::True(Operator::Dereference), ContainsOperator::False(String::from(&input[1..]))].to_vec() }
        else if input.contains("!=")
        {
            let strings : Vec<&str> = input.split("!=").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::NotEqual)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::NotEqual), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.starts_with("!") { return [ContainsOperator::True(Operator::Not), ContainsOperator::False(String::from(&input[1..]))].to_vec() }
        else if input.contains(".")
        {
            let strings : Vec<&str> = input.split(".").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::AccessStruct)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::AccessStruct), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("->")
        {
            let strings : Vec<&str> = input.split("->").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::AccessStructPointer)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::AccessStructPointer), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("::")
        {
            let strings : Vec<&str> = input.split("::").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::AccessEnum)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::AccessEnum), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("+")
        {
            let strings : Vec<&str> = input.split("+").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Add)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Add), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("-")
        {
            let strings : Vec<&str> = input.split("-").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Subtract)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Subtract), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("*")
        {
            let strings : Vec<&str> = input.split("*").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Multiply)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Multiply), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("/")
        {
            let strings : Vec<&str> = input.split("/").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Divide)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Divide), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("%")
        {
            let strings : Vec<&str> = input.split("%").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Remain)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Remain), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("&&")
        {
            let strings : Vec<&str> = input.split("&&").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::And)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::And), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("||")
        {
            let strings : Vec<&str> = input.split("||").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Or)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Or), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("&")
        {
            let strings : Vec<&str> = input.split("&").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::BitwiseAnd)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::BitwiseAnd), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("|")
        {
            let strings : Vec<&str> = input.split("|").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::BitwiseOr)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::BitwiseOr), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("^")
        {
            let strings : Vec<&str> = input.split("^").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::BitwiseXor)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::BitwiseXor), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains(">>")
        {
            let strings : Vec<&str> = input.split(">>").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::BitShiftRight)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::BitShiftRight), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("<<")
        {
            let strings : Vec<&str> = input.split("<<").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::BitShiftLeft)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::BitShiftLeft), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("==")
        {
            let strings : Vec<&str> = input.split("==").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Equal)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Equal), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains(">=")
        {
            let strings : Vec<&str> = input.split(">=").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::GreaterOrEqual)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::GreaterOrEqual), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains(">")
        {
            let strings : Vec<&str> = input.split(">").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Greater)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Greater), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("<=")
        {
            let strings : Vec<&str> = input.split("<=").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::LessOrEqual)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::LessOrEqual), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("<")
        {
            let strings : Vec<&str> = input.split("<").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Less)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Less), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains("=")
        {
            let strings : Vec<&str> = input.split("=").collect();
            if strings.is_empty() { return vec![ContainsOperator::True(Operator::Move)] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::True(Operator::Move), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        else if input.contains(",")
        {
            let strings : Vec<&str> = input.split(",").collect();
            if strings.is_empty() { return vec![] }
            return [ContainsOperator::False(String::from(strings[0])), ContainsOperator::False(String::from(strings[1]))].to_vec()
        }
        
        vec![]
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

#[derive(PartialEq, Debug)]
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
        else if !Operator::from(word).is_empty()
        {
            let chunk = Operator::from(word);
            let mut vector = vec![];
            for element in &chunk
            {               
                match element
                {
                    ContainsOperator::True(operator) => { vector.push(Token::Operator(*operator)); }
                    ContainsOperator::False(string) => { vector.append(&mut Token::from(&string)); }
                }
            }
            return vector
        }
        else if Type::from(word).is_some() { vec![Token::Type(Type::from(word).unwrap())] }
        else if word.contains("{")
        {
            let words : Vec<&str> = word.split("{").collect();
            let mut tokens = vec![];
            if words.len() > 1
            {
                for word in 0..words.len() - 1
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::BlockStart);
                }
                tokens.append(&mut Token::from(words[words.len() - 1]));
            }
            else { tokens.push(Token::BlockStart); }
            tokens
        }
        else if word.contains("}")
        {
            let words : Vec<&str> = word.split("}").collect();
            let mut tokens = vec![];
            if words.len() > 1
            {
                for word in 0..words.len() - 1
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::BlockEnd);
                }
                tokens.append(&mut Token::from(words[words.len() - 1]));
            }
            else { tokens.push(Token::BlockEnd); }
            tokens
        }
        else if word.contains("(")
        {
            let words : Vec<&str> = word.split("(").collect();
            let mut tokens = vec![];
            if words.len() > 1
            {
                for word in 0..words.len() - 1
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::ParenthesisStart);
                }
                tokens.append(&mut Token::from(words[words.len() - 1]));
            }
            else { tokens.push(Token::ParenthesisStart); }
            tokens
        }
        else if word.contains(")")
        {
            let words : Vec<&str> = word.split("(").collect();
            let mut tokens = vec![];
            if words.len() > 1
            {
                for word in 0..words.len() - 1
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::ParenthesisEnd);
                }
                tokens.append(&mut Token::from(words[words.len() - 1]));
            }
            else { tokens.push(Token::ParenthesisEnd); }
            tokens
        }
        else if word.contains("[")
        {
            let words : Vec<&str> = word.split("[").collect();
            let mut tokens = vec![];
            if words.len() > 1
            {
                for word in 0..words.len() - 1
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::BracketStart);
                }
                tokens.append(&mut Token::from(words[words.len() - 1]));
            }
            else { tokens.push(Token::BracketStart); }
            tokens
        }
        else if word.contains("]")
        {
            let words : Vec<&str> = word.split("]").collect();
            let mut tokens = vec![];
            if words.len() > 1
            {
                for word in 0..words.len() - 1
                {
                    tokens.append(&mut Token::from(words[word]));
                    tokens.push(Token::BracketEnd);
                }
                tokens.append(&mut Token::from(words[words.len() - 1]));
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

pub(crate) fn lexer(text : &str) -> Vec<Token>
{
    let mut output = vec![];
    let lines: Vec<&str> = text.lines().collect();
    let mut line = 0;
    loop
    {
        if line >= lines.len() { break; }
        let words: Vec<&str> = lines[line].split_whitespace().collect();
        let mut index = 0;
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
                    output.push(Token::Str(String::from("")));
                },
                _ => { output.append(&mut token); }
            }
            index += 1;
        }
        line += 1;
    }

    return output;
}
