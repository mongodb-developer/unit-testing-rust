pub struct Supervillain {
    pub first_name: String,
    pub last_name: String,
}

impl Supervillain {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[cfg(test)]
mod tests {
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
}
