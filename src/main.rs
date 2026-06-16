use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Clone)]
struct Stock {
    code: String,
    name: String,

    buy_price: f64,
    current_price: f64,

    quantity: u32,

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
    println!("\n=============== 股票列表 ===============\n");

    if stocks.is_empty() {
        println!("暂无股票记录");
        return;
    }

    for stock in stocks {
        let cost = stock.buy_price * stock.quantity as f64;
        let value = stock.current_price * stock.quantity as f64;
        let profit = value - cost;

        println!(
            "{} | {} | 数量:{} | 成本:{:.2} | 现价:{:.2} | 盈亏:{:.2}",
            stock.code,
            stock.name,
            stock.quantity,
            stock.buy_price,
            stock.current_price,
            profit
        );
    }

    println!("\n========================================\n");
}

fn add_stock(stocks: &mut Vec<Stock>) {
    let mut code = String::new();
    let mut name = String::new();
    let mut buy_price = String::new();
    let mut current_price = String::new();
    let mut quantity = String::new();
    let mut note = String::new();

    println!("股票代码:");
    io::stdin().read_line(&mut code).unwrap();

    println!("股票名称:");
    io::stdin().read_line(&mut name).unwrap();

    println!("买入价格:");
    io::stdin().read_line(&mut buy_price).unwrap();

    println!("当前价格:");
    io::stdin().read_line(&mut current_price).unwrap();

    println!("持股数量:");
    io::stdin().read_line(&mut quantity).unwrap();

    println!("备注:");
    io::stdin().read_line(&mut note).unwrap();

    let stock = Stock {
        code: code.trim().to_string(),
        name: name.trim().to_string(),

        buy_price: buy_price.trim().parse().unwrap_or(0.0),
        current_price: current_price.trim().parse().unwrap_or(0.0),

        quantity: quantity.trim().parse().unwrap_or(0),

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
            let cost = stock.buy_price * stock.quantity as f64;
            let value = stock.current_price * stock.quantity as f64;
            let profit = value - cost;

            println!();
            println!("股票代码: {}", stock.code);
            println!("股票名称: {}", stock.name);
            println!("持股数量: {}", stock.quantity);
            println!("成本价: {:.2}", stock.buy_price);
            println!("现价: {:.2}", stock.current_price);
            println!("总成本: {:.2}", cost);
            println!("总市值: {:.2}", value);
            println!("总盈亏: {:.2}", profit);
            println!("备注: {}", stock.note);

            return;
        }
    }

    println!("未找到股票");
}

fn portfolio_stats(stocks: &Vec<Stock>) {
    let mut total_cost = 0.0;
    let mut total_value = 0.0;

    for stock in stocks {
        total_cost += stock.buy_price * stock.quantity as f64;
        total_value += stock.current_price * stock.quantity as f64;
    }

    let total_profit = total_value - total_cost;

    let profit_rate = if total_cost > 0.0 {
        total_profit / total_cost * 100.0
    } else {
        0.0
    };

    println!();
    println!("=========== 投资统计 ===========");
    println!("股票数量: {}", stocks.len());
    println!("总成本: {:.2}", total_cost);
    println!("总市值: {:.2}", total_value);
    println!("总盈亏: {:.2}", total_profit);
    println!("收益率: {:.2}%", profit_rate);
    println!("===============================");
}

fn main() {
    let mut stocks = load_from_file();

    loop {
        println!();
        println!("==========================");
        println!("      股票助手 V7");
        println!("==========================");
        println!("1. 查询股票");
        println!("2. 查看全部股票");
        println!("3. 添加股票");
        println!("4. 删除股票");
        println!("5. 保存到JSON");
        println!("6. 从JSON加载");
        println!("7. 投资统计");
        println!("8. 退出");
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

            "7" => portfolio_stats(&stocks),

            "8" => {
                println!("程序已退出");
                break;
            }

            _ => println!("无效选项"),
        }
    }
}