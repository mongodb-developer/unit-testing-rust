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
    pub fn attack(&self, weapon: &impl Megaweapon) {
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
    use std::{cell::RefCell, panic};

    use super::*;

    const PRIMARY_FIRST_NAME: &str = "Lex";
    const PRIMARY_LAST_NAME: &str = "Luthor";
    const PRIMARY_FULL_NAME: &str = "Lex Luthor";
    const SECONDARY_FIRST_NAME: &str = "Darth";
    const SECONDARY_LAST_NAME: &str = "Vader";
    const SECONDARY_FULL_NAME: &str = "Darth Vader";

    #[test]
    fn full_name_is_first_name_space_last_name() {
        run_test(|ctx| {
            // Act
            let full_name = ctx.sut.full_name();
            // Assert
            assert_eq!(full_name, PRIMARY_FULL_NAME);
        });
    }
    #[test]
    fn set_full_name_sets_first_and_last_name() {
        // Arrange
        let mut sut = Supervillain {
            first_name: PRIMARY_FIRST_NAME.to_string(),
            last_name: PRIMARY_LAST_NAME.to_string(),
        };
        // Act
        sut.set_full_name(SECONDARY_FULL_NAME);
        // Assert
        assert_eq!(sut.first_name, SECONDARY_FIRST_NAME);
        assert_eq!(sut.last_name, SECONDARY_LAST_NAME);
    }
    #[test]
    fn from_str_slice_produces_supervillain_with_first_and_last_name() {
        // Arrange

        // Act
        let sut = Supervillain::from(SECONDARY_FULL_NAME);
        // Assert
        assert_eq!(sut.first_name, SECONDARY_FIRST_NAME);
        assert_eq!(sut.last_name, SECONDARY_LAST_NAME);
    }
    #[test]
    fn attack_shoots_weapon() {
        // Arrange
        let sut = Supervillain {
            first_name: PRIMARY_FIRST_NAME.to_string(),
            last_name: PRIMARY_LAST_NAME.to_string(),
        };
        let weapon = WeaponDouble::new();
        // Act
        sut.attack(&weapon);
        // Assert
        assert!(*weapon.is_shot.borrow());
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
    struct Context {
        sut: Supervillain,
    }
    impl Context {
        fn setup() -> Context {
            Context {
                sut: Supervillain {
                    first_name: PRIMARY_FIRST_NAME.to_string(),
                    last_name: PRIMARY_LAST_NAME.to_string(),
                },
            }
        }
        fn teardown(self) {}
    }
    fn run_test<T>(tst: T)
    where
        T: FnOnce(&mut Context) -> () + panic::UnwindSafe,
    {
        let mut ctx = Context::setup();
        let mut wrapper = panic::AssertUnwindSafe(&mut ctx);
        let result = panic::catch_unwind(move || tst(*wrapper));
        ctx.teardown();
        if let Err(err) = result {
            std::panic::resume_unwind(err);
        }
    }
}
