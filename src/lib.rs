use std::mem;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

/// Creates a new instance of `DropOwned` containing
/// the passed `val`.
#[inline]
pub fn drop_owned<T: OwnedDroppable>(val: T) -> DropOwned<T> {
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
pub struct DropOwned<T: OwnedDroppable> {
    inner: MaybeUninit<T>,
}

impl<T: OwnedDroppable> DropOwned<T> {

    /// Creates a new instance of `DropOwned` containing
    /// the passed `val`.
    #[inline]
    pub fn new(val: T) -> Self {
        Self {
            inner: MaybeUninit::new(val),
        }
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

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe, as the only way for uninitialized data
        // to be present in `inner` is when this struct gets dropped
        // at which point `deref` can't ever possibly be called anymore.
        unsafe { self.inner.assume_init_ref() }
    }
}

impl<T: OwnedDroppable> DerefMut for DropOwned<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: This is safe, as the only way for uninitialized data
        // to be present in `inner` is when this struct gets dropped
        // at which point `deref_mut` can't ever possibly be called anymore.
        unsafe { self.inner.assume_init_mut() }
    }
}

impl<T: OwnedDroppable> Drop for DropOwned<T> {
    #[inline]
    fn drop(&mut self) {
        let owned = mem::replace(&mut self.inner, MaybeUninit::uninit());
        // SAFETY: This is safe because the previous inner value has to be
        // initialized because `DropOwnedMemCpy` can only be created with
        // an initialized value.
        unsafe { owned.assume_init() }.drop_owned();
    }
}
