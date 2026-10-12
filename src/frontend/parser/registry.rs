
 //! Deterministic, extensible registry for parser grammar extensions.
 //!
 //! # Responsibilities
 //! - Associate stable extension identities with extension handlers.
 //! - Route namespaced syntax dispatch keys to registered handlers.
 //! - Validate registry API compatibility and registration conflicts.
 //! - Provide deterministic iteration and immutable lookup after construction.
 //!
 //! # Non-responsibilities
 //! - Parsing tokens or source text.
 //! - Defining the canonical AST or source-span representation.
 //! - Resolving types, effects, capabilities, or resource requirements.
 //! - Selecting hardware, allocating devices, or lowering operations.
 //!
 //! # Integration
 //! Use `GrammarRegistry<dyn GrammarExtension>` when `extensions.rs` defines
 //! the repository's canonical `GrammarExtension` trait. The registry deliberately
 //! does not define that trait itself.
 //!
 //! Build the registry before parsing begins, validate all registrations, then
 //! share it immutably through `context.rs`. Each parse session should own its
 //! mutable parsing state independently.
 //!
 //! This module uses safe Rust only.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// Stable, namespaced identity for a registered extension.
///
/// Identities are case-sensitive. Whitespace and control characters are
/// rejected so accidental differences do not create visually ambiguous IDs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtensionId(String);

impl ExtensionId {
    /// Creates an extension ID such as `zamani.quantum` or `vendor.dialect`.
    pub fn new(value: impl Into<String>) -> Result<Self, RegistryError> {
        let value = value.into();
        validate_identifier("extension ID", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExtensionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A dispatch key identifying a grammar extension point.
///
/// `namespace` identifies the grammar family or owner; `name` identifies the
/// entry point within that namespace. For example:
///
/// - `("core", "item")`
/// - `("quantum", "operation")`
/// - `("hdl", "concurrent_statement")`
/// - `("vendor.example", "custom_declaration")`
///
/// Keys are data, not a hard-coded list of all supported language constructs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DispatchKey {
    namespace: String,
    name: String,
}

impl DispatchKey {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, RegistryError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier("dispatch namespace", &namespace)?;
        validate_identifier("dispatch name", &name)?;

        Ok(Self { namespace, name })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for DispatchKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/// Version of a registered extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtensionVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ExtensionVersion {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

/// Metadata required to register a grammar extension.
///
/// `registry_api_major` declares which major version of the registry contract
/// the extension was built against. It is checked by the builder rather than
/// silently accepting an incompatible extension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionMetadata {
    pub id: ExtensionId,
    pub version: ExtensionVersion,
    pub registry_api_major: u32,
    pub description: Option<String>,
}

impl ExtensionMetadata {
    pub fn new(
        id: ExtensionId,
        version: ExtensionVersion,
        registry_api_major: u32,
    ) -> Self {
        Self {
            id,
            version,
            registry_api_major,
            description: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Errors encountered while constructing or validating a registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    InvalidIdentifier {
        kind: &'static str,
        value: String,
        reason: &'static str,
    },
    EmptyDispatchKeys {
        extension: ExtensionId,
    },
    DuplicateExtension {
        extension: ExtensionId,
    },
    DuplicateDispatchKey {
        key: DispatchKey,
        existing_extension: ExtensionId,
    },
    DuplicateDispatchKeyInRegistration {
        key: DispatchKey,
        extension: ExtensionId,
    },
    IncompatibleRegistryApi {
        extension: ExtensionId,
        required_major: u32,
        provided_major: u32,
    },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind, value, reason } => {
                write!(f, "invalid {kind} {value:?}: {reason}")
            }
            Self::EmptyDispatchKeys { extension } => {
                write!(f, "extension {extension} has no dispatch keys")
            }
            Self::DuplicateExtension { extension } => {
                write!(f, "extension {extension} is already registered")
            }
            Self::DuplicateDispatchKey {
                key,
                existing_extension,
            } => write!(
                f,
                "dispatch key {key} is already owned by extension \
                 {existing_extension}"
            ),
            Self::DuplicateDispatchKeyInRegistration { key, extension } => {
                write!(
                    f,
                    "extension {extension} declares dispatch key {key} more than once"
                )
            }
            Self::IncompatibleRegistryApi {
                extension,
                required_major,
                provided_major,
            } => write!(
                f,
                "extension {extension} targets registry API major \
                 {provided_major}, but this registry requires major \
                 {required_major}"
            ),
        }
    }
}

impl Error for RegistryError {}

/// A successfully registered extension and its declared dispatch points.
///
/// The handler is shared through `Arc`, allowing immutable registry access
/// without requiring the handler type itself to implement `Clone`.
pub struct RegisteredExtension<E: ?Sized> {
    metadata: ExtensionMetadata,
    dispatch_keys: Vec<DispatchKey>,
    handler: Arc<E>,
}

impl<E: ?Sized> RegisteredExtension<E> {
    pub fn metadata(&self) -> &ExtensionMetadata {
        &self.metadata
    }

