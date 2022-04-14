use clap::Parser;

// 定义HTTPie的CLI主入口，包含若干个子命令
// 下面///的注视时文档，clap会将其作为CLI的帮助

// A native httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
        /// HTTP 请求的 URL
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}


/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}",opts);
}
