use std::collections::BTreeSet;
use std::path::Path;

use crate::abi::executable_extension::ExecutableExtensionRegistration;
use crate::native_extension::{NativeExtensionLibrary, NativeExtensionLibraryError};
use crate::{EndpointExtensionRegistration, OperatorId, Session};

impl Session {
    /// Loads one trusted packaged native extension from an exact absolute path
    /// and atomically imports all of its source, operator, and endpoint
    /// registrations into this Session.
    ///
    /// The library must export `pks_extension_library_v1`. PocketStation
    /// canonicalizes the path, never uses ambient library search, validates
    /// every registration before mutating Session state, and retains the
    /// executable library until all callback contexts are destroyed.
    pub fn load_native_extension_library(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<NativeExtensionLibrary, NativeExtensionLibraryError> {
        let loaded = crate::native_extension::load_native_extension_library(path.as_ref())?;
        let mut source_registrations = self
            .source_registrations
            .lock()
            .map_err(|_| NativeExtensionLibraryError::registration_state_unavailable("source"))?;
        let mut operator_registrations = self
            .operator_registrations
            .lock()
            .map_err(|_| NativeExtensionLibraryError::registration_state_unavailable("operator"))?;
        let mut endpoint_extensions = self.endpoint_extensions.lock().map_err(|_| {
            NativeExtensionLibraryError::registration_state_unavailable("endpoint extension")
        })?;
        let endpoint_registrations = self.endpoint_registrations.lock().map_err(|_| {
            NativeExtensionLibraryError::registration_state_unavailable("endpoint driver")
        })?;

        let mut existing_ids = BTreeSet::new();
        existing_ids.extend(
            source_registrations
                .iter()
                .map(|factory| factory.manifest().source_type_id().as_str().to_owned()),
        );
        existing_ids.extend(
            operator_registrations
                .iter()
                .map(|factory| factory.manifest().operator_id().as_str().to_owned()),
        );
        existing_ids.extend(
            endpoint_extensions
                .iter()
                .map(|registration| registration.operator_id.as_str().to_owned()),
        );
        existing_ids.extend(
            endpoint_registrations
                .iter()
                .map(|registration| registration.operator_id.as_str().to_owned()),
        );
        for registration in &loaded.registrations {
            if existing_ids.contains(registration.id()) {
                return Err(NativeExtensionLibraryError::duplicate_registration(
                    registration.id(),
                ));
            }
        }

        let receipt = loaded.receipt;
        for registration in loaded.registrations {
            match registration {
                ExecutableExtensionRegistration::Source { factory, .. } => {
                    source_registrations.push(factory);
                }
                ExecutableExtensionRegistration::Operator { factory, .. } => {
                    operator_registrations.push(factory);
                }
                ExecutableExtensionRegistration::Endpoint {
                    id,
                    definition,
                    factory,
                } => endpoint_extensions.push(EndpointExtensionRegistration {
                    operator_id: OperatorId::new(id),
                    definition,
                    factory,
                }),
            }
        }
        Ok(receipt)
    }
}
