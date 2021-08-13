use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("引数の個数が正しくありません");
        }
        2 => {
            let number: i32 = match args[1].parse() {
                Ok(x) => x,
                Err(_) => {
                    eprintln!("not number!");
                    return;
                }
            };
            println!(".intel_syntax noprefix");
            println!(".globl main");
            println!("main:");
            println!("  mov rax, {0}", number);
            println!("  ret");
        }

        _ => {}
    }
}
