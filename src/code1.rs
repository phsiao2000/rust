struct User {
    name: String,
    age: u16,
    occupation: Option<String>,
}
impl User {
    fn new(name: &str, age: u16, occupation: &str) -> User {
        User {
            name: String::from(name),
            age,
            occupation: (!occupation.is_empty()).then_some(String::from(occupation)),
        }
    }
}
fn main() {
    let users = vec![
        User::new("Tony", 25, "Baker"),
        User::new("Freddy", 35, "Salesman"),
        User::new("Carl", 63, "Teacher"),
        User::new("Amos", 25, "Fisherman"),
        User::new("Paul", 65, "Cleric"),
        User::new("Anthony", 32, "Mailman"),
        User::new("Amos", 25, "Fisherman"),
        User::new("Larry", 45, "Race Car Driver"),
        User::new("Hank", 55, "Priest"),
        User::new("David", 25, ""),
        User::new("Earl", 25, ""),
    ];
    for user in &users {
        println!(
            "{} (aged {}) occupation: {:?}",
            user.name,
            user.age,
            user.occupation.clone().unwrap_or(String::from("--NONE--"))
        )
    }
    let mut ns = vec![4, 5, 3, 6, 2, 3, 9, 6, 4, 2, 1, 3, 2];
    ns.retain(|num| num % 3 == 0);
    ns.sort();
    println!("ns_sorted: {ns:?}");
    const LIMIT: u16 = 40;
    let age_limit = |user:&&User| user.age > LIMIT;
    let mut older = users
        .iter()
        .filter(age_limit)
        .collect::<Vec<&User>>();
    for user in &older {
        println!(
            "older {} (aged {}) occupation: {:?}",
            user.name,
            user.age,
            user.occupation.clone().unwrap_or(String::from("--NONE--"))
        );
    }
    older.sort_by(|a, b| a.age.cmp(&b.age));

    println!("Older users: (over {LIMIT}), sorted by age");
    for user in &older {
        println!(
            "{} (aged {}) occupation: {:?}",
            user.name,
            user.age,
            user.occupation.clone().unwrap_or(String::from("--NONE--"))
        )
    }

    //older.retain(|p: &&User| p.age > 60);
    for user in &{older.retain(|p: &&User| p.age > 60); older} {
        println!(
            "older than 60 {} (aged {}) occupation: {:?}",
            user.name,
            user.age,
            user.occupation.clone().unwrap_or(String::from("--NONE--"))
        );
    }
}
