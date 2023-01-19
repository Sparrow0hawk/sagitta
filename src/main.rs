use sagitta::{read_file, JobInfo};
use std::io::BufRead;

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
        job_id: job_id,
    })
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;

    let open_file = read_file(&args.file).unwrap_or_else(|e| panic!("Error opening file: {e}"));

    let line: _ = open_file
        .lines()
        .skip(4)
        .map(|l| l.unwrap())
        .filter(|x| x.split(":").nth(5).unwrap().parse::<i32>().unwrap() == args.job_id)
        .collect::<String>();

    if line.len() > 0 {
        println!("{:?}", JobInfo::new(line.split(":").collect::<Vec<&str>>()));
    } else {
        println!("No job with ID {}", args.job_id);
    }

    Ok(())
}
