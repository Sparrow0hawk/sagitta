use anyhow;
use std::io::BufRead;
use std::{fmt, fs::File, io, path::Path};

use chrono::NaiveDateTime;

pub fn find_line(f: String, id: i32) -> Result<String, anyhow::Error> {
    let open_file = read_file(&f)?;

    let line: String = open_file
        .lines()
        .skip(4)
        .map(|l| l.unwrap())
        .filter(|x| x.split(":").nth(5).unwrap().parse::<i32>().unwrap().eq(&id))
        .collect();

    Ok(line)
}

#[derive(Debug)]
pub struct JobInfo {
    qname: String,
    hostname: String,
    group: String,
    owner: String,
    job_name: String,
    job_number: i32,
    account: String,
    priority: String,
    submission_time: NaiveDateTime,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    failed: String,
    exit_status: String,
    ru_wallclock: f64,
    ru_utime: f64,
    ru_stime: f64,
    ru_maxrss: f64,
    ru_ixrss: f64,
    ru_ismrss: f64,
    ru_idrss: f64,
    ru_isrss: f64,
    ru_minflt: f64,
    ru_majflt: f64,
    ru_nswap: f64,
    ru_inblock: f64,
    ru_oublock: f64,
    ru_msgsnd: f64,
    ru_msgrcv: f64,
    ru_nsignals: f64,
    ru_nvcsw: f64,
    ru_nivcsw: f64,
    project: String,
    department: String,
    granted_pe: String,
    slots: i32,
    task_number: i32,
    cpu: f64,
    mem: f64,
    io: f64,
    category: String,
    iow: f64,
    pe_taskid: String,
    maxvmem: f64,
    arid: f64,
    ar_submission_time: i32,
}

impl fmt::Display for JobInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"
Queue name:                      {}
Hostname:                        {}
Group:                           {}
Job Owner:                       {}
Job Name:                        {}
Job Number:                      {}
Account:                         {}
Job priority:                    {}
Submission Time:                 {}
Job Start Time:                  {}
Job End Time:                    {}
Failed status:                   {}
Exit status:                     {}
Wallclock Time:                  {}
User CPU Time:                   {}
System CPU Time:                 {}
Maximum resident shared set:     {}
Integral shared memory size:     {}
ru_ismrss:                       {}
Integral unshared data size:     {}
Integral unshared stack size:    {}
Page reclaims:                   {}
Page faults:                     {}
Swaps:                           {}
Block input operations:          {}
Block output operations:         {}
IPC messages sent:               {}
IPC messages received:           {}
Signals received:                {}
Voluntary context switches:      {}
Involuntary context switches:    {}
Project:                         {}
Department:                      {}
Granted parallel environment:    {}
Slots (cores):                   {}
Task number:                     {}
CPU Time usage (s):              {}
Integral memory use              {}
(Gbytes/cpu sec)
Input/Output operations:         {}
Job Category:                    {}
Input/Output wait time (s):      {}
Identifier for parallel job:     {}
Max Virtual memory size (bytes): {}
Advanced reservation ID:         {}
Advanced reservation             {}
submission time:                         "#,
            self.qname,
            self.hostname,
            self.group,
            self.owner,
            self.job_name,
            self.job_number,
            self.account,
            self.priority,
            self.submission_time,
            self.start_time,
            self.end_time,
            self.failed,
            self.exit_status,
            self.ru_wallclock,
            self.ru_utime,
            self.ru_stime,
            self.ru_maxrss,
            self.ru_ixrss,
            self.ru_ismrss,
            self.ru_idrss,
            self.ru_isrss,
            self.ru_minflt,
            self.ru_majflt,
            self.ru_nswap,
            self.ru_inblock,
            self.ru_oublock,
            self.ru_msgsnd,
            self.ru_msgrcv,
            self.ru_nsignals,
            self.ru_nvcsw,
            self.ru_nivcsw,
            self.project,
            self.department,
            self.granted_pe,
            self.slots,
            self.task_number,
            self.cpu,
            self.mem,
            self.io,
            self.category,
            self.iow,
            self.pe_taskid,
            self.maxvmem,
            self.arid,
            self.ar_submission_time,
        )
    }
}

impl JobInfo {
    pub fn new(f: Vec<&str>) -> JobInfo {
        JobInfo {
            qname: f[0].to_string(),
            hostname: f[1].to_string(),
            group: f[2].parse().unwrap(),
            owner: f[3].to_string(),
            job_name: f[4].to_string(),
            job_number: f[5].parse().unwrap(),
            account: f[6].to_string(),
            priority: f[7].parse().unwrap(),
            submission_time: NaiveDateTime::from_timestamp_opt(f[8].parse::<i64>().unwrap(), 0)
                .unwrap(),
            start_time: NaiveDateTime::from_timestamp_opt(f[9].parse::<i64>().unwrap(), 0).unwrap(),
            end_time: NaiveDateTime::from_timestamp_opt(f[10].parse::<i64>().unwrap(), 0).unwrap(),
            failed: f[11].to_string(),
            exit_status: f[12].to_string(),
            ru_wallclock: f[13].parse().unwrap(),
            ru_utime: f[14].parse().unwrap(),
            ru_stime: f[15].parse().unwrap(),
            ru_maxrss: f[16].parse().unwrap(),
            ru_ixrss: f[17].parse().unwrap(),
            ru_ismrss: f[18].parse().unwrap(),
            ru_idrss: f[19].parse().unwrap(),
            ru_isrss: f[20].parse().unwrap(),
            ru_minflt: f[21].parse().unwrap(),
            ru_majflt: f[22].parse().unwrap(),
            ru_nswap: f[23].parse().unwrap(),
            ru_inblock: f[24].parse().unwrap(),
            ru_oublock: f[25].parse().unwrap(),
            ru_msgsnd: f[26].parse().unwrap(),
            ru_msgrcv: f[27].parse().unwrap(),
            ru_nsignals: f[28].parse().unwrap(),
            ru_nvcsw: f[29].parse().unwrap(),
            ru_nivcsw: f[30].parse().unwrap(),
            project: f[31].to_string(),
            department: f[32].to_string(),
            granted_pe: f[33].to_string(),
            slots: f[34].parse().unwrap(),
            task_number: f[35].parse().unwrap(),
            cpu: f[36].parse().unwrap(),
            mem: f[37].parse().unwrap(),
            io: f[38].parse().unwrap(),
            category: f[39].to_string(),
            iow: f[40].parse().unwrap(),
            pe_taskid: f[42].to_string(),
            maxvmem: f[42].parse().unwrap(),
            arid: f[43].parse().unwrap(),
            ar_submission_time: f[44].parse().unwrap(),
        }
    }
}

pub fn read_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
