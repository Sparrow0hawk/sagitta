use sagitta::{find_line, JobInfo};

struct Args {
    file: String,
    job_id: i32,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut file = None;
    let mut job_id = 1;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('j') | Long("job-id") => {
                job_id = parser.value()?.parse()?;
            }
            Value(val) if file.is_none() => {
                file = Some(val.into_string()?);
            }
            Long("help") => {
                println!("Usage: sagitta [-j|--job-id=JOB_ID] FILE");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        file: file.ok_or("missing argument FILE")?,
        job_id,
    })
}

fn main() -> Result<(), anyhow::Error> {
    let args = parse_args()?;

    let line = find_line(args.file, args.job_id)?;

    match line {
        Some(line) => println!("{}", JobInfo::new(line.split(":").collect::<Vec<&str>>())),
        _ => println!("No job with ID {}", args.job_id),
    }

    Ok(())
}
