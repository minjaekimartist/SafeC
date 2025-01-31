mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2
    {
        println!("Usage: safe_c <file> [-D<definition>...] [-I<include_path>...] [-L<link_file>...] [--release]");
        return;
    }
    let mut is_release = false;
    let mut code = vec![];
    let mut include_paths = vec![];

    for arg in &args[1..]
    {
        if arg.starts_with("-L") || arg.starts_with("-l")
        {
            let link_files = arg.replace("-L", "").replace("-l", "");
            if !(link_files.ends_with(".a") || link_files.ends_with(".lib") || link_files.ends_with(".so") || link_files.ends_with(".dylib") || link_files.ends_with(".dll")) { break; }
            let mut input = vec![];
            let mut file = std::fs::File::open(link_files).expect("Failed to open the file.");
            std::io::Read::read_to_end(&mut file, &mut input).expect("Failed to read the file.");
            // Link Library
        }
        else if arg.starts_with("-D")
        {
            let definition = arg.replace("-D", "");
            if definition.contains("=") == false { break; }
            let definition = definition.replace(" ", "").replace("=", " ");
            code.append(&mut lexer(&format!("#define {}", definition)));
        }
        else if arg.starts_with("-I")
        {
            let include_path = arg.replace("-I", "");
            if include_path.ends_with("/") == false { break; }
            include_paths.push(include_path);
        }
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
    for file in &code
    {
        parser(&include_paths, file);
    }
    if is_release
    {
        // Compile in Release Profile
    }
}
