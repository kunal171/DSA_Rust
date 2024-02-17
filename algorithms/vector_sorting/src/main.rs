
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn main() {
    //Sort a Vector of Integers
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("{:?}", vec);

    //Sort a Vector of Floats
    let mut vec1 = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec1.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec1, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
    println!("{:?}", vec1);

    //Sort a Vector of Structs
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    println!("{:?}", people);

    // Sort people by derived natural order (Name and age)
    people.sort();

    // Print the sorted vector by name and age
    println!("Sorted by name and age: {:?}", people);

    // Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age));

    // Print the sorted vector by age
    println!("Sorted by age: {:?}", people);
}