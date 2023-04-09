use sagitta::{JobInfo, Seeker};

struct Args {
    file: String,
    job_id: i32,
    forward: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut file = None;
    let mut job_id = 1;
    let mut parser = lexopt::Parser::from_env();
    let mut forward = false;
    while let Some(arg) = parser.next()? {
        match arg {
            Short('j') | Long("job-id") => {
                job_id = parser.value()?.parse()?;
            }
            Value(val) if file.is_none() => {
                file = Some(val.into_string()?);
            }
            Short('f') | Long("forward") => forward = true,
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
        forward,
    })
}

fn main() -> Result<(), anyhow::Error> {
    let args = parse_args()?;

    let line = Seeker::new(args.file, args.job_id)
        .forward(args.forward)
        .run()?;

    if line.len() > 0 {
        println!("{}", JobInfo::new(line.split(':').collect::<Vec<&str>>()))
    } else {
        println!("No job with ID {}", args.job_id)
    }

    Ok(())
}
