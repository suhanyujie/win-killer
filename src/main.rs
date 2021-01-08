extern crate clap;
extern crate prettytable;

use std::{process::Command, vec};
use clap::{Arg, App, SubCommand};
use prettytable::{Cell, Row, Table};

fn main() {
    // 创建命令行应用。
    let matches = App::new("win-process-killer")
                        .version("0.1.0")
                        .author("suhanyujie<suhanyujie@qq.com>")
                        .about("Kill one process for windows. ")
                        .subcommands(
                            vec![
                                SubCommand::with_name("ps")
                                .about("子命令：查询进程信息。")
                                .arg(
                                    Arg::with_name("str")
                                    // 子命令后的参数索引声明，用索引替代参数名声明。从 1 开始计数。如值为2，则表示命令后，第2个参数为 `str` 对应的值。
                                    .index(1)
                                    .long("str")
                                    .value_name("string")
                                    .help("Set a string for match. ")
                                ),
                                SubCommand::with_name("find")
                                .about("子命令：查询tcp服务信息。")
                                .arg(
                                    Arg::with_name("str")
                                    // 子命令后的参数索引声明，用索引替代参数名声明。从 1 开始计数。如值为2，则表示命令后，第2个参数为 `str` 对应的值。
                                    .index(1)
                                    .long("str")
                                    .value_name("string")
                                    .help("Set a string for match. ")
                                ),
                                SubCommand::with_name("kill")
                                .about("子命令：关闭进程。")
                                .arg(
                                    Arg::with_name("pid")
                                    .index(1)
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
    // 针对不同命令的处理逻辑
    if let Some(find_match) = matches.subcommand_matches("find") {
        let needle_str = find_match.value_of("str").unwrap_or("");
        handle_find(needle_str);
        return;
    } else {
        // 没有匹配上 find 子命令，说明不是 find 子命令，无需做处理。
    }
    if let Some(find_match) = matches.subcommand_matches("ps") {
        let needle_str = find_match.value_of("str").unwrap_or("");
        handle_ps(needle_str);
        return;
    }
    if let Some(kill_match) = matches.subcommand_matches("kill") {
        let mut pid_int: usize = 0;
        let pid = kill_match.value_of("pid").unwrap_or("0");
        if let Ok(tmp_val) = pid.parse::<usize>() {
            pid_int = tmp_val;
        } else {
            panic!("解析 pid 异常，请输入合法的 pid。")
        }
        if pid_int < 1 {
            panic!("pid 不合法，请输入合法的 pid。")
        }
        handle_kill_one(pid_int);
        return
    }
}

/// 关闭进程
/// taskkill /f /t /im "{pid}"
fn handle_kill_one(pid: usize) {
    let res = Command::new("powershell")
        .args(&["taskkill"])
        .args(&["/f /t /im"])
        .arg(pid.to_string())
        .output()
        .expect("exec command taskkill error");
    let list = split_output(&res.stdout);
    // 查询命令执行结果
    let res = Command::new("powershell")
        .args(&["$?"])
        .output()
        .expect("exec command $? error");
    let output_lines = split_output(&res.stdout);
    println!("执行结果：{:?}", output_lines);
}

fn handle_find(needle_str: &str) {
    // netstat -ano | findstr 443
    let res = Command::new("powershell")
        .args(&["netstat"])
        .args(&["-ano | findstr "])
        .arg(needle_str)
        .output()
        // .spawn()
        .expect("exec command netstat error");
    let list = split_output(&res.stdout);
    // 能否做到按多字节对字节切片进行分割呢？
    // let list: Vec<_> = (&res.stdout[..])
    //     .split(|c| *c == '\n' as u8)
    //     .map(|s1 | std::str::from_utf8(s1).unwrap_or(""))
    //     .collect();
    // println!("\n{:?}", std::str::from_utf8(&res.stdout[..]));
    // todo 为了更友好，先显示一个头部，表示每一列的意义。
    let mut data_list: Vec<Vec<&str>> = vec![];
    for line in list {
        // 将一行数据切割程一个个 cell
        let res1: Vec<_> = line.split(" ").filter(|s| s.trim().len() > 0).collect();
        data_list.push(res1);
    }
    render(get_netstat_header_name_list(), data_list);
}

// 查询进程列表
fn handle_ps(needle_str: &str) {
    // tasklist | findstr {someStr}
    let res = Command::new("powershell")
        .args(&["tasklist"])
        .args(&["| findstr "])
        .arg(needle_str)
        .output()
        .expect("exec command tasklist error");
    let list = split_output(&res.stdout);
    let mut data_list: Vec<Vec<&str>> = vec![];
    for line in list {
        // 将一行数据切割程一个个 cell
        let res1: Vec<_> = line.split(" ").filter(|s| s.trim().len() > 0).collect();
        data_list.push(res1);
    }
    render(vec![
        "程序名",
        "进程id",
        "会话名",
        "会话id",
        "内存占用",
    ], data_list);
}

/// 展示列表信息
fn render(header: Vec<&str>, data_list: Vec<Vec<&str>>) {
    let mut table = Table::new();
    table.add_row(trans_arr_into_row(header));
    for row_data in data_list {
        table.add_row(trans_arr_into_row(row_data));
    }
    table.printstd();
}

fn trans_arr_into_row(arr: Vec<&str>) -> Row {
    let cell_arr: Vec<Cell> = arr.iter().map(|s| Cell::new(*s)).collect();
    return Row::new(cell_arr);
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
/// 头部：协议  本地地址          外部地址        状态           进程id/模板
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
        "进程id/模板",
    ];
    return header_name_list;
}

fn get_win_tasklist_header_name_list() ->Vec<&'static str> {
    let header_name_list = vec![
        "协议",
        "本地地址",
        "外部地址",
        "状态",
        "进程id/模板",
    ];
    return header_name_list;
}
