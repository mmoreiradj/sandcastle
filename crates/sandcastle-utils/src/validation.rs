use std::fmt;
use validator::ValidationError;

#[derive(Debug, Clone)]
pub enum K8sValidationError {
    TooLong(usize, usize),
    TooShort(usize, usize),
    InvalidFormat(String),
    InvalidCharacter(char),
    InvalidStart(char),
    InvalidEnd(char),
    Empty,
    InvalidPrefix(String),
    InvalidName(String),
}

impl fmt::Display for K8sValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            K8sValidationError::TooLong(actual, max) => {
                write!(f, "Name too long: {} characters (max: {})", actual, max)
            }
            K8sValidationError::TooShort(actual, min) => {
                write!(f, "Name too short: {} characters (min: {})", actual, min)
            }
            K8sValidationError::InvalidFormat(format) => {
                write!(f, "Name does not match required format: {}", format)
            }
            K8sValidationError::InvalidCharacter(ch) => {
                write!(f, "Invalid character '{}' in name", ch)
            }
            K8sValidationError::InvalidStart(ch) => {
                write!(f, "Name cannot start with '{}'", ch)
            }
            K8sValidationError::InvalidEnd(ch) => {
                write!(f, "Name cannot end with '{}'", ch)
            }
            K8sValidationError::Empty => {
                write!(f, "Name cannot be empty")
            }
            K8sValidationError::InvalidPrefix(prefix) => {
                write!(f, "Invalid prefix: {}", prefix)
            }
            K8sValidationError::InvalidName(name) => {
                write!(f, "Invalid name: {}", name)
            }
        }
    }
}

impl From<K8sValidationError> for ValidationError {
    fn from(err: K8sValidationError) -> Self {
        let message = Box::leak(err.to_string().into_boxed_str());
        ValidationError::new(message)
    }
}

pub fn validate_dns_label(name: &str, max_length: usize) -> Result<(), K8sValidationError> {
    if name.is_empty() {
        return Err(K8sValidationError::Empty);
    }

    if name.len() > max_length {
        return Err(K8sValidationError::TooLong(name.len(), max_length));
    }

    let chars: Vec<char> = name.chars().collect();

    if !chars[0].is_ascii_alphanumeric() {
        return Err(K8sValidationError::InvalidStart(chars[0]));
    }

    if !chars[chars.len() - 1].is_ascii_alphanumeric() {
        return Err(K8sValidationError::InvalidEnd(chars[chars.len() - 1]));
    }

    for ch in &chars {
        if !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && *ch != '-' {
            return Err(K8sValidationError::InvalidCharacter(*ch));
        }
    }

    Ok(())
}

pub fn validate_dns_subdomain(name: &str, max_length: usize) -> Result<(), K8sValidationError> {
    if name.is_empty() {
        return Err(K8sValidationError::Empty);
    }

    if name.len() > max_length {
        return Err(K8sValidationError::TooLong(name.len(), max_length));
    }

    for label in name.split('.') {
        validate_dns_label(label, 63)?;
    }

    Ok(())
}

pub fn validate_label_value(value: &str) -> Result<(), K8sValidationError> {
    if value.is_empty() {
        return Ok(());
    }

    if value.len() > 63 {
        return Err(K8sValidationError::TooLong(value.len(), 63));
    }

    let chars: Vec<char> = value.chars().collect();

    if !chars[0].is_ascii_alphanumeric() {
        return Err(K8sValidationError::InvalidStart(chars[0]));
    }

    if !chars[chars.len() - 1].is_ascii_alphanumeric() {
        return Err(K8sValidationError::InvalidEnd(chars[chars.len() - 1]));
    }

    for ch in &chars {
        if !ch.is_ascii_alphanumeric() && *ch != '-' && *ch != '_' && *ch != '.' {
            return Err(K8sValidationError::InvalidCharacter(*ch));
        }
    }

    Ok(())
}

pub fn validate_label_key(key: &str) -> Result<(), K8sValidationError> {
    if key.is_empty() {
        return Err(K8sValidationError::Empty);
    }

    if key.len() > 253 {
        return Err(K8sValidationError::TooLong(key.len(), 253));
    }

    if let Some(slash_pos) = key.find('/') {
        let prefix = &key[..slash_pos];
        let name = &key[slash_pos + 1..];

        if !prefix.is_empty() {
            validate_dns_subdomain(prefix, 253)?;
        }

        validate_dns_label(name, 63)?;
    } else {
        validate_dns_label(key, 63)?;
    }

    Ok(())
}

pub fn validate_configmap_secret_key(key: &str) -> Result<(), K8sValidationError> {
    if key.is_empty() {
        return Err(K8sValidationError::Empty);
    }

    if key == "." || key == ".." {
        return Err(K8sValidationError::InvalidName(key.to_string()));
    }

    if key.starts_with('.') {
        return Err(K8sValidationError::InvalidStart('.'));
    }

    for ch in key.chars() {
        if !ch.is_ascii() {
            return Err(K8sValidationError::InvalidCharacter(ch));
        }
    }

    Ok(())
}

pub fn validate_k8s_dns_label(name: &str) -> Result<(), ValidationError> {
    validate_dns_label(name, 63).map_err(ValidationError::from)
}

pub fn validate_k8s_dns_subdomain(name: &str) -> Result<(), ValidationError> {
    validate_dns_subdomain(name, 253).map_err(ValidationError::from)
}

pub fn validate_k8s_label_key(key: &str) -> Result<(), ValidationError> {
    validate_label_key(key).map_err(ValidationError::from)
}

pub fn validate_k8s_label_value(value: &str) -> Result<(), ValidationError> {
    validate_label_value(value).map_err(ValidationError::from)
}

pub fn validate_k8s_configmap_key(key: &str) -> Result<(), ValidationError> {
    validate_configmap_secret_key(key).map_err(ValidationError::from)
}
