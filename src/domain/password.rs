use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordValidationError {
    pub message: String,
    pub validation_type: PasswordValidationType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PasswordValidationType {
    TooShort,
    TooLong,
    MissingUppercase,
    MissingLowercase,
    MissingDigit,
    MissingSpecialCharacter,
    ContainsInvalidCharacters,
    CommonPassword,
}

impl PasswordValidationError {
    pub fn new(message: String, validation_type: PasswordValidationType) -> Self {
        Self { message, validation_type }
    }
}

impl std::fmt::Display for PasswordValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PasswordValidationError {}

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordValidator {
    min_length: usize,
    max_length: usize,
    require_uppercase: bool,
    require_lowercase: bool,
    require_digit: bool,
    require_special: bool,
    allowed_special_chars: String,
}

impl Default for PasswordValidator {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special: false,
            allowed_special_chars: "!@#$%^&*()_+-=[]{}|;':\",./<>?`~".to_string(),
        }
    }
}

impl PasswordValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }

    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn with_uppercase_requirement(mut self, required: bool) -> Self {
        self.require_uppercase = required;
        self
    }

    pub fn with_lowercase_requirement(mut self, required: bool) -> Self {
        self.require_lowercase = required;
        self
    }

    pub fn with_digit_requirement(mut self, required: bool) -> Self {
        self.require_digit = required;
        self
    }

    pub fn with_special_requirement(mut self, required: bool) -> Self {
        self.require_special = required;
        self
    }

    pub fn with_allowed_special_chars(mut self, chars: String) -> Self {
        self.allowed_special_chars = chars;
        self
    }

    pub fn validate(&self, password: &str) -> Result<(), PasswordValidationError> {
        // Check length
        if password.len() < self.min_length {
            return Err(PasswordValidationError::new(
                format!("Password must be at least {} characters long", self.min_length),
                PasswordValidationType::TooShort,
            ));
        }

        if password.len() > self.max_length {
            return Err(PasswordValidationError::new(
                format!("Password must be at most {} characters long", self.max_length),
                PasswordValidationType::TooLong,
            ));
        }

        // Check character requirements
        if self.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(PasswordValidationError::new(
                "Password must contain at least one uppercase letter".to_string(),
                PasswordValidationType::MissingUppercase,
            ));
        }

        if self.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err(PasswordValidationError::new(
                "Password must contain at least one lowercase letter".to_string(),
                PasswordValidationType::MissingLowercase,
            ));
        }

        if self.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(PasswordValidationError::new(
                "Password must contain at least one digit".to_string(),
                PasswordValidationType::MissingDigit,
            ));
        }

        if self.require_special {
            let has_special = password.chars().any(|c| self.allowed_special_chars.contains(c));
            if !has_special {
                return Err(PasswordValidationError::new(
                    format!("Password must contain at least one special character: {}", self.allowed_special_chars),
                    PasswordValidationType::MissingSpecialCharacter,
                ));
            }
        }

        // Check for invalid characters (control characters)
        if password.chars().any(|c| c.is_control()) {
            return Err(PasswordValidationError::new(
                "Password contains invalid control characters".to_string(),
                PasswordValidationType::ContainsInvalidCharacters,
            ));
        }

        // Check for common passwords (basic check)
        let common_passwords = [
            "password", "123456", "123456789", "qwerty", "abc123", "password123",
            "admin", "letmein", "welcome", "monkey", "12345678", "1234567",
        ];

        if common_passwords.contains(&password.to_lowercase().as_str()) {
            return Err(PasswordValidationError::new(
                "Password is too common, please choose a more secure password".to_string(),
                PasswordValidationType::CommonPassword,
            ));
        }

        Ok(())
    }

    pub fn get_requirements(&self) -> Vec<String> {
        let mut requirements = Vec::new();
        
        requirements.push(format!("At least {} characters long", self.min_length));
        requirements.push(format!("At most {} characters long", self.max_length));
        
        if self.require_uppercase {
            requirements.push("At least one uppercase letter".to_string());
        }
        
        if self.require_lowercase {
            requirements.push("At least one lowercase letter".to_string());
        }
        
        if self.require_digit {
            requirements.push("At least one digit".to_string());
        }
        
        if self.require_special {
            requirements.push(format!("At least one special character: {}", self.allowed_special_chars));
        }
        
        requirements.push("No control characters allowed".to_string());
        requirements.push("Cannot be a common password".to_string());
        
        requirements
    }
}

#[async_trait]
pub trait PasswordRepository: Send + Sync {
    async fn generate_hash(&self, password: String) -> crate::Result<String>;
    async fn verify_hash(&self, password: String, hash: String) -> crate::Result<bool>;
    async fn validate_password(&self, password: &str) -> Result<(), PasswordValidationError>;
}
