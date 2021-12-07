// daily_visitors_mod.rs

struct LogLine {
    ip: String,
    date: String,
    #[allow(dead_code)]
    url: String,
    code: i32,
    #[allow(dead_code)]
    length: i32,
}

#[derive(Debug)]
struct GroupRequestsByDay {
    date: String,
    count_visits: i32,
}

#[derive(Debug)]
struct GroupVisitorsByDay {
    date: String,
    count_visitors: i32,
}

pub fn read_nginx_log_and_fill_daily_visitors() -> Vec<crate::reserved_folder_mod::DailyVisitors> {
    // read and parse the nginx log files and export into csv for excel
    let base_path = "/var/log/nginx/access.log";
    // access.log, access.log.1, access.log.2.gz,...
    let mut num = 0;
    let mut file_path = base_path.to_string();
    let mut log_lines: Vec<LogLine> = vec![];
    loop {
        // region: read text or decompress gz
        let mut text = String::new();
        if file_path.ends_with(".gz") {
            let tar_gz = std::fs::File::open(file_path).unwrap();
            let mut tar = flate2::read::GzDecoder::new(tar_gz);
            std::io::Read::read_to_string(&mut tar, &mut text).unwrap();
        } else {
            text = std::fs::read_to_string(file_path).unwrap();
        }
        // endregion: read text or decompress gz

        // region: capture groups with regex
        let re = regex::Regex::new(
            r#"([0-9.]+) - - \[(.{20}) \+0000] "GET (.+) HTTP/2.0" ([0-9]{3}) ([0-9]+)"#,
        )
        .unwrap();
        // 51.222.253.19 - - [06/Dec/2021:23:21:24 +0000] "GET /rust-reviews/crate/regex-automata/crate/N HTTP/2.0" 200 1690 "-" "Mozilla/5.0 (compatible; xxxBot/7.0; +http://ahrefs.com/robot/)"
        for line in text.lines() {
            for caps in re.captures_iter(line) {
                // stupidest date format ever. Transform into 2021-12-06 23:21:24
                let date = caps.get(2).unwrap().as_str().to_string();
                let month = match &date[3..6] {
                    "Jan" => "01",
                    "Feb" => "02",
                    "Mar" => "03",
                    "Apr" => "04",
                    "May" => "05",
                    "Jun" => "06",
                    "Jul" => "07",
                    "Aug" => "08",
                    "Sep" => "09",
                    "Oct" => "10",
                    "Nov" => "11",
                    "Dec" => "12",
                    _ => "",
                };
                let date = format!(
                    "{}-{}-{} {}",
                    &date[7..11],
                    month,
                    &date[0..2],
                    &date[12..20]
                );
                let url = caps.get(3).unwrap().as_str().to_string();
                // limit url for only /rust-reviews/
                if url.starts_with("/rust-reviews/") {
                    let code = caps.get(4).unwrap().as_str().parse().unwrap();
                    // only code 200 are true requests and not robots or malware
                    if code == 200 {
                        log_lines.push(LogLine {
                            ip: caps.get(1).unwrap().as_str().to_string(),
                            date,
                            url,
                            code,
                            length: caps.get(4).unwrap().as_str().parse().unwrap(),
                        });
                    }
                }
            }
        }
        // endregion: capture groups with regex
        // region: loop condition
        num += 1;
        file_path = format!("{}.{}", base_path, num);
        if !std::path::Path::new(&file_path).exists() {
            file_path = format!("{}.{}.gz", base_path, num);
            if !std::path::Path::new(&file_path).exists() {
                break;
            }
        }
        // endregion: loop condition
    }

    let v1 = group_visits_by_day(&mut log_lines);
    let v2 = group_visitors_by_day(&mut log_lines);
    // zip 2 iterators together
    let v3 = v1.iter().zip(v2.iter());
    let mut vec: Vec<crate::reserved_folder_mod::DailyVisitors> = vec![];
    for x in v3 {
        vec.push(crate::reserved_folder_mod::DailyVisitors {
            date: x.0.date.to_string(),
            visitors: x.1.count_visitors.to_string(),
            requests: x.0.count_visits.to_string(),
        });
    }
    // return
    vec
}

fn group_visits_by_day(log_lines: &mut Vec<LogLine>) -> Vec<GroupRequestsByDay> {
    let mut vec_gr: Vec<GroupRequestsByDay> = vec![];
    // sort by day
    log_lines.sort_by(|a, b| a.date[..10].cmp(&b.date[..10]));
    let mut old_day = "".to_string();
    let mut count_visits = 0;
    for line in log_lines.iter() {
        if line.code == 200 {
            let new_day = line.date[..10].to_string();
            if new_day != old_day {
                if !old_day.is_empty() {
                    vec_gr.push(GroupRequestsByDay {
                        date: old_day.to_string(),
                        count_visits,
                    });
                }
                old_day = new_day;
                count_visits = 1;
            } else {
                count_visits += 1;
            }
        }
    }
    vec_gr
}

fn group_visitors_by_day(log_lines: &mut Vec<LogLine>) -> Vec<GroupVisitorsByDay> {
    let mut vec_gr: Vec<GroupVisitorsByDay> = vec![];
    // sort by day + ip
    log_lines.sort_by(|a, b| {
        format!("{}{}", &a.date[..10], a.ip).cmp(&format!("{}{}", &b.date[..10], b.ip))
    });
    let mut old_day = "".to_string();
    let mut old_ip = "".to_string();
    let mut count_visitors = 0;
    for line in log_lines.iter() {
        if line.code == 200 {
            let new_day = line.date[..10].to_string();
            let new_ip = line.ip.to_string();
            if new_day != old_day {
                if !old_day.is_empty() {
                    vec_gr.push(GroupVisitorsByDay {
                        date: old_day.to_string(),
                        count_visitors,
                    });
                }
                old_day = new_day;
                old_ip = new_ip.to_string();
                count_visitors = 1;
            } else {
                if new_ip != old_ip {
                    count_visitors += 1;
                    old_ip = new_ip;
                }
            }
        }
    }
    vec_gr
}
