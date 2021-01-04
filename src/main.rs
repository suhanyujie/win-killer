extern crate clap;
use std::process::Command;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("win-process-killer")
                        .version("0.1.0")
                        .author("suhanyujie<suhanyujie@qq.com>")
                        .about("Kill one process for windows. ")
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Sets a custom config file")
                        ).arg(
                            Arg::with_name("pid")
                            .long("pid")
                            .value_name("Number")
                            .help("Set a process flag that called pid. ")
                        )
                        .get_matches();
    let pid = matches.value_of("pid").unwrap_or("0");
    let mut pid_int: usize = 0;
    if let Ok(tmp_val) = pid.parse::<usize>() {
        pid_int = tmp_val;
    } else {
        panic!("解析 pid 异常，请输入合法的 pid。")
    }
    if pid_int < 1 {
        panic!("pid 不合法，请输入合法的 pid。")
    }
    // netstat -ano | findstr 443
    let res = Command::new("powershell")
        .args(&["netstat", "-ano | findstr "])
        .arg(pid)
        .spawn()
        .expect("exec command netstat error");
}
