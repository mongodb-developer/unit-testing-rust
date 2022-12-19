#![allow(missing_docs)]
//! Module for supervillains and their related stuff
use std::time::Duration;

use anyhow::anyhow;

/// Type that represents supervillains.
pub struct Supervillain {
    pub first_name: String,
    pub last_name: String,
}

pub trait Megaweapon {
    fn shoot(&self);
}

impl Supervillain {
    /// Return the value of the full name as a single string.
    ///
    /// Full name is produced concatenating first name, a single space, and the last name.
    ///
    /// # Examples
    /// ```
    /// # use evil::supervillain::Supervillain;
    /// let lex = Supervillain {
    ///     first_name: "Lex".to_string(),
    ///     last_name: "Luthor".to_string(),
    /// };
    /// assert_eq!(lex.full_name(), "Lex Luthor");
    /// ```
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn set_full_name(&mut self, name: &str) {
        let components = name.split(" ").collect::<Vec<_>>();
        println!("Received {} components.", components.len());
        if components.len() != 2 {
            panic!("Name must have first and last name");
        }
        self.first_name = components[0].to_string();
        self.last_name = components[1].to_string();
    }
    pub fn attack(&self, weapon: &impl Megaweapon) {
        weapon.shoot();
    }
    pub async fn come_up_with_plan(&self) -> String {
        tokio::time::sleep(Duration::from_millis(100)).await;
        String::from("Take over the world!")
    }
}

impl TryFrom<&str> for Supervillain {
    type Error = anyhow::Error;
    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let components = name.split(" ").collect::<Vec<_>>();
        if components.len() < 2 {
            Err(anyhow!("Too few arguments"))
        } else {
            Ok(Supervillain {
                first_name: components[0].to_string(),
                last_name: components[1].to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::test_common;
    use test_context::{test_context, AsyncTestContext, TestContext};

    #[test_context(Context)]
    #[test]
    fn full_name_returns_first_name_space_last_name(ctx: &mut Context) {
        // Act
        let full_name = ctx.sut.full_name();
        // Assert
        assert_eq!(full_name, test_common::PRIMARY_FULL_NAME);
    }
    #[test_context(Context)]
    #[test]
    fn set_full_name_sets_first_and_last_name(ctx: &mut Context) {
        // Act
        ctx.sut.set_full_name(test_common::SECONDARY_FULL_NAME);
        // Assert
        assert_eq!(ctx.sut.first_name, test_common::SECONDARY_FIRST_NAME);
        assert_eq!(ctx.sut.last_name, test_common::SECONDARY_LAST_NAME);
    }
    #[test_context(Context)]
    #[test]
    #[should_panic(expected = "Name must have first and last name")]
    fn set_full_name_panics_with_empty_name(ctx: &mut Context) {
        // Arrange

        // Act
        ctx.sut.set_full_name("");
        // Assert
    }
    #[test]
    fn from_str_slice_produces_supervillain_with_first_and_last_name() -> Result<(), anyhow::Error>
    {
        // Act
        let sut = Supervillain::try_from(test_common::SECONDARY_FULL_NAME)?;
        // Assert
        assert_eq!(sut.first_name, test_common::SECONDARY_FIRST_NAME);
        assert_eq!(sut.last_name, test_common::SECONDARY_LAST_NAME);
        Ok(())
    }
    #[test]
    fn from_str_slice_produces_error_with_less_than_two_substrings() {
        // Act
        let result = Supervillain::try_from("");
        // Assert
        let Err(_) = result else { panic!("Unexpected value returned by try_from"); };
    }
    #[test_context(Context)]
    #[test]
    fn attack_shoots_weapon(ctx: &mut Context) {
        // Arrange
        let weapon = WeaponDouble::new();
        // Act
        ctx.sut.attack(&weapon);
        // Assert
        assert!(*weapon.is_shot.borrow());
    }
    #[test_context(AsyncContext)]
    #[tokio::test]
    async fn plan_is_sadly_expected(ctx: &mut AsyncContext) {
        assert_eq!(ctx.sut.come_up_with_plan().await, "Take over the world!");
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
    impl TestContext for Context {
        fn setup() -> Context {
            Context {
                sut: Supervillain {
                    first_name: test_common::PRIMARY_FIRST_NAME.to_string(),
                    last_name: test_common::PRIMARY_LAST_NAME.to_string(),
                },
            }
        }
        fn teardown(self) {}
    }
    struct AsyncContext {
        sut: Supervillain,
    }
    #[async_trait::async_trait]
    impl AsyncTestContext for AsyncContext {
        async fn setup() -> AsyncContext {
            AsyncContext {
                sut: Supervillain {
                    first_name: test_common::PRIMARY_FIRST_NAME.to_string(),
                    last_name: test_common::PRIMARY_LAST_NAME.to_string(),
                },
            }
        }
        async fn teardown(self) {}
    }
}
