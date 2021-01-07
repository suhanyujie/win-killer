extern crate clap;
use std::{process::Command, vec};

use clap::{Arg, App, SubCommand};

fn main() {
    // 创建命令行应用。
    let matches = App::new("win-process-killer")
                        .version("0.1.0")
                        .author("suhanyujie<suhanyujie@qq.com>")
                        .about("Kill one process for windows. ")
                        .subcommands(
                            vec![
                                SubCommand::with_name("find")
                                .about("子命令：查询进程信息。")
                                .arg(
                                    Arg::with_name("str")
                                    .long("str")
                                    .value_name("string")
                                    .help("Set a string for match. ")
                                ),
                                SubCommand::with_name("kill")
                                .about("子命令：关闭进程。")
                                .arg(
                                    Arg::with_name("pid")
                                    .long("pid")
                                    .value_name("Number")
                                    .help("Set a process flag that called pid. ")
                                ),
                            ]
                        )
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Sets a custom config file")
                        )
                        .get_matches();
    
    let mut needle_str = "";
    if let Some(find_match) = matches.subcommand_matches("find") {
        needle_str = find_match.value_of("str").unwrap_or("");
    } else {
        // 没有匹配上 find 子命令，说明不是 find 子命令，无需做处理。
    }
    let mut pid_int: usize = 0;
    if let Some(kill_match) = matches.subcommand_matches("kill") {
        let pid = kill_match.value_of("pid").unwrap_or("0");
        if let Ok(tmp_val) = pid.parse::<usize>() {
            pid_int = tmp_val;
        } else {
            panic!("解析 pid 异常，请输入合法的 pid。")
        }
        if pid_int < 1 {
            panic!("pid 不合法，请输入合法的 pid。")
        }
    } else {
        // 没有匹配上 kill 子命令，说明不是 kill 子命令，无需做处理。
    }

    // netstat -ano | findstr 443
    let res = Command::new("powershell")
        .args(&["netstat"])
        .args(&["-ano | findstr "])
        .arg(needle_str)
        .output()
        // .spawn()
        .expect("exec command netstat error");
    let mut list = split_output(&res.stdout);
    // 能否做到按多字节对字节切片进行分割呢？
    // let list: Vec<_> = (&res.stdout[..])
    //     .split(|c| *c == '\n' as u8)
    //     .map(|s1 | std::str::from_utf8(s1).unwrap_or(""))
    //     .collect();
    // println!("\n{:?}", std::str::from_utf8(&res.stdout[..]));
    // todo 为了更友好，先显示一个头部，表示每一列的意义。
    let header_line = get_netstat_header_line();
    let mut total_list = vec![&*header_line];
    // total_list.append(&mut list);
    let mut data_list: Vec<Vec<&str>> = vec![
        get_netstat_header_name_list(),
    ];
    for line in list {
        // 将一行数据切割程一个个 cell
        let res1: Vec<_> = line.split(" ").filter(|s| s.trim().len() > 0).collect();
        data_list.push(res1);
        break;
    }
    println!("{:?}", data_list);
    // get_process_info(6532);
}

/// 显示进程信息 todo
/// tasklist | findstr "进程id号"
fn get_process_info(pid: u32) {
    let pid_str = pid.to_string();
    let res = Command::new("powershell")
        .args(&["tasklist", "|", "findstr", &pid_str])
        .output()
        .expect("exec tasklist error. ");
    let output_str_list = split_output(&res.stdout);
    println!("{:?}", output_str_list);
}

/// 将一行一行的输出结果转换为字符串列表
fn split_output(output_slice: &[u8]) -> Vec<&str> {
    let result: Vec<&str> = vec![];
    let output_str = std::str::from_utf8(output_slice).unwrap_or("");
    let list: Vec<_> = output_str.split("\r\n").filter(|s| s.len() > 0 ).collect();
    return list;
}

/// 显示 netstat -ano 命令后的头部
/// 头部：协议  本地地址          外部地址        状态           模板
fn get_netstat_header_line() ->String {
    let header_name_list = get_netstat_header_name_list();
    return header_name_list.join("    ");
}

fn get_netstat_header_name_list() ->Vec<&'static str> {
    let header_name_list = vec![
        "协议",
        "本地地址",
        "外部地址",
        "状态",
        "模板",
    ];
    return header_name_list;
}
