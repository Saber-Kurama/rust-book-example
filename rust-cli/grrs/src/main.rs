/*
 * @Author: saber
 * @Date: 2021-12-23 21:43:14
 * @LastEditTime: 2021-12-24 14:23:20
 * @LastEditors: saber
 * @Description:
 */
use std::path::PathBuf;
use structopt::StructOpt;
use anyhow:: { Result, Context };

// /// 文档注释，Search for a pattern in a file and display the lines that contain it.
// #[derive(Debug, StructOpt)]
// #[structopt(name = "grrs", about = "这是一个例子")]
// struct Opt {
//     #[structopt(short, long)]
//     debug: bool,
// }

/// 文档注释Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
// #[derive(Debug)]
// struct CustomError(String);

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

// fn main() -> Result<()> {
//     // let args = Cli::from_args();
//     // println!("Hello, world! {:?}", args);
//     // let content = std::fs::read_to_string(&args.path).expect("could not read file");
//     // for line in content.lines() {
//     //     if line.contains(&args.pattern) {
//     //         println!("{}", line);
//     //     }
//     // }

//     // let result = std::fs::read_to_string("/Users/saber/coding/mygithub/rust-book-example/rust-cli/grrs/src/test.txt");
//     // match result {
//     //     Ok(content) => { println!("{}", content)},
//     //     Err(error) => { return Err(error.into()) }
//     // };
//     // Ok(())

//     // let result = std::fs::read_to_string("/Users/saber/coding/mygithub/rust-book-example/rust-cli/grrs/src/test1.txt")?;
//     // println!("file content {}", result);
//     // Ok(())

//     // 自定义的错误
//     let path = "/Users/saber/coding/mygithub/rust-book-example/rust-cli/grrs/src/test.txt";
//     // let result = std::fs::read_to_string(path).map_err(|err| CustomError(format!("{}: {}", path, err )))?;
//     // println!("file content {}", result);
//     // Ok(())    
//     let result = std::fs::read_to_string(path).with_context(|| format!("could node read file: {}", path))?;
//     println!("file content {}", result);
//     eprintln!("This is an error! :(");
//     Ok(())
// }

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}