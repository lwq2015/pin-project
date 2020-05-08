use pin_project::pin_project;
#[pin(__private())]
pub struct Struct<T, U> {
    #[pin]
    pub pinned: T,
    pub unpinned: U,
}
#[doc(hidden)]
#[allow(clippy::mut_mut)]
#[allow(dead_code)]
pub(crate) struct __StructProjection<'pin, T, U>
where
    Struct<T, U>: 'pin,
{
    pub pinned: ::pin_project::__reexport::pin::Pin<&'pin mut (T)>,
    pub unpinned: &'pin mut (U),
}
#[doc(hidden)]
#[allow(dead_code)]
pub(crate) struct __StructProjectionRef<'pin, T, U>
where
    Struct<T, U>: 'pin,
{
    pub pinned: ::pin_project::__reexport::pin::Pin<&'pin (T)>,
    pub unpinned: &'pin (U),
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
const __SCOPE_Struct: () = {
    impl<T, U> Struct<T, U> {
        pub(crate) fn project<'pin>(
            self: ::pin_project::__reexport::pin::Pin<&'pin mut Self>,
        ) -> __StructProjection<'pin, T, U> {
            unsafe {
                let Self { pinned, unpinned } = self.get_unchecked_mut();
                __StructProjection {
                    pinned: ::pin_project::__reexport::pin::Pin::new_unchecked(pinned),
                    unpinned,
                }
            }
        }
        pub(crate) fn project_ref<'pin>(
            self: ::pin_project::__reexport::pin::Pin<&'pin Self>,
        ) -> __StructProjectionRef<'pin, T, U> {
            unsafe {
                let Self { pinned, unpinned } = self.get_ref();
                __StructProjectionRef {
                    pinned: ::pin_project::__reexport::pin::Pin::new_unchecked(pinned),
                    unpinned,
                }
            }
        }
    }
    pub struct __Struct<'pin, T, U> {
        __pin_project_use_generics: ::pin_project::__private::AlwaysUnpin<'pin, (T, U)>,
        __field0: T,
    }
    impl<'pin, T, U> ::pin_project::__reexport::marker::Unpin for Struct<T, U> where
        __Struct<'pin, T, U>: ::pin_project::__reexport::marker::Unpin
    {
    }
    trait StructMustNotImplDrop {}
    #[allow(clippy::drop_bounds)]
    impl<T: ::pin_project::__reexport::ops::Drop> StructMustNotImplDrop for T {}
    #[allow(single_use_lifetimes)]
    impl<T, U> StructMustNotImplDrop for Struct<T, U> {}
    #[allow(single_use_lifetimes)]
    impl<T, U> ::pin_project::__private::PinnedDrop for Struct<T, U> {
        unsafe fn drop(self: ::pin_project::__reexport::pin::Pin<&mut Self>) {}
    }
    #[allow(single_use_lifetimes)]
    #[deny(safe_packed_borrows)]
    fn __assert_not_repr_packed<T, U>(val: &Struct<T, U>) {
        &val.pinned;
        &val.unpinned;
    }
};
fn main() {}