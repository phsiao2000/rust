pub mod code2;
use crate::code2::inc_and_tax;

use std::{collections::HashMap, error::Error, iter::FromIterator, result::Result};

#[derive(Clone, Debug)]
struct Order<'a> {
    name: &'a str,
    amount: u16,
}
fn order_sweets<T: Taxable>(orders: &Vec<Order>, prices: &HashMap<&str, T>) -> Option<f64> {
    let mut price = 0.0;
    reverse(&mut orders.clone());
    for order in orders {
        price += (prices.get(&order.name))
            .expect(&("bad name: ".to_string() + order.name))
            .with_tax();
    }
    Some(price)
}

fn ordered_items<T: Taxable>(orders: &Vec<Order>, prices: &HashMap<&str, T>) -> Option<f64> {
    let mut total: f64 = 0.0;
    let mut found_price: f64;
    for order in orders {
        found_price = (prices.get(&order.name))?.as64() * order.amount as f64;
        total += found_price;
        println!(
            "{} {} {:?}",
            order.name,
            order.amount,
            prices.get(&order.name)?.as64()
        );
        //.expect(&("bad name: ".to_string() + order.name));
    }
    println!(
        "taxable is ${}, tax rate is {}%, with tax is {} ",
        total,
        total.tax_rate(),
        total.with_tax()
    );
    Some(total.with_tax())
}
trait Taxable {
    fn with_tax(&self) -> f64;
    fn as64(&self) -> f64;
    fn tax_rate(&self) -> f64;
}
impl Taxable for f64 {
    fn as64(&self) -> f64 {
        *self
    }
    fn with_tax(&self) -> f64 {
        self * (1.0 + self.tax_rate() / 100.0)
    }

    fn tax_rate(&self) -> f64 {
        6.5
    }
}

fn reverse<T>(vector: &mut Vec<T>) {
    let mut new_vector = Vec::new();

    while let Some(last) = vector.pop() {
        new_vector.push(last);
    }

    *vector = new_vector;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mushrooms = vec!["Porcini", "Oyster", "Shiitake"];
    let m = Vec::from_iter(mushrooms.iter().map(|m| m.to_uppercase()));
    println!("mushrooms: {:?}", m);

    let prices = HashMap::from([
        ("Cake", 1.2),
        ("Donut", 1.0),
        ("Apple Pie", 16.25),
        ("Birthday Cake", 12.65),
        ("Whole Wheat Bread", 5.25),
        ("Artisan White Bread", 4.25),
    ]);

    for (pastry, price) in prices.iter() {
        println!("{:<20} $ {:5.2}", pastry, price);
    }
    let order = vec![
        Order {
            name: "Cake",
            amount: 1,
        },
        Order {
            name: "Donut",
            amount: 12,
        },
        Order {
            name: "Apple Pie",
            amount: 2,
        },
        Order {
            name: "Whole Wheat Bread",
            amount: 3,
        },
    ];
    let all_prices: Vec<_> = prices
        .clone()
        .into_iter()
        .filter(|&(name, _)| name != "Cake")
        .map(|(_, price)| price)
        .collect();

    let vec_prices: Vec<f64> = prices.clone().iter().map(|(_, price)| *price).collect();

    let sum_prices = prices
        .clone()
        .iter()
        .map(|(_, price)| *price)
        .reduce(|acc, e| acc + e);

    let l_pastries = prices
        .clone()
        .iter()
        .fold("END".to_string(), |acc, m| format!("{} -> {acc}", m.0));

    let l_prices = prices
        .clone()
        .iter()
        .map(|(_, price)| *price)
        .fold(0.0, |acc, m| acc + m);

    println!("l_prices       {:5.2}", l_prices);
    println!("l_pastries     {}", l_pastries);
    println!("price filtered {:>05.2?}", all_prices);
    println!("vec prices     {:>5.2?}", vec_prices);
    println!("sum_prices     ${:5.2}", sum_prices.unwrap());

    let total_price = order_sweets(&order, &prices);
    println!("total_price    ${:5.2?}", total_price);
    println!("ordered items {:?}", ordered_items(&order, &prices));
    println!("prices         {:?}", prices);
    show_order(order);

    let mut sum_total = 0.0;
    println!("begin summing");
    for x in 0..100_000_000 {
        sum_total += x as f64;
    }
    println!("{} is sum_total", sum_total);

    use std::thread;
    use std::time::Duration;

    const SLEEP1: u8 = 1;
    const SLEEP2: u8 = 2;
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(SLEEP1 as u64));
        println!("Hello from the spawned thread (after {SLEEP1} second sleep)!");
    });
    println!("Spawned a thread");

    thread::sleep(Duration::from_secs(SLEEP2 as u64));
    println!("Hello from the main thread (after {SLEEP2} seconds)!");

    println!("{}", inc_and_tax(95.0));
    Ok(())
}

fn show_order(order: Vec<Order>) {
    println!("------- Order ----------");
    println!("ITEM                 QTY");
    println!("------------------------");
    for item in order {
        println!("{:19} {:4}", item.name, item.amount);
    }
}
