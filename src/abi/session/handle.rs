use crate::abi::session::abi::{PksSessionHandle, PksSessionHandleKind};
use crate::abi::session::error::AbiError;

#[derive(Debug)]
struct HandleSlot<T> {
    generation: u64,
    value: Option<T>,
}

impl<T> Default for HandleSlot<T> {
    fn default() -> Self {
        Self {
            generation: 1,
            value: None,
        }
    }
}

#[derive(Debug)]
pub struct HandleTable<T> {
    kind: PksSessionHandleKind,
    scope_id: u64,
    slots: Box<[HandleSlot<T>]>,
}

impl<T> HandleTable<T> {
    pub fn new(capacity_count: usize, kind: PksSessionHandleKind, scope_id: u64) -> Self {
        let mut slots = Vec::with_capacity(capacity_count);
        slots.resize_with(capacity_count, HandleSlot::default);
        Self {
            kind,
            scope_id,
            slots: slots.into_boxed_slice(),
        }
    }

    pub fn insert(&mut self, value: T) -> Result<PksSessionHandle, AbiError> {
        for (slot_index, slot) in self.slots.iter_mut().enumerate() {
            if slot.value.is_none() {
                let generation = slot.generation;
                slot.value = Some(value);
                return Ok(PksSessionHandle {
                    kind: self.kind,
                    slot_index: slot_index as u32,
                    generation,
                    scope_id: self.scope_id,
                });
            }
        }
        Err(AbiError::NoCapacity)
    }

    pub fn get(&self, handle: PksSessionHandle) -> Result<&T, AbiError> {
        let slot = self.slot(handle)?;
        match &slot.value {
            Some(value) => Ok(value),
            None => Err(AbiError::StaleHandle),
        }
    }

    pub fn get_mut(&mut self, handle: PksSessionHandle) -> Result<&mut T, AbiError> {
        let slot = self.slot_mut(handle)?;
        match &mut slot.value {
            Some(value) => Ok(value),
            None => Err(AbiError::StaleHandle),
        }
    }

    pub fn remove(&mut self, handle: PksSessionHandle) -> Result<T, AbiError> {
        let slot = self.slot_mut(handle)?;
        let value = slot.value.take().ok_or(AbiError::StaleHandle)?;
        slot.generation = slot.generation.wrapping_add(1).max(1);
        Ok(value)
    }

    pub fn active_count(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| slot.value.is_some())
            .count()
    }

    pub fn for_each_mut(&mut self, mut visit: impl FnMut(&mut T)) {
        for slot in &mut self.slots {
            if let Some(value) = slot.value.as_mut() {
                visit(value);
            }
        }
    }

    fn slot(&self, handle: PksSessionHandle) -> Result<&HandleSlot<T>, AbiError> {
        self.validate_owner(handle)?;
        let slot = self
            .slots
            .get(handle.slot_index as usize)
            .ok_or(AbiError::InvalidHandle)?;
        if slot.generation != handle.generation {
            return Err(AbiError::StaleHandle);
        }
        Ok(slot)
    }

    fn slot_mut(&mut self, handle: PksSessionHandle) -> Result<&mut HandleSlot<T>, AbiError> {
        self.validate_owner(handle)?;
        let slot = self
            .slots
            .get_mut(handle.slot_index as usize)
            .ok_or(AbiError::InvalidHandle)?;
        if slot.generation != handle.generation {
            return Err(AbiError::StaleHandle);
        }
        Ok(slot)
    }

    fn validate_owner(&self, handle: PksSessionHandle) -> Result<(), AbiError> {
        if handle.kind != self.kind {
            return Err(AbiError::InvalidHandle);
        }
        if handle.scope_id != self.scope_id {
            return Err(AbiError::ForeignHandle);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::abi::session::error::AbiError;

    use crate::abi::session::abi::PksSessionHandleKind;

    use super::HandleTable;

    #[test]
    fn given_removed_handle_when_lookup_then_stale_is_reported() {
        let mut table = HandleTable::new(1, PksSessionHandleKind::Session, 7);
        let handle = table.insert(7_u32).expect("insert");
        let value = table.remove(handle).expect("remove");

        assert_eq!(value, 7);
        assert_eq!(
            table.get(handle).expect_err("stale handle"),
            AbiError::StaleHandle
        );
    }

    #[test]
    fn given_full_table_when_insert_then_capacity_failure_is_returned() {
        let mut table = HandleTable::new(1, PksSessionHandleKind::Session, 7);
        let _ = table.insert(7_u32).expect("insert");

        assert_eq!(
            table.insert(8_u32).expect_err("full table"),
            AbiError::NoCapacity
        );
    }

    #[test]
    fn given_other_scope_when_lookup_then_foreign_handle_is_reported() {
        let mut first = HandleTable::new(1, PksSessionHandleKind::Session, 7);
        let second = HandleTable::<u32>::new(1, PksSessionHandleKind::Session, 8);
        let handle = first.insert(7_u32).expect("insert");

        assert_eq!(
            second.get(handle).expect_err("foreign handle"),
            AbiError::ForeignHandle
        );
    }
}
