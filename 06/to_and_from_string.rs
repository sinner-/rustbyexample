use std::string::ToString;
use std::str::FromStr;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let info: Vec<&str> = s.split("Circle of radius ").collect();

        Ok(Circle {
            radius: info[1].parse::<i32>().unwrap(),
        })
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed+turbo_parsed;
    println!("sum: {:?}", sum);

    let circle2 = Circle::from_str("Circle of radius 27").unwrap();
    println!("from_str: {}", circle2.to_string());

}
