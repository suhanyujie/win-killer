# win command like linux
windows 下的 Linux 风格的命令行工具。

## Support
* ps 查看进程: `wt ps Ding`
* kill 关闭进程: `wt kill 23636`

## preview
### help 
* `wt -h`

```
win-command-tools 0.1.0
suhanyujie<suhanyujie@qq.com>
Command tools for windows.

USAGE:
    win-kill.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    find    子命令：查询tcp服务信息。
    help    Prints this message or the help of the given subcommand(s)
    kill    子命令：关闭进程。
    ps      子命令：查询进程信息。
```

### 查看 tcp 服务 find 
* `wt find 443`

```
+------+---------------------+---------------------+-------------+-------------+
| 协议 | 本地地址            | 外部地址            | 状态        | 进程id/模板 |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 0.0.0.0:443         | 0.0.0.0:0           | LISTENING   | 5764        |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 192.168.88.13:50133 | 27.19.249.224:443   | ESTABLISHED | 5904        |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 192.168.88.13:50184 | 203.119.169.194:443 | ESTABLISHED | 6696        |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 192.168.88.13:50185 | 220.243.141.8:443   | ESTABLISHED | 5904        |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 192.168.88.13:50215 | 40.119.211.203:443  | ESTABLISHED | 3756        |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 192.168.88.13:50259 | 47.102.162.67:443   | ESTABLISHED | 11752       |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 192.168.88.13:50364 | 220.243.141.8:443   | ESTABLISHED | 9372        |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 192.168.88.13:50405 | 122.14.230.129:443  | ESTABLISHED | 9372        |
+------+---------------------+---------------------+-------------+-------------+
| TCP  | 192.168.88.13:54036 | 123.58.182.210:443  | CLOSE_WAIT  | 11124       |
+------+---------------------+---------------------+-------------+-------------+
| ...
+------+---------------------+---------------------+-------------+-------------+
```

### 查询进程 ps
* `wt ps main`

```
+----------+--------+---------+--------+----------+---+
| 程序名   | 进程id | 会话名  | 会话id | 内存占用 |   |
+----------+--------+---------+--------+----------+---+
| main.exe | 2568   | Console | 3      | 15,892   | K |
+----------+--------+---------+--------+----------+---+
| main.exe | 9616   | Console | 3      | 18,756   | K |
+----------+--------+---------+--------+----------+---+
| main.exe | 18072  | Console | 3      | 21,624   | K |
+----------+--------+---------+--------+----------+---+
| ...
+----------+--------+---------+--------+----------+---+
```

### 关闭进程 kill
* 2568 只是一个示例的进程 id: `wt kill 2568`

```
执行结果：["True"]
```

## reference
* win 关闭进程 https://jingyan.baidu.com/article/fdffd1f89a0c8af3e98ca10e.html
* clap https://docs.rs/clap/2.33.3/clap/
* powershell 手册 https://www.yiibai.com/powershell
* rust 表格打印 https://crates.io/crates/prettytable-rs