    pub fn dispatch_keys(&self) -> &[DispatchKey] {
        &self.dispatch_keys
    }

    pub fn handler(&self) -> &Arc<E> {
        &self.handler
    }
}

/// Mutable builder for a registry that will become immutable after `build`.
///
/// Registration is transactional: a failed registration does not partially
/// mutate the extension or dispatch maps.
pub struct GrammarRegistryBuilder<E: ?Sized> {
    required_api_major: u32,
    extensions: BTreeMap<ExtensionId, RegisteredExtension<E>>,
    dispatch: BTreeMap<DispatchKey, ExtensionId>,
}

impl<E: ?Sized> GrammarRegistryBuilder<E> {
    pub fn new(required_api_major: u32) -> Self {
        Self {
            required_api_major,
            extensions: BTreeMap::new(),
            dispatch: BTreeMap::new(),
        }
    }

    pub fn required_api_major(&self) -> u32 {
        self.required_api_major
    }

    /// Registers one extension against one or more dispatch keys.
    ///
    /// All keys must be unique both within this registration and across the
    /// registry. Conflicts are errors; there is no implicit priority-based
    /// override and no "last registration wins" behavior.
    pub fn register(
        &mut self,
        metadata: ExtensionMetadata,
        dispatch_keys: impl IntoIterator<Item = DispatchKey>,
        handler: Arc<E>,
    ) -> Result<(), RegistryError> {
        if self.extensions.contains_key(&metadata.id) {
            return Err(RegistryError::DuplicateExtension {
                extension: metadata.id,
            });
        }

        if metadata.registry_api_major != self.required_api_major {
            return Err(RegistryError::IncompatibleRegistryApi {
                extension: metadata.id,
                required_major: self.required_api_major,
                provided_major: metadata.registry_api_major,
            });
        }

        let extension_id = metadata.id.clone();
        let mut unique_keys = BTreeSet::new();

        for key in dispatch_keys {
            if !unique_keys.insert(key.clone()) {
                return Err(RegistryError::DuplicateDispatchKeyInRegistration {
                    key,
                    extension: extension_id,
                });
            }
        }

        if unique_keys.is_empty() {
            return Err(RegistryError::EmptyDispatchKeys {
                extension: extension_id,
            });
        }

        // Validate every conflict before mutating either map.
        for key in &unique_keys {
            if let Some(existing_extension) = self.dispatch.get(key) {
                return Err(RegistryError::DuplicateDispatchKey {
                    key: key.clone(),
                    existing_extension: existing_extension.clone(),
                });
            }
        }

        let dispatch_keys: Vec<DispatchKey> = unique_keys.into_iter().collect();

        for key in &dispatch_keys {
            self.dispatch.insert(key.clone(), metadata.id.clone());
        }

        self.extensions.insert(
            metadata.id.clone(),
            RegisteredExtension {
                metadata,
                dispatch_keys,
                handler,
            },
        );

        Ok(())
    }

