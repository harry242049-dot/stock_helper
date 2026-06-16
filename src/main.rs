use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io;

const API_KEY: &str = "I8JPJQ1CEPYLS235";

#[derive(Serialize, Deserialize, Clone)]
struct Stock {
    code: String,
    name: String,
    buy_price: f64,
    current_price: f64,
    quantity: u32,
    note: String,
}

async fn fetch_price(symbol: &str) -> Option<f64> {
    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        symbol,
        API_KEY
    );

    let response = reqwest::get(&url).await.ok()?;

    let json: Value = response.json().await.ok()?;

    let price = json["Global Quote"]["05. price"]
        .as_str()?
        .parse::<f64>()
        .ok()?;

    Some(price)
}

async fn update_one_stock(stocks: &mut Vec<Stock>) {
    let mut code = String::new();

    println!("输入股票代码:");

    io::stdin().read_line(&mut code).unwrap();

    let code = code.trim().to_uppercase();

    for stock in stocks {
        if stock.code.to_uppercase() == code {
            println!("联网获取价格中...");

            match fetch_price(&code).await {
                Some(price) => {
                    stock.current_price = price;

                    println!(
                        "{} 最新价格更新成功：{:.2}",
                        stock.code,
                        price
                    );
                }

                None => {
                    println!("获取价格失败");
                }
            }

            return;
        }
    }

    println!("未找到股票");
}

async fn update_all_stocks(stocks: &mut Vec<Stock>) {
    if stocks.is_empty() {
        println!("暂无股票");
        return;
    }

    println!("开始联网更新...");

    for stock in stocks {
        match fetch_price(&stock.code).await {
            Some(price) => {
                stock.current_price = price;

                println!(
                    "{} -> {:.2}",
                    stock.code,
                    stock.current_price
                );
            }

            None => {
                println!("{} 更新失败", stock.code);
            }
        }
    }

    println!("全部更新完成！");
}

fn save_to_file(stocks: &Vec<Stock>) {
    let json = serde_json::to_string_pretty(stocks).unwrap();

    fs::write("stocks.json", json).unwrap();

    println!("保存成功！");
}

