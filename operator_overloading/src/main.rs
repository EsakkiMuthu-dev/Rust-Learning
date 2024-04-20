use std::ops::Add;

#[derive(Debug)]
struct Person {
    firstname: String,
    lastname: String,
}

#[derive(Debug)]
struct Marriage {
    husband: Person,
    wife: Person,
    location: String,
    date: chrono::naive::NaiveDate,
}

impl Add<Person> for Person {
    type Output = Marriage;

    fn add(self, rhs: Self) -> Self::Output {
        let marriage = Marriage {
            husband: self,
            wife: rhs,
            location: "Tirunelveli".to_string(),
            date: chrono::offset::Local::now().date_naive(),
        };
        marriage
    }
}
fn main() {
    let person1 = Person {
        firstname: "Some one".to_string(),
        lastname: "some one's last name".to_string(),
    };
    let person2 = Person {
        firstname: "No one".to_string(),
        lastname: "No one's last name".to_string(),
    };
    println!("person : {:?} ", person1);
    let couples = person1 + person2;
    println!("{:?}", couples);
}
