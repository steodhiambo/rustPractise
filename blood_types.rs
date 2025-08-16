#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        use Antigen::*;
        use RhFactor::*;
        
        // Check Rh compatibility: positive can receive from both, negative only from negative
        let rh_compatible = match (&self.rh_factor, &other.rh_factor) {
            (Positive, _) => true,
            (Negative, Negative) => true,
            (Negative, Positive) => false,
        };
        
        if !rh_compatible {
            return false;
        }
        
        // Check antigen compatibility
        match (&self.antigen, &other.antigen) {
            (AB, _) => true,           // AB can receive from everyone
            (A, A) | (A, O) => true,   // A can receive from A and O
            (B, B) | (B, O) => true,   // B can receive from B and O
            (O, O) => true,            // O can only receive from O
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_types = Self::all_blood_types();
        all_types
            .into_iter()
            .filter(|donor| self.can_receive_from(donor))
            .collect()
    }

    pub fn recipients(&self) -> Vec<Self> {
        let all_types = Self::all_blood_types();
        all_types
            .into_iter()
            .filter(|recipient| recipient.can_receive_from(self))
            .collect()
    }
    
    fn all_blood_types() -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;
        
        vec![
            BloodType { antigen: A, rh_factor: Positive },
            BloodType { antigen: A, rh_factor: Negative },
            BloodType { antigen: B, rh_factor: Positive },
            BloodType { antigen: B, rh_factor: Negative },
            BloodType { antigen: AB, rh_factor: Positive },
            BloodType { antigen: AB, rh_factor: Negative },
            BloodType { antigen: O, rh_factor: Positive },
            BloodType { antigen: O, rh_factor: Negative },
        ]
    }
}

fn main() {
    let blood_type = BloodType {
        antigen: Antigen::O,
        rh_factor: RhFactor::Positive,
    };
    println!("recipients of O+ {:?}", blood_type.recipients());
    println!("donors of O+ {:?}", blood_type.donors());
    let another_blood_type = BloodType {
        antigen: Antigen::O,
        rh_factor: RhFactor::Positive,
    };
    println!(
        "donors of O+ can receive from {:?} {:?}",
        &another_blood_type,
        blood_type.can_receive_from(&another_blood_type)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_ab_neg_with_a_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_a_pos() {
        let blood_type = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_ab_neg() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_neg_with_o_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_pos_with_o_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_compatible_ab_neg_with_o_neg() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_antigen_ab_from_str() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        assert_eq!(blood_type.antigen, Antigen::AB);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);
    }

    #[test]
    fn test_antigen_a_from_str() {
        let blood_type = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        assert_eq!(blood_type.antigen, Antigen::A);
        assert_eq!(blood_type.rh_factor, RhFactor::Negative);
    }

    #[test]
    fn test_donors() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let mut givers = blood_type.donors();
        // println!("Before sorting {:?}", &givers);
        givers.sort();
        // println!("{:?}", &givers);
        let mut expected = vec![
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
        ];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_a_neg_donors() {
        let blood = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        let mut givers = blood.donors();
        givers.sort();
        let mut expected = vec![
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
        ];

        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_o_neg_donors() {
        let blood = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        };

        let mut givers = blood.donors();
        givers.sort();
        let mut expected = vec![blood.clone()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_ab_pos_recipients() {
        let blood = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let mut recipients = blood.recipients();
        recipients.sort();
        let mut expected = vec![blood.clone()];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_a_neg_recipients() {
        let blood = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };

        let mut recipients = blood.recipients();
        recipients.sort();
        let mut expected = vec![
            blood.clone(),
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
        ];
        expected.sort();
        assert_eq!(recipients, expected);
    }
}
