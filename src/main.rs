use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Clone)]
struct Stock {
    code: String,
    name: String,
    price: f64,
    note: String,
}

fn save_to_file(stocks: &Vec<Stock>) {
    let json = serde_json::to_string_pretty(stocks).unwrap();
    fs::write("stocks.json", json).unwrap();
    println!("保存成功！");
}

fn load_from_file() -> Vec<Stock> {
    match fs::read_to_string("stocks.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn show_all(stocks: &Vec<Stock>) {
    println!("\n================ 股票列表 ================");

    for stock in stocks {
        println!(
            "{} | {} | {:.2}元 | {}",
            stock.code,
            stock.name,
            stock.price,
            stock.note
        );
    }

    println!("=========================================\n");
}

fn add_stock(stocks: &mut Vec<Stock>) {
    let mut code = String::new();
    let mut name = String::new();
    let mut price = String::new();
    let mut note = String::new();

    println!("股票代码:");
    io::stdin().read_line(&mut code).unwrap();

    println!("股票名称:");
    io::stdin().read_line(&mut name).unwrap();

    println!("买入价格:");
    io::stdin().read_line(&mut price).unwrap();

    println!("备注:");
    io::stdin().read_line(&mut note).unwrap();

    let stock = Stock {
        code: code.trim().to_string(),
        name: name.trim().to_string(),
        price: price.trim().parse().unwrap_or(0.0),
        note: note.trim().to_string(),
    };

    stocks.push(stock);

    println!("添加成功！");
}

fn delete_stock(stocks: &mut Vec<Stock>) {
    let mut code = String::new();

    println!("输入要删除的股票代码:");

    io::stdin().read_line(&mut code).unwrap();

    let code = code.trim();

    stocks.retain(|s| s.code != code);

    println!("删除完成！");
}

fn search_stock(stocks: &Vec<Stock>) {
    let mut keyword = String::new();

    println!("输入股票代码:");

    io::stdin().read_line(&mut keyword).unwrap();

    let keyword = keyword.trim();

    for stock in stocks {
        if stock.code == keyword {
            println!(
                "{} | {} | {:.2}元 | {}",
                stock.code,
                stock.name,
                stock.price,
                stock.note
            );
            return;
        }
    }

    println!("未找到股票");
}

fn main() {
    let mut stocks = load_from_file();

    loop {
        println!();
        println!("==========================");
        println!("      股票助手 V6");
        println!("==========================");
        println!("1. 查询股票");
        println!("2. 查看全部股票");
        println!("3. 添加股票");
        println!("4. 删除股票");
        println!("5. 保存到JSON");
        println!("6. 从JSON加载");
        println!("7. 退出");
        println!("==========================");

        let mut choice = String::new();

        println!("请输入选项:");

        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => search_stock(&stocks),

            "2" => show_all(&stocks),

            "3" => add_stock(&mut stocks),

            "4" => delete_stock(&mut stocks),

            "5" => save_to_file(&stocks),

            "6" => {
                stocks = load_from_file();
                println!("加载成功！");
            }

            "7" => {
                println!("程序已退出");
                break;
            }

            _ => println!("无效选项"),
        }
    }
}