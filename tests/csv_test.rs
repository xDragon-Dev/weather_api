use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize)]
struct Person {
    pub name: String,
    pub age: u8,
    pub gender: Gender,
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    let people = vec![
        Person {
            name: "Juan".into(),
            age: 20,
            gender: Gender::Male,
        },
        Person {
            name: "Maria".into(),
            age: 20,
            gender: Gender::Female,
        },
    ];

    let mut writter = csv::Writer::from_writer(Vec::new());
    for person in people {
        writter.serialize(person)?;
    }
    let inner = writter.into_inner()?;
    let _output = String::from_utf8(inner)?;
    Ok(())
}