    /// Checks internal consistency without consuming the builder.
    pub fn validate(&self) -> Result<(), RegistryError> {
        for (key, extension_id) in &self.dispatch {
            let Some(extension) = self.extensions.get(extension_id) else {
                // This state cannot be created through the public API. Keep
                // validation defensive if the implementation is later changed.
                return Err(RegistryError::InvalidIdentifier {
                    kind: "registry mapping",
                    value: format!("{key} -> {extension_id}"),
                    reason: "dispatch entry refers to a missing extension",
                });
            };

            if extension.dispatch_keys.binary_search(key).is_err() {
                return Err(RegistryError::InvalidIdentifier {
                    kind: "registry mapping",
                    value: format!("{key} -> {extension_id}"),
                    reason: "extension does not declare the mapped dispatch key",
                });
            }
        }

        for (extension_id, extension) in &self.extensions {
            if extension.metadata.id != *extension_id {
                return Err(RegistryError::InvalidIdentifier {
                    kind: "extension mapping",
                    value: extension_id.to_string(),
                    reason: "extension map key differs from metadata identity",
                });
            }

            if extension.metadata.registry_api_major != self.required_api_major {
                return Err(RegistryError::IncompatibleRegistryApi {
                    extension: extension_id.clone(),
                    required_major: self.required_api_major,
                    provided_major: extension.metadata.registry_api_major,
                });
            }

            for key in &extension.dispatch_keys {
                if self.dispatch.get(key) != Some(extension_id) {
                    return Err(RegistryError::InvalidIdentifier {
                        kind: "registry mapping",
                        value: format!("{key} -> {extension_id}"),
                        reason: "declared dispatch key has no matching registry entry",
                    });
                }
            }
        }

        Ok(())
    }

    /// Freezes the builder into an immutable registry.
    pub fn build(self) -> Result<GrammarRegistry<E>, RegistryError> {
        self.validate()?;

        Ok(GrammarRegistry {
            required_api_major: self.required_api_major,
            extensions: self.extensions,
            dispatch: self.dispatch,
        })
    }
}

/// Immutable registry used by parser contexts and grammar dispatch.
///
/// BTreeMap-backed lookup provides deterministic iteration and stable ordering.
/// Lookup is logarithmic in the number of registered keys; the registry does
/// not impose a fixed extension-count or hardware-capacity ceiling.
pub struct GrammarRegistry<E: ?Sized> {
    required_api_major: u32,
    extensions: BTreeMap<ExtensionId, RegisteredExtension<E>>,
    dispatch: BTreeMap<DispatchKey, ExtensionId>,
}

impl<E: ?Sized> GrammarRegistry<E> {
    pub fn builder(required_api_major: u32) -> GrammarRegistryBuilder<E> {
        GrammarRegistryBuilder::new(required_api_major)
    }

    pub fn required_api_major(&self) -> u32 {
        self.required_api_major
    }

    pub fn extension_count(&self) -> usize {
        self.extensions.len()
    }

    pub fn dispatch_key_count(&self) -> usize {
        self.dispatch.len()
    }

    pub fn is_empty(&self) -> bool {
        self.extensions.is_empty()
    }

    pub fn contains_extension(&self, id: &ExtensionId) -> bool {
        self.extensions.contains_key(id)
    }

    pub fn extension(&self, id: &ExtensionId) -> Option<&RegisteredExtension<E>> {
        self.extensions.get(id)
    }

    pub fn metadata(&self, id: &ExtensionId) -> Option<&ExtensionMetadata> {
        self.extensions.get(id).map(RegisteredExtension::metadata)
    }

    /// Resolves a dispatch key to its registered extension handler.
    pub fn resolve(&self, key: &DispatchKey) -> Option<&Arc<E>> {
        let extension_id = self.dispatch.get(key)?;
        self.extensions
            .get(extension_id)
            .map(RegisteredExtension::handler)
    }

    /// Returns the extension identity that owns a dispatch key.
    pub fn owner_of(&self, key: &DispatchKey) -> Option<&ExtensionId> {
        self.dispatch.get(key)
    }

