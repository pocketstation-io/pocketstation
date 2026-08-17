use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::graph::NodeConfig;
use crate::session::EndpointConfiguration;

pub const MAX_CONNECTOR_CONFIGURATION_FIELDS: usize = 128;
pub const MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES: usize = 16 * 1024;

#[derive(Clone, PartialEq, Eq)]
pub struct ConnectorSecret(String);

impl ConnectorSecret {
    pub fn new(value: impl Into<String>) -> Result<Self, ConnectorConfigurationError> {
        let value = value.into();
        if value.is_empty() {
            return Err(ConnectorConfigurationError::new(
                ConnectorConfigurationErrorCode::EmptySecret,
                None,
                "secret value cannot be empty",
            ));
        }
        if value.len() > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES {
            return Err(ConnectorConfigurationError::new(
                ConnectorConfigurationErrorCode::ValueTooLarge,
                None,
                "secret value exceeds the connector configuration byte limit",
            ));
        }
        Ok(Self(value))
    }

    /// Exposes the secret to the owning connector during setup or worker use.
    ///
    /// The returned value must not be copied into logs, errors, metrics, or
    /// serialized observations.
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ConnectorSecret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ConnectorSecret(<redacted>)")
    }
}

impl Drop for ConnectorSecret {
    fn drop(&mut self) {
        crate::secret::clear_string(&mut self.0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorConfigurationValueKind {
    Text,
    Boolean,
    SignedInteger,
    UnsignedInteger,
    DurationMilliseconds,
    ByteCount,
    Secret,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectorConfigurationValue {
    Text(String),
    Boolean(bool),
    SignedInteger(i64),
    UnsignedInteger(u64),
    DurationMilliseconds(u64),
    ByteCount(u64),
    Secret(ConnectorSecret),
}

impl ConnectorConfigurationValue {
    pub const fn kind(&self) -> ConnectorConfigurationValueKind {
        match self {
            Self::Text(_) => ConnectorConfigurationValueKind::Text,
            Self::Boolean(_) => ConnectorConfigurationValueKind::Boolean,
            Self::SignedInteger(_) => ConnectorConfigurationValueKind::SignedInteger,
            Self::UnsignedInteger(_) => ConnectorConfigurationValueKind::UnsignedInteger,
            Self::DurationMilliseconds(_) => ConnectorConfigurationValueKind::DurationMilliseconds,
            Self::ByteCount(_) => ConnectorConfigurationValueKind::ByteCount,
            Self::Secret(_) => ConnectorConfigurationValueKind::Secret,
        }
    }

    fn encoded(&self) -> String {
        match self {
            Self::Text(value) => value.clone(),
            Self::Boolean(value) => value.to_string(),
            Self::SignedInteger(value) => value.to_string(),
            Self::UnsignedInteger(value)
            | Self::DurationMilliseconds(value)
            | Self::ByteCount(value) => value.to_string(),
            Self::Secret(value) => value.expose_secret().to_owned(),
        }
    }

    fn text_bytes(&self) -> usize {
        match self {
            Self::Text(value) => value.len(),
            Self::Secret(value) => value.expose_secret().len(),
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConnectorConfiguration {
    values: BTreeMap<String, ConnectorConfigurationValue>,
}

impl ConnectorConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with(mut self, key: impl Into<String>, value: ConnectorConfigurationValue) -> Self {
        self.values.insert(key.into(), value);
        self
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: ConnectorConfigurationValue,
    ) -> Option<ConnectorConfigurationValue> {
        self.values.insert(key.into(), value)
    }

    pub fn get(&self, key: &str) -> Option<&ConnectorConfigurationValue> {
        self.values.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &ConnectorConfigurationValue)> {
        self.values.iter().map(|(key, value)| (key.as_str(), value))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectorConfigurationRequirement {
    Required,
    Optional,
    Default(ConnectorConfigurationValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectorConfigurationConstraint {
    NonEmpty,
    TextLengthBytes { minimum: usize, maximum: usize },
    SignedRange { minimum: i64, maximum: i64 },
    UnsignedRange { minimum: u64, maximum: u64 },
    OneOf(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorConfigurationField {
    name: String,
    value_kind: ConnectorConfigurationValueKind,
    requirement: ConnectorConfigurationRequirement,
    documentation: String,
    constraints: Vec<ConnectorConfigurationConstraint>,
    deprecation: Option<String>,
}

impl ConnectorConfigurationField {
    pub fn new(
        name: impl Into<String>,
        value_kind: ConnectorConfigurationValueKind,
        requirement: ConnectorConfigurationRequirement,
        documentation: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            value_kind,
            requirement,
            documentation: documentation.into(),
            constraints: Vec::new(),
            deprecation: None,
        }
    }

    #[must_use]
    pub fn with_constraint(mut self, constraint: ConnectorConfigurationConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    #[must_use]
    pub fn deprecated(mut self, message: impl Into<String>) -> Self {
        self.deprecation = Some(message.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn value_kind(&self) -> ConnectorConfigurationValueKind {
        self.value_kind
    }

    pub const fn requirement(&self) -> &ConnectorConfigurationRequirement {
        &self.requirement
    }

    pub fn documentation(&self) -> &str {
        &self.documentation
    }

    pub fn constraints(&self) -> &[ConnectorConfigurationConstraint] {
        &self.constraints
    }

    pub fn deprecation(&self) -> Option<&str> {
        self.deprecation.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorConfigurationSchema {
    revision: u32,
    fields: Vec<ConnectorConfigurationField>,
}

impl ConnectorConfigurationSchema {
    pub fn new(
        revision: u32,
        fields: Vec<ConnectorConfigurationField>,
    ) -> Result<Self, ConnectorConfigurationError> {
        let schema = Self { revision, fields };
        schema.validate_schema()?;
        Ok(schema)
    }

    pub const fn revision(&self) -> u32 {
        self.revision
    }

    pub fn fields(&self) -> &[ConnectorConfigurationField] {
        &self.fields
    }

    pub fn field(&self, name: &str) -> Option<&ConnectorConfigurationField> {
        self.fields.iter().find(|field| field.name == name)
    }

    pub fn resolve(
        &self,
        configuration: &ConnectorConfiguration,
    ) -> Result<ResolvedConnectorConfiguration, ConnectorConfigurationError> {
        if configuration.len() > MAX_CONNECTOR_CONFIGURATION_FIELDS {
            return Err(ConnectorConfigurationError::new(
                ConnectorConfigurationErrorCode::TooManyFields,
                None,
                "connector configuration exceeds the field limit",
            ));
        }
        for (name, _) in configuration.iter() {
            if self.field(name).is_none() {
                return Err(ConnectorConfigurationError::new(
                    ConnectorConfigurationErrorCode::UnknownField,
                    Some(name),
                    "connector configuration contains an unknown field",
                ));
            }
        }

        let mut resolved = ConnectorConfiguration::new();
        for field in &self.fields {
            let value = match configuration.get(&field.name) {
                Some(value) => value.clone(),
                None => match &field.requirement {
                    ConnectorConfigurationRequirement::Required => {
                        return Err(ConnectorConfigurationError::new(
                            ConnectorConfigurationErrorCode::MissingRequiredField,
                            Some(&field.name),
                            "required connector configuration field is missing",
                        ));
                    }
                    ConnectorConfigurationRequirement::Optional => continue,
                    ConnectorConfigurationRequirement::Default(value) => value.clone(),
                },
            };
            validate_value(field, &value)?;
            resolved.insert(field.name.clone(), value);
        }
        Ok(ResolvedConnectorConfiguration(resolved))
    }

    pub(crate) fn resolve_node_config(
        &self,
        configuration: &NodeConfig,
    ) -> Result<ResolvedConnectorConfiguration, ConnectorConfigurationError> {
        let mut typed = ConnectorConfiguration::new();
        for (name, encoded) in configuration.iter() {
            let field = self.field(name).ok_or_else(|| {
                ConnectorConfigurationError::new(
                    ConnectorConfigurationErrorCode::UnknownField,
                    Some(name),
                    "connector configuration contains an unknown field",
                )
            })?;
            let value =
                parse_encoded_value(field.value_kind, encoded, configuration.is_sensitive(name))
                    .map_err(|code| {
                        ConnectorConfigurationError::new(
                            code,
                            Some(name),
                            "connector configuration value has the wrong representation",
                        )
                    })?;
            typed.insert(name.to_owned(), value);
        }
        self.resolve(&typed)
    }

    fn validate_schema(&self) -> Result<(), ConnectorConfigurationError> {
        if self.revision == 0 {
            return Err(ConnectorConfigurationError::new(
                ConnectorConfigurationErrorCode::InvalidSchema,
                None,
                "connector configuration schema revision must be non-zero",
            ));
        }
        if self.fields.len() > MAX_CONNECTOR_CONFIGURATION_FIELDS {
            return Err(ConnectorConfigurationError::new(
                ConnectorConfigurationErrorCode::TooManyFields,
                None,
                "connector configuration schema exceeds the field limit",
            ));
        }
        let mut names = BTreeSet::new();
        for field in &self.fields {
            if field.name.trim().is_empty() || field.documentation.trim().is_empty() {
                return Err(ConnectorConfigurationError::new(
                    ConnectorConfigurationErrorCode::InvalidSchema,
                    Some(&field.name),
                    "connector configuration fields require a name and documentation",
                ));
            }
            if field.name.len() > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES
                || field.documentation.len() > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES
                || field
                    .deprecation
                    .as_ref()
                    .is_some_and(|message| message.len() > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES)
                || field.constraints.len() > MAX_CONNECTOR_CONFIGURATION_FIELDS
            {
                return Err(ConnectorConfigurationError::new(
                    ConnectorConfigurationErrorCode::InvalidSchema,
                    Some(&field.name),
                    "connector configuration field metadata exceeds a finite limit",
                ));
            }
            if !names.insert(field.name.clone()) {
                return Err(ConnectorConfigurationError::new(
                    ConnectorConfigurationErrorCode::DuplicateField,
                    Some(&field.name),
                    "connector configuration schema contains a duplicate field",
                ));
            }
            if let ConnectorConfigurationRequirement::Default(value) = &field.requirement {
                if field.value_kind == ConnectorConfigurationValueKind::Secret {
                    return Err(ConnectorConfigurationError::new(
                        ConnectorConfigurationErrorCode::SecretDefaultForbidden,
                        Some(&field.name),
                        "secret connector configuration fields cannot have defaults",
                    ));
                }
                validate_value(field, value)?;
            }
            validate_constraints(field)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedConnectorConfiguration(ConnectorConfiguration);

impl ResolvedConnectorConfiguration {
    pub fn get(&self, key: &str) -> Option<&ConnectorConfigurationValue> {
        self.0.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &ConnectorConfigurationValue)> {
        self.0.iter()
    }

    pub(crate) fn into_endpoint_configuration(self) -> EndpointConfiguration {
        let mut endpoint = EndpointConfiguration::new();
        for (key, value) in self.0.values {
            endpoint = match value {
                ConnectorConfigurationValue::Secret(secret) => {
                    endpoint.with_sensitive(key, secret.expose_secret())
                }
                value => endpoint.with(key, value.encoded()),
            };
        }
        endpoint
    }
}

fn parse_encoded_value(
    kind: ConnectorConfigurationValueKind,
    encoded: &str,
    sensitive: bool,
) -> Result<ConnectorConfigurationValue, ConnectorConfigurationErrorCode> {
    match kind {
        ConnectorConfigurationValueKind::Text if !sensitive => {
            Ok(ConnectorConfigurationValue::Text(encoded.to_owned()))
        }
        ConnectorConfigurationValueKind::Boolean if !sensitive => encoded
            .parse()
            .map(ConnectorConfigurationValue::Boolean)
            .map_err(|_| ConnectorConfigurationErrorCode::InvalidValue),
        ConnectorConfigurationValueKind::SignedInteger if !sensitive => encoded
            .parse()
            .map(ConnectorConfigurationValue::SignedInteger)
            .map_err(|_| ConnectorConfigurationErrorCode::InvalidValue),
        ConnectorConfigurationValueKind::UnsignedInteger if !sensitive => encoded
            .parse()
            .map(ConnectorConfigurationValue::UnsignedInteger)
            .map_err(|_| ConnectorConfigurationErrorCode::InvalidValue),
        ConnectorConfigurationValueKind::DurationMilliseconds if !sensitive => encoded
            .parse()
            .map(ConnectorConfigurationValue::DurationMilliseconds)
            .map_err(|_| ConnectorConfigurationErrorCode::InvalidValue),
        ConnectorConfigurationValueKind::ByteCount if !sensitive => encoded
            .parse()
            .map(ConnectorConfigurationValue::ByteCount)
            .map_err(|_| ConnectorConfigurationErrorCode::InvalidValue),
        ConnectorConfigurationValueKind::Secret if sensitive => ConnectorSecret::new(encoded)
            .map(ConnectorConfigurationValue::Secret)
            .map_err(|error| error.code()),
        ConnectorConfigurationValueKind::Secret => {
            Err(ConnectorConfigurationErrorCode::SecretClassificationMismatch)
        }
        _ => Err(ConnectorConfigurationErrorCode::UnexpectedSensitiveValue),
    }
}

fn validate_constraints(
    field: &ConnectorConfigurationField,
) -> Result<(), ConnectorConfigurationError> {
    for constraint in &field.constraints {
        let valid = match constraint {
            ConnectorConfigurationConstraint::NonEmpty => matches!(
                field.value_kind,
                ConnectorConfigurationValueKind::Text | ConnectorConfigurationValueKind::Secret
            ),
            ConnectorConfigurationConstraint::TextLengthBytes { minimum, maximum } => {
                matches!(
                    field.value_kind,
                    ConnectorConfigurationValueKind::Text | ConnectorConfigurationValueKind::Secret
                ) && minimum <= maximum
                    && *maximum <= MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES
            }
            ConnectorConfigurationConstraint::SignedRange { minimum, maximum } => {
                field.value_kind == ConnectorConfigurationValueKind::SignedInteger
                    && minimum <= maximum
            }
            ConnectorConfigurationConstraint::UnsignedRange { minimum, maximum } => {
                matches!(
                    field.value_kind,
                    ConnectorConfigurationValueKind::UnsignedInteger
                        | ConnectorConfigurationValueKind::DurationMilliseconds
                        | ConnectorConfigurationValueKind::ByteCount
                ) && minimum <= maximum
            }
            ConnectorConfigurationConstraint::OneOf(values) => {
                field.value_kind == ConnectorConfigurationValueKind::Text
                    && !values.is_empty()
                    && values.len() <= MAX_CONNECTOR_CONFIGURATION_FIELDS
                    && values.iter().all(|value| {
                        !value.is_empty() && value.len() <= MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES
                    })
                    && values.iter().collect::<BTreeSet<_>>().len() == values.len()
            }
        };
        if !valid {
            return Err(ConnectorConfigurationError::new(
                ConnectorConfigurationErrorCode::InvalidSchema,
                Some(&field.name),
                "connector configuration constraint does not match its field type",
            ));
        }
    }
    Ok(())
}

fn validate_value(
    field: &ConnectorConfigurationField,
    value: &ConnectorConfigurationValue,
) -> Result<(), ConnectorConfigurationError> {
    if field.value_kind != value.kind() {
        return Err(ConnectorConfigurationError::new(
            ConnectorConfigurationErrorCode::WrongType,
            Some(&field.name),
            "connector configuration value has the wrong type",
        ));
    }
    if value.text_bytes() > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES {
        return Err(ConnectorConfigurationError::new(
            ConnectorConfigurationErrorCode::ValueTooLarge,
            Some(&field.name),
            "connector configuration value exceeds the byte limit",
        ));
    }
    for constraint in &field.constraints {
        let valid = match (constraint, value) {
            (
                ConnectorConfigurationConstraint::NonEmpty,
                ConnectorConfigurationValue::Text(value),
            ) => !value.is_empty(),
            (
                ConnectorConfigurationConstraint::NonEmpty,
                ConnectorConfigurationValue::Secret(value),
            ) => !value.expose_secret().is_empty(),
            (
                ConnectorConfigurationConstraint::TextLengthBytes { minimum, maximum },
                ConnectorConfigurationValue::Text(value),
            ) => (*minimum..=*maximum).contains(&value.len()),
            (
                ConnectorConfigurationConstraint::TextLengthBytes { minimum, maximum },
                ConnectorConfigurationValue::Secret(value),
            ) => (*minimum..=*maximum).contains(&value.expose_secret().len()),
            (
                ConnectorConfigurationConstraint::SignedRange { minimum, maximum },
                ConnectorConfigurationValue::SignedInteger(value),
            ) => (*minimum..=*maximum).contains(value),
            (
                ConnectorConfigurationConstraint::UnsignedRange { minimum, maximum },
                ConnectorConfigurationValue::UnsignedInteger(value)
                | ConnectorConfigurationValue::DurationMilliseconds(value)
                | ConnectorConfigurationValue::ByteCount(value),
            ) => (*minimum..=*maximum).contains(value),
            (
                ConnectorConfigurationConstraint::OneOf(values),
                ConnectorConfigurationValue::Text(value),
            ) => values.iter().any(|candidate| candidate == value),
            _ => false,
        };
        if !valid {
            return Err(ConnectorConfigurationError::new(
                ConnectorConfigurationErrorCode::ConstraintViolation,
                Some(&field.name),
                "connector configuration value violates its declared constraint",
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorConfigurationErrorCode {
    InvalidSchema,
    DuplicateField,
    TooManyFields,
    UnknownField,
    MissingRequiredField,
    WrongType,
    InvalidValue,
    ConstraintViolation,
    ValueTooLarge,
    EmptySecret,
    SecretDefaultForbidden,
    SecretClassificationMismatch,
    UnexpectedSensitiveValue,
}

impl ConnectorConfigurationErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidSchema => "connector.configuration.invalid_schema",
            Self::DuplicateField => "connector.configuration.duplicate_field",
            Self::TooManyFields => "connector.configuration.too_many_fields",
            Self::UnknownField => "connector.configuration.unknown_field",
            Self::MissingRequiredField => "connector.configuration.missing_required_field",
            Self::WrongType => "connector.configuration.wrong_type",
            Self::InvalidValue => "connector.configuration.invalid_value",
            Self::ConstraintViolation => "connector.configuration.constraint_violation",
            Self::ValueTooLarge => "connector.configuration.value_too_large",
            Self::EmptySecret => "connector.configuration.empty_secret",
            Self::SecretDefaultForbidden => "connector.configuration.secret_default_forbidden",
            Self::SecretClassificationMismatch => {
                "connector.configuration.secret_classification_mismatch"
            }
            Self::UnexpectedSensitiveValue => "connector.configuration.unexpected_sensitive_value",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}", code = .code.as_str())]
pub struct ConnectorConfigurationError {
    code: ConnectorConfigurationErrorCode,
    field: Option<String>,
    message: String,
}

impl ConnectorConfigurationError {
    pub const fn code(&self) -> ConnectorConfigurationErrorCode {
        self.code
    }

    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(
        code: ConnectorConfigurationErrorCode,
        field: Option<&str>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            field: field.map(str::to_owned),
            message: message.into(),
        }
    }
}
