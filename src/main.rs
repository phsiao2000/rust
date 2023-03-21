use std::collections::HashMap;

struct Order {
    name: String,
}
fn order_sweets(orders: Vec<Order>, prices: HashMap<String, f64>) -> Option<f64> {
    let mut price = 0.0;
    for order in orders {
        price += prices.get(&order.name)?;
    } 
    // let donut_price = prices.get(&orders[0].name)?; // early return if None
    // let cake_price = prices.get(&orders[1].name)?; // early return if None
    Some(price)
}

fn main() {
    let prices = HashMap::from([
        ("Cake".to_string(), 1.2),
        ("Donut".to_string(), 1.0),
        ("Apple Pie".to_string(), 6.25),
        ("Birthday Cake".to_string(), 12.65),
        ("Whole Wheat Bread".to_string(), 5.25),
        ("Artisan White Bread".to_string(), 4.25),

    ]);

    let order = vec![
        Order {
            name: "Cake".to_string(),
        },
        Order {
            name: "Donut".to_string(),
        },
        Order {
            name: "Apple Pie".to_string(),
        },
        Order {
            name: "Whole Wheat Bread".to_string(),
        },

    ];
    let total_price = order_sweets(order, prices);
    println!("{:?}", total_price);
}
