use std::collections::HashMap;

struct Order<'a> {
    name: &'a str,
}
fn order_sweets(orders: Vec<Order>, prices: HashMap<&str, f64>) -> Option<f64> {
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
    let total_price = order_sweets(order, prices);
    println!("{:?}", total_price);
}
