use std::{collections::HashMap, error::Error};
#[derive(Clone, Debug)]
struct Order<'a> {
    name: &'a str,
}
fn order_sweets<T: Taxable>(orders: &Vec<Order>, prices: &HashMap<&str, T>) -> Option<f64> {
    let mut price = 0.0;
    reverse(&mut orders.clone());
    for order in orders {
        price += (prices.get(&order.name))
            .expect(&("bad name: ".to_string() + order.name))
            .tax();
    }
    Some(price)
}

trait Taxable {
    fn tax(&self) -> f64;
}
impl Taxable for f64 {
    fn tax(&self) -> f64 {
        self * 1.06
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
    let prices = HashMap::from([
        ("Cake", 1.2),
        ("Donut", 1.0),
        ("Apple Pie", 6.25),
        ("Birthday Cake", 12.65),
        ("Whole Wheat Bread", 5.25),
        ("Artisan White Bread", 4.25),
    ]);

    let order = vec![
        Order { name: "Cake" },
        Order { name: "Donut" },
        Order { name: "Apple Pie" },
        Order {
            name: "Whole Wheat Bread",
        },
    ];
    let total_price = order_sweets(&order, &prices);
    println!("{:?}", total_price);
    println!("{:?}", prices);
    println!("{:?}", order);

    use std::thread;
    use std::time::Duration;

    const SLEEP1: u8 = 2;
    const SLEEP2: u8 = 4;
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(SLEEP1 as u64));
        println!("Hello from the spawned thread (after {SLEEP1} second sleep)!");
    });
    println!("Spawned a thread");

    thread::sleep(Duration::from_secs(SLEEP2 as u64));
    println!("Hello from the main thread (after {SLEEP2} seconds)!");
    Ok(())
}
