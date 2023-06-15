use libc;
use pingpong_sys::*;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::marker::PhantomData;

pub fn pong_str(ping: &str) -> String {
    let cstr = CString::new(ping)
        .expect("Cannot construct C string")
        .into_raw();
    let response = unsafe {
        let ptr = pong(cstr);
        CStr::from_ptr(ptr)
    };
    let pong = response.to_str().unwrap().to_owned();
    unsafe {
        libc::free(response.as_ptr() as *mut c_void);
    }
    pong
}

pub fn pong_char(ping: i8) -> String {
    let cstr = unsafe {
        let ptr = pong(ping as *mut i8);
        CStr::from_ptr(ptr)
    };
    let pong = cstr.to_str().unwrap().to_owned();
    unsafe {
        libc::free(cstr.as_ptr() as *mut c_void);
    }
    pong
}

pub struct Session {
    ptr: *mut t_session
}

pub struct Buffer<'a> {
    ptr: *mut t_buffer,
    _phantom: PhantomData<&'a t_session>,
}

impl<'a> Session {
    pub fn new() -> Self {
        let session_ptr = unsafe { start() };
        Session {
            ptr: session_ptr
        }
    }

    pub fn buffer(&self) -> &str {
        let response = unsafe {
            let ptr = buffer(self.ptr);
            CStr::from_ptr(ptr)
        };
        response.to_str().unwrap()
    }

    pub fn buffer_copy(&self) -> String {
        self.buffer().to_owned()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { stop(self.ptr) }
    }
}

unsafe impl Send for Session {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pong_str("test");
        assert_eq!(result, "Pong test");
    }

    #[test]
    #[should_panic(expected = "Cannot construct C string")]
    fn invalid_c_str() {
        let result = pong_str("str\0str");
        assert_eq!(result, "Pong test");
    }

    #[test]
    fn lifetime_of_str() {
        let mut str: &str = "default";
        {
            assert_eq!(str, "default");
            let session = Session::new();
            str = session.buffer();
            assert_eq!(str, "Buffer data");
        }
        //assert_eq!(str, "Buffer data");
    }
}
