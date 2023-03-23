pub fn hello() -> String {
    String::from("hello one and all!")
}

fn create_greeting() -> impl Fn(&str) -> String {
    let greet = "Hello,";
    move |name: &str| format!("{greet} {name}!")
}

pub fn inc_and_tax(v: f64) -> f64 {
    let greeting_function = create_greeting();
    let greet_rust = greeting_function("Rusty nail");

    println!("{}", greet_rust);

    let tax = 1.1;
    let incr = |val: f64| (val + 5.0) * tax;
    incr(v)
}
