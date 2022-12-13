pub struct Supervillain {
    pub first_name: String,
    pub last_name: String,
}

pub trait Megaweapon {
    fn shoot(&self);
}

impl Supervillain {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn set_full_name(&mut self, name: &str) {
        let components = name.split(" ").collect::<Vec<_>>();
        self.first_name = components[0].to_string();
        self.last_name = components[1].to_string();
    }
    pub fn attack(&self, weapon: impl Megaweapon) {
        weapon.shoot();
    }
}

impl From<&str> for Supervillain {
    fn from(name: &str) -> Self {
        let components = name.split(" ").collect::<Vec<_>>();
        Supervillain {
            first_name: components[0].to_string(),
            last_name: components[1].to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    #[test]
    fn full_name_is_first_name_space_last_name() {
        // Arrange
        let sut = Supervillain {
            first_name: "Lex".to_string(),
            last_name: "Luthor".to_string(),
        };
        // Act
        let full_name = sut.full_name();
        // Assert
        assert_eq!(full_name, "Lex Luthor", "Unexpected full name");
    }
    #[test]
    fn set_full_name_sets_first_and_last_name() {
        // Arrange
        let mut sut = Supervillain {
            first_name: "Lex".to_string(),
            last_name: "Luthor".to_string(),
        };
        // Act
        sut.set_full_name("Darth Vader");
        // Assert
        assert_eq!(sut.first_name, "Darth");
        assert_eq!(sut.last_name, "Vader");
    }
    #[test]
    fn from_str_slice_produces_supervillain_with_first_and_last_name() {
        // Arrange

        // Act
        let sut = Supervillain::from("Darth Vader");
        // Assert
        assert_eq!(sut.first_name, "Darth");
        assert_eq!(sut.last_name, "Vader");
    }
    #[test]
    fn attack_shoots_weapon() {
        // Arrange
        let sut = Supervillain {
            first_name: "Lex".to_string(),
            last_name: "Luthor".to_string(),
        };
        let weapon = WeaponDouble::new();
        // Act
        sut.attack(weapon);
        // Assert
        // assert!(weapon.is_shot);
    }
    struct WeaponDouble {
        pub is_shot: RefCell<bool>,
    }
    impl WeaponDouble {
        fn new() -> WeaponDouble {
            WeaponDouble {
                is_shot: RefCell::new(false),
            }
        }
    }
    impl Megaweapon for WeaponDouble {
        fn shoot(&self) {
            *self.is_shot.borrow_mut() = true;
        }
    }
    impl Drop for WeaponDouble {
        fn drop(&mut self) {
            if *self.is_shot.borrow() != true {
                panic!("Failed to call shoot()");
            }
        }
    }
}
