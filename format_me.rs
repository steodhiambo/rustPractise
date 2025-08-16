use std::fmt;

pub struct Park {
    pub name: String,
    pub park_type: ParkType,
    pub address: String,
    pub cap: String,
    pub state: String,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = if self.name.is_empty() { "No name" } else { &self.name };
        let address = if self.address.is_empty() { "No address" } else { &self.address };
        let cap = if self.cap.is_empty() { "No cap" } else { &self.cap };
        let state = if self.state.is_empty() { "No state" } else { &self.state };
        
        write!(f, "{} - {}, {}, {} - {}", 
               self.park_type, name, address, cap, state)
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let park_type_str = match self {
            ParkType::Garden => "garden",
            ParkType::Forest => "forest",
            ParkType::Playground => "playground",
        };
        write!(f, "{}", park_type_str)
    }
}

fn main() {
    println!(
        "{}",
        Park {
            name: "Les Tuileries".to_string(),
            park_type: ParkType::Garden,
            address: "Pl. de la Concorde".to_string(),
            cap: "75001".to_string(),
            state: "France".to_string()
        }
    );
    println!(
        "{}",
        Park {
            name: "".to_string(),
            park_type: ParkType::Playground,
            address: "".to_string(),
            cap: "".to_string(),
            state: "".to_string()
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_park_display_with_values() {
        let park = Park {
            name: "Les Tuileries".to_string(),
            park_type: ParkType::Garden,
            address: "Pl. de la Concorde".to_string(),
            cap: "75001".to_string(),
            state: "France".to_string()
        };
        assert_eq!(park.to_string(), "garden - Les Tuileries, Pl. de la Concorde, 75001 - France");
    }

    #[test]
    fn test_park_display_with_empty_values() {
        let park = Park {
            name: "".to_string(),
            park_type: ParkType::Playground,
            address: "".to_string(),
            cap: "".to_string(),
            state: "".to_string()
        };
        assert_eq!(park.to_string(), "playground - No name, No address, No cap - No state");
    }

    #[test]
    fn test_park_type_display() {
        assert_eq!(ParkType::Garden.to_string(), "garden");
        assert_eq!(ParkType::Forest.to_string(), "forest");
        assert_eq!(ParkType::Playground.to_string(), "playground");
    }

    #[test]
    fn test_mixed_empty_and_filled_fields() {
        let park = Park {
            name: "Central Park".to_string(),
            park_type: ParkType::Forest,
            address: "".to_string(),
            cap: "10001".to_string(),
            state: "".to_string()
        };
        assert_eq!(park.to_string(), "forest - Central Park, No address, 10001 - No state");
    }
}
