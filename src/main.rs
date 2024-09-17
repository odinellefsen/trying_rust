struct Person {
    name: String,
}

fn main() {
    let people = vec![
        Person {
            name: "Odin".to_string(),
        },
        Person {
            name: "Baldur".to_string(),
        },
        Person {
            name: "Thor".to_string(),
        }
    ];

    let names: Vec<String> = people.iter().map(|p| p.name.clone()).collect();
    println!("{:?}", names);
}
