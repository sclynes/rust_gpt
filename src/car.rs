use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Car {
    pub description: String,
    pub glove_box_contents: Vec<String>,
    pub year: i32,
    pub make: String,
    pub model: String,
    pub top_speed: i32,
    pub condition: i32,
    pub fuel: i32,
    pub color: String,
    pub driveable: bool,
}

#[allow(dead_code)]
impl Car {
    pub fn new(description: String, glove_box_contents: Vec<String>, year: i32, make: String, model: String, top_speed: i32, condition: i32, fuel: i32, color: String, driveable: bool) -> Self {
        Car {
            description,
            glove_box_contents,
            year,
            make,
            model,
            top_speed,
            condition,
            fuel,
            color,
            driveable,
        }
    }
    pub fn default() -> Self {
        Car {
            description: String::from(""),
            glove_box_contents: vec![],
            year: 0,
            make: String::from(""),
            model: String::from(""),
            top_speed: 0,
            condition: 0,
            fuel: 0,
            color: String::from(""),
            driveable: false,
        }
    }

    pub fn from_json(json: String) -> Self {
        match serde_json::from_str(&json) as Result<Car, serde_json::Error> {
            Ok(c) => c,
            Err(e) => {
                println!("Error parsing car data: {}", e);
                Car::default()
            }
        }
    }

    pub fn from_result(result: String) -> Self {
        match serde_json::from_str(&result) as Result<Car, serde_json::Error> {
            Ok(c) => c,
            Err(e) => {
                println!("Error parsing car data: {}", e);
                Car::default()
            }
        }
    }

    pub fn print(&self) {
        println!("Description: {}", self.description);
        println!("Glove Box Contents: {:?}", self.glove_box_contents);
        println!("Year: {}", self.year);
        println!("Make: {}", self.make);
        println!("Model: {}", self.model);
        println!("Top Speed: {}kph", self.top_speed);
        println!("Condition: {}", self.condition);
        println!("Fuel: {}%", self.fuel);
        println!("Color: {}", self.color);
        println!("Driveable: {}", self.driveable);
    }
}