    /// Iterates extensions in deterministic identity order.
    pub fn extensions(
        &self,
    ) -> impl ExactSizeIterator<Item = (&ExtensionId, &RegisteredExtension<E>)> {
        self.extensions.iter()
    }

    /// Iterates dispatch mappings in deterministic key order.
    pub fn dispatch_entries(
        &self,
    ) -> impl ExactSizeIterator<Item = (&DispatchKey, &ExtensionId)> {
        self.dispatch.iter()
    }
}

fn validate_identifier(
    kind: &'static str,
    value: &str,
) -> Result<(), RegistryError> {
    let reason = if value.is_empty() {
        Some("must not be empty")
    } else if value.trim() != value {
        Some("must not have leading or trailing whitespace")
    } else if value.chars().any(char::is_whitespace) {
        Some("must not contain whitespace")
    } else if value.chars().any(char::is_control) {
        Some("must not contain control characters")
    } else {
        None
    };

    if let Some(reason) = reason {
        return Err(RegistryError::InvalidIdentifier {
            kind,
            value: value.to_owned(),
            reason,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn id(value: &str) -> ExtensionId {
        ExtensionId::new(value).expect("valid test extension ID")
    }

    fn key(namespace: &str, name: &str) -> DispatchKey {
        DispatchKey::new(namespace, name).expect("valid test dispatch key")
    }

    fn metadata(extension_id: &str) -> ExtensionMetadata {
        ExtensionMetadata::new(
            id(extension_id),
            ExtensionVersion::new(1, 0, 0),
            1,
        )
    }

    #[test]
    fn registers_and_resolves_handler() {
        let handler = Arc::new(String::from("quantum handler"));
        let mut builder = GrammarRegistry::builder::<String>(1);

        builder
            .register(
                metadata("zamani.quantum"),
                [key("quantum", "operation")],
                handler.clone(),
            )
            .expect("registration succeeds");

        let registry = builder.build().expect("registry is valid");
        let resolved = registry
            .resolve(&key("quantum", "operation"))
            .expect("handler resolves");

        assert_eq!(resolved.as_str(), "quantum handler");
        assert_eq!(registry.extension_count(), 1);
        assert_eq!(registry.dispatch_key_count(), 1);
    }

    #[test]
    fn permits_one_extension_to_own_multiple_keys() {
        let mut builder = GrammarRegistry::builder::<String>(1);

        builder
            .register(
                metadata("zamani.classical"),
                [
                    key("classical", "declaration"),
                    key("classical", "statement"),
                ],
                Arc::new(String::from("classical handler")),
            )
            .expect("registration succeeds");

        let registry = builder.build().expect("registry is valid");

        assert_eq!(registry.extension_count(), 1);
        assert_eq!(registry.dispatch_key_count(), 2);
        assert_eq!(
            registry.owner_of(&key("classical", "statement")),
            Some(&id("zamani.classical"))
        );
    }

    #[test]
    fn rejects_duplicate_extension_ids() {
        let mut builder = GrammarRegistry::builder::<String>(1);

        builder
            .register(
                metadata("zamani.quantum"),
                [key("quantum", "operation")],
                Arc::new(String::from("first")),
            )
            .expect("first registration succeeds");

        let result = builder.register(
            metadata("zamani.quantum"),
            [key("quantum", "other_operation")],
            Arc::new(String::from("second")),
        );

        assert!(matches!(
            result,
            Err(RegistryError::DuplicateExtension { .. })
        ));
        assert_eq!(builder.extensions.len(), 1);
        assert_eq!(builder.dispatch.len(), 1);
    }

    #[test]
    fn rejects_dispatch_conflicts_without_partial_registration() {
        let mut builder = GrammarRegistry::builder::<String>(1);

        builder
            .register(
                metadata("zamani.classical"),
                [key("core", "item")],
                Arc::new(String::from("classical")),
            )
            .expect("first registration succeeds");

        let result = builder.register(
            metadata("zamani.quantum"),
            [
                key("quantum", "operation"),
                key("core", "item"),
            ],
            Arc::new(String::from("quantum")),
        );

        assert!(matches!(
            result,
            Err(RegistryError::DuplicateDispatchKey { .. })
        ));
        assert_eq!(builder.extensions.len(), 1);
        assert_eq!(builder.dispatch.len(), 1);
        assert!(!builder
            .extensions
            .contains_key(&id("zamani.quantum")));
        assert!(!builder
            .dispatch
            .contains_key(&key("quantum", "operation")));
    }

    #[test]
    fn rejects_duplicate_keys_within_one_registration() {
        let mut builder = GrammarRegistry::builder::<String>(1);
        let duplicate = key("core", "item");

        let result = builder.register(
            metadata("zamani.test"),
            [duplicate.clone(), duplicate],
            Arc::new(String::from("handler")),
        );

        assert!(matches!(
            result,
            Err(RegistryError::DuplicateDispatchKeyInRegistration { .. })
        ));
        assert!(builder.extensions.is_empty());
        assert!(builder.dispatch.is_empty());
    }

    #[test]
    fn rejects_extensions_with_no_dispatch_keys() {
        let mut builder = GrammarRegistry::builder::<String>(1);

        let result = builder.register(
            metadata("zamani.empty"),
            [],
            Arc::new(String::from("handler")),
        );

        assert!(matches!(
            result,
            Err(RegistryError::EmptyDispatchKeys { .. })
        ));
    }

    #[test]
    fn rejects_incompatible_registry_api_major() {
        let mut builder = GrammarRegistry::builder::<String>(2);

        let result = builder.register(
            metadata("vendor.extension"),
            [key("vendor.example", "declaration")],
            Arc::new(String::from("handler")),
        );

        assert!(matches!(
            result,
            Err(RegistryError::IncompatibleRegistryApi {
                required_major: 2,
                provided_major: 1,
                ..
            })
        ));
    }

    #[test]
    fn rejects_empty_or_ambiguous_identifiers() {
        assert!(ExtensionId::new("").is_err());
        assert!(ExtensionId::new(" leading").is_err());
        assert!(ExtensionId::new("embedded whitespace").is_err());
        assert!(DispatchKey::new("core", "").is_err());
        assert!(DispatchKey::new("core namespace", "item").is_err());
    }

    #[test]
    fn registry_iteration_is_deterministic() {
        let mut builder = GrammarRegistry::builder::<String>(1);

        builder
            .register(
                metadata("z.extension"),
                [key("z", "item")],
                Arc::new(String::from("z")),
            )
            .expect("registration succeeds");

        builder
            .register(
                metadata("a.extension"),
                [key("a", "item")],
                Arc::new(String::from("a")),
            )
            .expect("registration succeeds");

        let registry = builder.build().expect("registry is valid");
        let extension_ids: Vec<&str> = registry
            .extensions()
            .map(|(extension_id, _)| extension_id.as_str())
            .collect();

        assert_eq!(extension_ids, ["a.extension", "z.extension"]);

        let dispatch_keys: Vec<String> = registry
            .dispatch_entries()
            .map(|(dispatch_key, _)| dispatch_key.to_string())
            .collect();

        assert_eq!(dispatch_keys, ["a::item", "z::item"]);
    }

    #[test]
    fn registry_supports_trait_objects() {
        trait TestHandler: Send + Sync {
            fn name(&self) -> &'static str;
        }

        struct Handler;

        impl TestHandler for Handler {
            fn name(&self) -> &'static str {
                "test"
            }
        }

        let handler: Arc<dyn TestHandler> = Arc::new(Handler);
        let mut builder = GrammarRegistry::<dyn TestHandler>::builder(1);

        builder
            .register(
                metadata("test.handler"),
                [key("test", "entry")],
                handler,
            )
            .expect("trait-object registration succeeds");

        let registry = builder.build().expect("registry is valid");
        assert_eq!(
            registry
                .resolve(&key("test", "entry"))
                .expect("handler resolves")
                .name(),
            "test"
        );
    }
}