fn load_from_file() -> Vec<Stock> {
    match fs::read_to_string("stocks.json") {
        Ok(content) => {
            serde_json::from_str(&content)
                .unwrap_or_default()
        }

        Err(_) => Vec::new(),
    }
}
fn show_all(stocks: &Vec<Stock>) {
    println!("\n=========== 股票列表 ===========");

    if stocks.is_empty() {
        println!("暂无股票");
        return;
    }

    for stock in stocks {
        let profit =
            (stock.current_price - stock.buy_price)
                * stock.quantity as f64;

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

    println!("===============================");
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

    stocks.push(Stock {
        code: code.trim().to_uppercase(),
        name: name.trim().to_string(),
        buy_price: buy_price.trim().parse().unwrap_or(0.0),
        current_price: current_price.trim().parse().unwrap_or(0.0),
        quantity: quantity.trim().parse().unwrap_or(0),
        note: note.trim().to_string(),
    });

    println!("添加成功！");
}

fn delete_stock(stocks: &mut Vec<Stock>) {
    let mut code = String::new();

    println!("输入股票代码:");

    io::stdin().read_line(&mut code).unwrap();

    let code = code.trim().to_uppercase();

    stocks.retain(|s| s.code.to_uppercase() != code);

    println!("删除完成！");
}

fn search_stock(stocks: &Vec<Stock>) {
    let mut code = String::new();

    println!("输入股票代码:");

    io::stdin().read_line(&mut code).unwrap();

    let code = code.trim().to_uppercase();

    for stock in stocks {
        if stock.code.to_uppercase() == code {
            let cost =
                stock.buy_price * stock.quantity as f64;

            let value =
                stock.current_price * stock.quantity as f64;

            let profit = value - cost;

            println!();
            println!("股票代码: {}", stock.code);
            println!("股票名称: {}", stock.name);
            println!("持股数量: {}", stock.quantity);
            println!("买入价: {:.2}", stock.buy_price);
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

fn update_price(stocks: &mut Vec<Stock>) {
    let mut code = String::new();
    let mut price = String::new();

    println!("股票代码:");
    io::stdin().read_line(&mut code).unwrap();

    println!("新价格:");
    io::stdin().read_line(&mut price).unwrap();

    let code = code.trim().to_uppercase();
    let price: f64 = price.trim().parse().unwrap_or(0.0);

    for stock in stocks {
        if stock.code.to_uppercase() == code {
            stock.current_price = price;
            println!("修改成功！");
            return;
        }
    }

    println!("未找到股票");
}

fn update_quantity(stocks: &mut Vec<Stock>) {
    let mut code = String::new();
    let mut qty = String::new();

    println!("股票代码:");
    io::stdin().read_line(&mut code).unwrap();

    println!("新数量:");
    io::stdin().read_line(&mut qty).unwrap();

    let code = code.trim().to_uppercase();
    let qty: u32 = qty.trim().parse().unwrap_or(0);

    for stock in stocks {
        if stock.code.to_uppercase() == code {
            stock.quantity = qty;
            println!("修改成功！");
            return;
        }
    }

    println!("未找到股票");
}

fn portfolio_stats(stocks: &Vec<Stock>) {
    let mut cost = 0.0;
    let mut value = 0.0;

    for stock in stocks {
        cost += stock.buy_price * stock.quantity as f64;
        value += stock.current_price * stock.quantity as f64;
    }

    let profit = value - cost;

    let rate = if cost > 0.0 {
        profit / cost * 100.0
    } else {
        0.0
    };

    println!("\n=========== 投资统计 ===========");
    println!("股票数量: {}", stocks.len());
    println!("总成本: {:.2}", cost);
    println!("总市值: {:.2}", value);
    println!("总盈亏: {:.2}", profit);
    println!("收益率: {:.2}%", rate);
    println!("===============================");
}

fn ranking(stocks: &Vec<Stock>) {
    let mut list = stocks.clone();

    list.sort_by(|a, b| {
        let pa =
            (a.current_price - a.buy_price)
                * a.quantity as f64;

        let pb =
            (b.current_price - b.buy_price)
                * b.quantity as f64;

        pb.partial_cmp(&pa).unwrap()
    });

    println!("\n=========== 盈亏排行 ===========");

    for stock in list {
        let profit =
            (stock.current_price - stock.buy_price)
                * stock.quantity as f64;

        println!(
            "{} | {} | 盈亏 {:.2}",
            stock.code,
            stock.name,
            profit
        );
    }

    println!("===============================");
}
#[tokio::main]
async fn main() {
    let mut stocks = load_from_file();

    loop {
        println!();
        println!("========== 股票助手 V10 ==========");
        println!("1. 查询股票");
        println!("2. 查看全部股票");
        println!("3. 添加股票");
        println!("4. 删除股票");
        println!("5. 保存到JSON");
        println!("6. 从JSON加载");
        println!("7. 投资统计");
        println!("8. 修改当前价格");
        println!("9. 修改持股数量");
        println!("10. 联网更新单只股票");
        println!("11. 联网更新全部股票");
        println!("12. 盈亏排行榜");
        println!("13. 自动保存");
        println!("14. 退出");
        println!("==============================");

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

            "8" => update_price(&mut stocks),

            "9" => update_quantity(&mut stocks),

            "10" => {
                update_one_stock(&mut stocks).await;
            }

            "11" => {
                update_all_stocks(&mut stocks).await;
            }

            "12" => ranking(&stocks),

            "13" => {
                save_to_file(&stocks);
                println!("自动保存完成！");
            }

            "14" => {
                println!("程序已退出");
                break;
            }

            _ => println!("无效选项"),
        }
    }
}