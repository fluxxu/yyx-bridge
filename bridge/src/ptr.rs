use std::ffi::c_void;
use std::mem::transmute;
use std::slice;

pub trait PtrExt {
  unsafe fn ptr_get_element<T>(self, offset: usize) -> T;
  unsafe fn ptr_get_vtable<'a>(self, len: usize) -> VTable<'a>;
  unsafe fn ptr_get_vtable_fn<F>(self, index: usize) -> Option<*const F>;
}

impl<S> PtrExt for *const S {
  unsafe fn ptr_get_element<T>(self, offset: usize) -> T {
    let p: *const u8 = transmute(self);
    let p: *const T = transmute(p.offset(offset as isize));
    p.read()
  }

  unsafe fn ptr_get_vtable<'a>(self, len: usize) -> VTable<'a> {
    let p: *const *const c_void = *transmute::<_, *const *const *const c_void>(self);
    VTable {
      entries: slice::from_raw_parts(p, len),
    }
  }

  unsafe fn ptr_get_vtable_fn<F>(self, index: usize) -> Option<*const F> {
    self.ptr_get_vtable(index + 1).get_fn(index)
  }
}

pub struct VTable<'a> {
  entries: &'a [*const c_void],
}

impl<'a> VTable<'a> {
  pub fn get_fn<F>(&self, i: usize) -> Option<*const F> {
    self.entries.get(i).map(|p| unsafe { transmute(p) })
  }
}
