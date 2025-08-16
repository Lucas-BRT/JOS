use crate::domain::password::{PasswordDomainError, PasswordRequirement};
use crate::{Error, Result};

const DEFAULT_MIN_LENGTH: usize = 8;
const DEFAULT_MAX_LENGTH: usize = 128;
const DEFAULT_MIN_DIGITS: usize = 1;
const DEFAULT_MIN_SPECIAL_CHARS: usize = 1;
const DEFAULT_ALLOWED_SPECIAL_CHARS: &str = "!@#$%^&*()_+-=[]{}|;':\",./<>?`~";

#[derive(Clone)]
pub struct LengthRequirement {
    min_length: usize,
    max_length: usize,
}

impl LengthRequirement {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        Self {
            min_length,
            max_length,
        }
    }
}

impl Default for LengthRequirement {
    fn default() -> Self {
        Self {
            min_length: DEFAULT_MIN_LENGTH,
            max_length: DEFAULT_MAX_LENGTH,
        }
    }
}

#[derive(Clone)]
pub struct CaseRequirement {
    require_uppercase: bool,
    require_lowercase: bool,
}

impl Default for CaseRequirement {
    fn default() -> Self {
        Self {
            require_uppercase: true,
            require_lowercase: true,
        }
    }
}

#[derive(Clone)]
pub struct DigitRequirement {
    require_digit: bool,
    min_digits: usize,
}

impl Default for DigitRequirement {
    fn default() -> Self {
        Self {
            require_digit: true,
            min_digits: DEFAULT_MIN_DIGITS,
        }
    }
}

#[derive(Clone)]
pub struct SpecialCharsRequirement {
    require_special: bool,
    allowed_special_chars: String,
}

impl Default for SpecialCharsRequirement {
    fn default() -> Self {
        Self {
            require_special: true,
            allowed_special_chars: DEFAULT_ALLOWED_SPECIAL_CHARS.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct CommonPasswordRequirement {
    require_common_password: bool,
    common_passwords: Vec<String>,
}

#[derive(Clone)]
pub struct PasswordValidator {
    length_requirement: LengthRequirement,
    case_requirement: Option<CaseRequirement>,
    digit_requirement: Option<DigitRequirement>,
    special_requirement: Option<SpecialCharsRequirement>,
    common_password_requirement: Option<CommonPasswordRequirement>,
}

impl Default for PasswordValidator {
    fn default() -> Self {
        Self {
            length_requirement: LengthRequirement::default(),
            case_requirement: None,
            digit_requirement: None,
            special_requirement: None,
            common_password_requirement: None,
        }
    }
}

impl PasswordValidator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PasswordValidator {
    pub fn with_length_requirement(
        mut self,
        length_requirement: Option<LengthRequirement>,
    ) -> Self {
        self.length_requirement = length_requirement.unwrap_or(LengthRequirement::default());
        self
    }

    pub fn with_case_requirement(mut self, case_requirement: Option<CaseRequirement>) -> Self {
        self.case_requirement = case_requirement;
        self
    }

    pub fn with_digit_requirement(mut self, digit_requirement: Option<DigitRequirement>) -> Self {
        self.digit_requirement = digit_requirement;
        self
    }

    pub fn with_special_requirement(
        mut self,
        special_requirement: Option<SpecialCharsRequirement>,
    ) -> Self {
        self.special_requirement = special_requirement;
        self
    }

    pub fn with_common_password_requirement(
        mut self,
        common_password_requirement: Option<CommonPasswordRequirement>,
    ) -> Self {
        self.common_password_requirement = common_password_requirement;
        self
    }

    pub fn validate(&self, password: &str) -> Result<()> {
        if password.len() < self.length_requirement.min_length {
            return Err(Error::Domain(PasswordDomainError::TooShort.into()));
        }

        if password.len() > self.length_requirement.max_length {
            return Err(Error::Domain(PasswordDomainError::TooLong.into()));
        }

        if let Some(case_requirement) = &self.case_requirement {
            if case_requirement.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
                return Err(Error::Domain(PasswordDomainError::MissingUppercase.into()));
            }
        }

        if let Some(case_requirement) = &self.case_requirement {
            if case_requirement.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
                return Err(Error::Domain(PasswordDomainError::MissingLowercase.into()));
            }
        }

        if let Some(digit_requirement) = &self.digit_requirement {
            if digit_requirement.require_digit && !password.chars().any(|c| c.is_digit(10)) {
                return Err(Error::Domain(PasswordDomainError::MissingDigit.into()));
            }
        }

        if let Some(special_requirement) = &self.special_requirement {
            if special_requirement.require_special
                && !password
                    .chars()
                    .any(|c| special_requirement.allowed_special_chars.contains(c))
            {
                return Err(Error::Domain(
                    PasswordDomainError::MissingSpecialChar.into(),
                ));
            }
        }

        if let Some(common_password_requirement) = &self.common_password_requirement {
            if common_password_requirement.require_common_password
                && common_password_requirement
                    .common_passwords
                    .contains(&password.to_string())
            {
                return Err(Error::Domain(PasswordDomainError::TooCommon.into()));
            }
        }

        Ok(())
    }
}

impl PasswordValidator {
    pub fn get_requirements(&self) -> Vec<PasswordRequirement> {
        vec![
            PasswordRequirement::new(
                "Length".to_string(),
                self.length_requirement.min_length.to_string(),
            ),
            PasswordRequirement::new(
                "Case".to_string(),
                self.case_requirement
                    .as_ref()
                    .unwrap_or(&CaseRequirement::default())
                    .require_uppercase
                    .to_string(),
            ),
            PasswordRequirement::new(
                "Digit".to_string(),
                self.digit_requirement
                    .as_ref()
                    .unwrap_or(&DigitRequirement::default())
                    .require_digit
                    .to_string(),
            ),
            PasswordRequirement::new(
                "Special".to_string(),
                self.special_requirement
                    .as_ref()
                    .unwrap_or(&SpecialCharsRequirement::default())
                    .require_special
                    .to_string(),
            ),
        ]
    }
}
