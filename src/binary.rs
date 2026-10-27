use std::any::Any;

use crate::error::UserNotificationsError;
use crate::ffi::abi::{UNBufferRaw, UNSliceRaw, UNStringRaw};

pub trait Record: Sized {
    type Raw: Copy + 'static;

    fn raw(&self, arena: &mut Arena) -> Self::Raw;
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError>;
}

#[derive(Default)]
pub struct Arena {
    values: Vec<Box<dyn Any>>,
}

impl Arena {
    pub fn keep<T: 'static>(&mut self, value: T) -> *const T {
        let value = Box::new(value);
        let pointer = &raw const *value;
        self.values.push(value);
        pointer
    }

    pub fn array<T: 'static>(&mut self, values: Vec<T>) -> UNSliceRaw {
        let raw = UNSliceRaw {
            data: values.as_ptr().cast(),
            len: values.len(),
        };
        self.values.push(Box::new(values));
        raw
    }
}

pub struct Input {
    _arena: Arena,
    raw: UNBufferRaw,
}

impl Input {
    pub fn new<T: Record>(value: &T) -> Self {
        let mut arena = Arena::default();
        let record = value.raw(&mut arena);
        let data = arena.keep(record).cast();
        Self {
            _arena: arena,
            raw: UNBufferRaw {
                data,
                size: core::mem::size_of::<T::Raw>(),
                owner: core::ptr::null_mut(),
                release: None,
            },
        }
    }

    pub const fn as_ptr(&self) -> *const UNBufferRaw {
        &raw const self.raw
    }

    pub fn into_owned(self) -> *mut UNBufferRaw {
        let pointer = Box::into_raw(Box::new(self));
        unsafe {
            (*pointer).raw.owner = pointer.cast();
            (*pointer).raw.release = Some(release_input);
            &raw mut (*pointer).raw
        }
    }
}

unsafe extern "C" fn release_input(pointer: *mut UNBufferRaw) {
    if let Some(raw) = unsafe { pointer.as_ref() } {
        drop(unsafe { Box::from_raw(raw.owner.cast::<Input>()) });
    }
}

pub fn invalid_data() -> UserNotificationsError {
    UserNotificationsError::FrameworkError("the bridge returned invalid binary data".into())
}

pub unsafe fn slice<'a, T>(raw: UNSliceRaw) -> Result<&'a [T], UserNotificationsError> {
    if raw.len == 0 {
        return Ok(&[]);
    }
    if raw.data.is_null()
        || raw.len > isize::MAX as usize / core::mem::size_of::<T>()
        || raw.data as usize % core::mem::align_of::<T>() != 0
    {
        return Err(invalid_data());
    }
    Ok(unsafe { core::slice::from_raw_parts(raw.data.cast(), raw.len) })
}

pub unsafe fn read<T: Record>(pointer: *const UNBufferRaw) -> Result<T, UserNotificationsError> {
    let raw = unsafe { pointer.as_ref() }.ok_or_else(invalid_data)?;
    if raw.data.is_null()
        || raw.size != core::mem::size_of::<T::Raw>()
        || raw.data as usize % core::mem::align_of::<T::Raw>() != 0
    {
        return Err(invalid_data());
    }
    unsafe { T::read(&*raw.data.cast()) }
}

struct Output(*mut UNBufferRaw);

impl Drop for Output {
    fn drop(&mut self) {
        if let Some(raw) = unsafe { self.0.as_ref() } {
            if let Some(release) = raw.release {
                unsafe { release(self.0) };
            }
        }
    }
}

pub fn decode_binary<T: Record>(pointer: *mut UNBufferRaw) -> Result<T, UserNotificationsError> {
    let output = Output(pointer);
    unsafe { read(output.0) }
}

impl Record for String {
    type Raw = UNStringRaw;

    fn raw(&self, arena: &mut Arena) -> Self::Raw {
        let bytes = arena.array(self.as_bytes().to_vec());
        UNStringRaw {
            data: bytes.data.cast(),
            len: bytes.len,
        }
    }

    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        let bytes = unsafe {
            slice::<u8>(UNSliceRaw {
                data: raw.data.cast(),
                len: raw.len,
            })
        }?;
        core::str::from_utf8(bytes)
            .map(str::to_owned)
            .map_err(|_| invalid_data())
    }
}

impl<T: Record> Record for Vec<T> {
    type Raw = UNSliceRaw;

    fn raw(&self, arena: &mut Arena) -> Self::Raw {
        let values = self.iter().map(|value| value.raw(arena)).collect();
        arena.array(values)
    }

    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        unsafe { slice::<T::Raw>(*raw) }?
            .iter()
            .map(|value| unsafe { T::read(value) })
            .collect()
    }
}
