use core::ops::{Deref, DerefMut};
use std::mem::ManuallyDrop;

/// Creates a new instance of `DropOwned` containing
/// the passed `val`.
#[inline]
pub const fn drop_owned<T: OwnedDroppable>(val: T) -> DropOwned<T> {
    DropOwned::new(val)
}

/// This trait has to be implemented for types that
/// can be dropped ownedly.
pub trait OwnedDroppable: Sized {
    /// This method is called once the `OwnedDrop`
    /// got dropped and provides the dropped instance to
    /// the implementor.
    fn drop_owned(self);
}

/// Once this type gets dropped, the contained value
/// is passed to the `drop_owned` function it has to implement.
pub struct DropOwned<T: OwnedDroppable>(ManuallyDrop<T>);

impl<T: OwnedDroppable> DropOwned<T> {
    /// Creates a new instance of `DropOwned` containing
    /// the passed `val`.
    #[inline]
    pub const fn new(val: T) -> Self {
        Self(ManuallyDrop::new(val))
    }

    /// Consumes the `DropOwned` to produces its inner value
    #[inline]
    pub fn into_inner(mut slot: Self) -> T {
        // SAFETY this `ManuallyDrop` will never get used again since we took ownership of it
        // and will now drop it
        unsafe { ManuallyDrop::take(&mut slot.0) }
    }
}

impl<T: OwnedDroppable> From<T> for DropOwned<T> {
    #[inline]
    fn from(val: T) -> Self {
        DropOwned::new(val)
    }
}

impl<T: OwnedDroppable> Deref for DropOwned<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: OwnedDroppable> DerefMut for DropOwned<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl<T: OwnedDroppable> Drop for DropOwned<T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY this `ManuallyDrop` will never get used again since we are inside the destructor
        unsafe { ManuallyDrop::take(&mut self.0) }.drop_owned();
    }
}
