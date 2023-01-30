use anyhow;
use std::io::BufRead;
use std::{fmt, fs::File, io, path::Path};

use chrono::NaiveDateTime;

/// Finds a line based on ID column within file
///
/// SGE accounting files as colon separated with the job ID as the 5th item
/// we want to retrieve the whole line based on this ID
///
/// # Examples
/// ```rust
/// use std::io::Write;
/// use tempfile::NamedTempFile;
/// use sagitta::{find_line};
///
/// let mut file = NamedTempFile::new().unwrap();
/// writeln!(file, "test:one:big:thing:mem:1:20\ntest:two:job:thing:mem:2:100\ntest:three:job:thing:mem:3:234").unwrap();
///
/// let line = "test:two:job:thing:mem:2:100";
/// let file_path = String::from(file.path().to_str().unwrap());
///
/// assert_eq!(line.to_string(), find_line(file_path, 2).unwrap());
/// ```
pub fn find_line(f: String, id: i32) -> Result<String, anyhow::Error> {
    let open_file = read_file(&f)?;

    let line: String = open_file
        .lines()
        .map(|l| l.unwrap())
        .filter(|x| !x.starts_with("#"))
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
            pe_taskid: f[41].to_string(),
            maxvmem: f[42].parse().unwrap(),
            arid: f[43].parse().unwrap(),
            ar_submission_time: f[44].parse().unwrap(),
        }
    }
}

/// read a file and return a buffer
///
/// taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
///
pub fn read_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_line() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "40core-192G.q:d12s6b3.arc4.leeds.ac.uk:iss:issmcd:openmpi.40:101:sge:0:1564497817:1564497821:1564497823:0:1:2:0.132815:0.240530:18036.000000:0:0:0:0:43625:111:0:151480.000000:760:0:0:0:2061:206:ARC:defaultdepartment:ib-edr-part-1:80:0:0.373345:0.000000:0.000477:-l env=centos7,exclusive=true,h_rt=600,h_vmem=5153960755,node_type=40core-192G -pe ib-edr-part-* 80:0.000000:NONE:3702784.000000:0:0\n40core-192G.q:d12s4b2.arc4.leeds.ac.uk:iss:issmcd:openmpi.37:98:sge:0:1564497817:1564497821:1564497823:0:1:2:0.140881:0.236862:18040.000000:0:0:0:0:43632:111:0:151480.000000:760:0:0:0:2089:206:ARC:defaultdepartment:ib-edr-part-1:80:0:0.377743:0.000008:0.000477:-l env=centos7,exclusive=true,h_rt=600,h_vmem=5153960755,node_type=40core-192G -pe ib-edr-part-* 80:0.000000:NONE:3690496.000000:0:0\n40core-192G.q:d10s2b2.arc4.leeds.ac.uk:iss:issmcd:openmpi.5:66:sge:0:1564497811:1564497822:1564497823:0:1:1:0.122607:0.233584:18044.000000:0:0:0:0:43742:2:0:64.000000:752:0:0:0:1897:200:ARC:defaultdepartment:ib-edr-part-1:80:0:0.356191:0.000008:0.000477:-l env=centos7,exclusive=true,h_rt=600,h_vmem=5153960755,node_type=40core-192G -pe ib-edr-part-* 80:0.000000:NONE:3690496.000000:0:0").unwrap();

        let right_line = "40core-192G.q:d10s2b2.arc4.leeds.ac.uk:iss:issmcd:openmpi.5:66:sge:0:1564497811:1564497822:1564497823:0:1:1:0.122607:0.233584:18044.000000:0:0:0:0:43742:2:0:64.000000:752:0:0:0:1897:200:ARC:defaultdepartment:ib-edr-part-1:80:0:0.356191:0.000008:0.000477:-l env=centos7,exclusive=true,h_rt=600,h_vmem=5153960755,node_type=40core-192G -pe ib-edr-part-* 80:0.000000:NONE:3690496.000000:0:0";

        let file_string = String::from(file.path().to_str().unwrap());

        assert_eq!(right_line.to_string(), find_line(file_string, 66).unwrap());
    }

    #[test]
    fn test_skip_hash_line() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "#").unwrap();

        let right_line = "";

        let file_string = String::from(file.path().to_str().unwrap());

        assert_eq!(right_line.to_string(), find_line(file_string, 66).unwrap());
    }
}
