use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::io;

unsafe extern "C" {
    fn msh_glob(
        pattern: *const c_char,
        emit: extern "C" fn(*const c_char, *mut c_void),
        context: *mut c_void,
    ) -> c_int;
}

extern "C" fn collect_match(path: *const c_char, context: *mut c_void) {
    let matches = unsafe { &mut *context.cast::<Vec<String>>() };
    let path = unsafe { CStr::from_ptr(path) };
    if let Ok(path) = path.to_str() {
        matches.push(path.to_string());
    }
}

pub fn expand(pattern: &str) -> io::Result<Vec<String>> {
    let pattern = CString::new(pattern)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;

    let mut matches = Vec::<String>::new();
    let status = unsafe {
        msh_glob(
            pattern.as_ptr(),
            collect_match,
            (&mut matches as *mut Vec<String>).cast(),
        )
    };

    match status {
        0 => (),
        1 => {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "glob ran out of memory",
            ));
        }
        _ => return Err(io::Error::other("glob could not read a directory")),
    }

    Ok(matches)
}
