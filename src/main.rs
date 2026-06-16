use std::fs;
use std::io;

#[derive(Clone)]
struct Stock {
    code: String,
    name: String,
    price: f64,
    advice: String,
}

fn main() {
    let mut stocks = vec![
        Stock {
            code: "600519".to_string(),
            name: "贵州茅台".to_string(),
            price: 1500.0,
            advice: "长期持有".to_string(),
        },
        Stock {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 12.0,
            advice: "观察".to_string(),
        },
        Stock {
            code: "601318".to_string(),
            name: "中国平安".to_string(),
            price: 55.0,
            advice: "持有".to_string(),
        },
        Stock {
            code: "600036".to_string(),
            name: "招商银行".to_string(),
            price: 42.0,
            advice: "买入".to_string(),
        },
        Stock {
            code: "300750".to_string(),
            name: "宁德时代".to_string(),
            price: 220.0,
            advice: "买入".to_string(),
        },
    ];

    loop {
        println!();
        println!("======================");
        println!("      股票助手 V4");
        println!("======================");
        println!("1. 查询股票");
        println!("2. 查看全部股票");
        println!("3. 添加股票");
        println!("4. 删除股票");
        println!("5. 保存到文件");
        println!("6. 从文件加载");
        println!("7. 退出");
        println!("请输入选项：");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("读取失败");

        match choice.trim() {
            "1" => {
                println!("请输入股票代码：");

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("读取失败");

                let input = input.trim();

                match stocks.iter().find(|s| s.code == input) {
                    Some(stock) => {
                        println!();
                        println!("查询结果");
                        println!("股票代码：{}", stock.code);
                        println!("股票名称：{}", stock.name);
                        println!("当前价格：{}元", stock.price);
                        println!("投资建议：{}", stock.advice);
                    }
                    None => {
                        println!("未找到该股票");
                    }
                }
            }

            "2" => {
                println!();
                println!("========== 股票列表 ==========");

                for stock in &stocks {
                    println!(
                        "{} | {} | {}元 | {}",
                        stock.code,
                        stock.name,
                        stock.price,
                        stock.advice
                    );
                }
            }

            "3" => {
                println!("股票代码：");
                let mut code = String::new();
                io::stdin().read_line(&mut code).unwrap();

                println!("股票名称：");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                println!("股票价格：");
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();

                println!("投资建议：");
                let mut advice = String::new();
                io::stdin().read_line(&mut advice).unwrap();

                let stock = Stock {
                    code: code.trim().to_string(),
                    name: name.trim().to_string(),
                    price: price.trim().parse().unwrap_or(0.0),
                    advice: advice.trim().to_string(),
                };

                stocks.push(stock);

                println!("添加成功");
            }

            "4" => {
                println!("输入要删除的股票代码：");

                let mut code = String::new();

                io::stdin()
                    .read_line(&mut code)
                    .expect("读取失败");

                let code = code.trim();

                let old_len = stocks.len();

                stocks.retain(|s| s.code != code);

                if stocks.len() < old_len {
                    println!("删除成功");
                } else {
                    println!("未找到该股票");
                }
            }

            "5" => {
                let mut content = String::new();

                for stock in &stocks {
                    content.push_str(&format!(
                        "{},{},{},{}\n",
                        stock.code,
                        stock.name,
                        stock.price,
                        stock.advice
                    ));
                }

                match fs::write("stocks.txt", content) {
                    Ok(_) => println!("保存成功"),
                    Err(e) => println!("保存失败：{}", e),
                }
            }

            "6" => {
                match fs::read_to_string("stocks.txt") {
                    Ok(content) => {
                        stocks.clear();

                        for line in content.lines() {
                            let parts: Vec<&str> = line.split(',').collect();

                            if parts.len() == 4 {
                                stocks.push(Stock {
                                    code: parts[0].to_string(),
                                    name: parts[1].to_string(),
                                    price: parts[2].parse().unwrap_or(0.0),
                                    advice: parts[3].to_string(),
                                });
                            }
                        }

                        println!("加载成功");
                    }

                    Err(_) => {
                        println!("未找到 stocks.txt");
                    }
                }
            }

            "7" => {
                println!("程序已退出");
                break;
            }

            _ => {
                println!("无效选项，请重新输入");
            }
        }
    }
}