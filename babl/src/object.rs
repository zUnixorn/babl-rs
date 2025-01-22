glib::wrapper! {
    pub struct Object(Boxed<ffi::Babl>);

    match fn {
        copy => |ptr| ptr as *mut _,
        free => |_ptr| (),
    }
}

