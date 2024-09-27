/// Functions that take `Function<T>` type as a parameter also
/// take a pointer to contextual data that you provide.
/// When your dispatch function is called, the pointer to that contextual
/// data is passed as the parameter to the function.
/// The pointer to the contextual data is passed unmodified to your
/// function and it is your responsibility to ensure that the pointer
/// is valid.
#[doc(alias = "dispatch_function_t")]
pub type Fn<T> = extern "C-unwind" fn(*mut T);
