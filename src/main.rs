use lexer::token_stream::TokenStream;
mod lexer;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!();
        return Err("Invalud number of args");
    }

    let src_file_name = args.get(1).unwrap();
    let src = std::fs::read_to_string(src_file_name).unwrap();

    println!("{src}");

    let mut ts = TokenStream::new(&src);
    let tokens = ts.lex().unwrap();
    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}
