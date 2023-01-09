use sagitta::LogFile;

struct Args {
    file: String,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut file = None;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) if file.is_none() => {
                file = Some(val.into_string()?);
            }
            Long("help") => {
                println!("Usage: sagitta FILE");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        file: file.ok_or("missing argument FILE")?,
    })
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;

    let parser = LogFile::new(&args.file);

    println!("{:?}", parser);
    Ok(())
}
