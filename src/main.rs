use quote::ToTokens;
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process,
};

#[derive(Debug)]
enum Error {
    IncorrectUsage,
    ReadFile(io::Error),
    ParseFile(syn::Error),
}

fn main() {
    if let Err(error) = try_main() {
        _ = writeln!(io::stderr(), "{error:?}");
        process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let mut args = env::args_os();
    _ = args.next(); // executable name

    let filepath = match (args.next(), args.next()) {
        (Some(arg), None) => PathBuf::from(arg),
        _ => return Err(Error::IncorrectUsage),
    };

    let code = fs::read_to_string(&filepath).map_err(Error::ReadFile)?;
    let syntax = syn::parse_file(&code).map_err(Error::ParseFile)?;
    println!("{:#?}", syntax);

    std::fs::write("./src/outuput.rs", syntax.into_token_stream().to_string()).unwrap();

    Ok(())
}
