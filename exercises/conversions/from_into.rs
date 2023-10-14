#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            return Person::default();
        }

        let parts: Vec<&str> = s.split(',').collect();

        let name = parts.get(0).unwrap_or(&"").to_string();
        if name.is_empty() {
            return Person::default();
        }

        if let Some(age_str) = parts.get(1) {
            if let Ok(age) = age_str.parse::<usize>() {
                return Person { name, age };
            }
        }

        Person::default()
    }
}

fn main() {
    let p1 = Person::from("Mark,20");
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    // test functions
}
