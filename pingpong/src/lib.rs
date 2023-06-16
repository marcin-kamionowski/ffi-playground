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
    ptr: *mut t_session,
}

pub struct Buffer<'a> {
    ptr: *mut t_buffer,
    _phantom: PhantomData<&'a t_buffer>,
}

impl<'a> Session {
    pub fn new() -> Self {
        let session_ptr = unsafe { start() };
        Session { ptr: session_ptr }
    }

    pub fn buffer(&self) -> Buffer {
        let ptr = unsafe { session_buffer(self.ptr) };
        Buffer::from_raw(ptr)
    }
}

impl<'a> Buffer<'a> {
    pub(crate) fn from_raw(buffer_ptr: *mut t_buffer) -> Buffer<'a> {
        Buffer {
            ptr: buffer_ptr,
            _phantom: PhantomData,
        }
    }

    pub fn data(&self) -> &str {
        let cstr = unsafe {
            let ptr = buffer_data(self.ptr);
            CStr::from_ptr(ptr)
        };
        cstr.to_str().unwrap()
    }

    pub fn data_copy(&self) -> String {
        self.data().to_owned()
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
    fn const_raw_pointer() {
        // Explicit cast:
        let i: u32 = 1;
        let p_imm: *const u32 = &i as *const u32;

        // Implicit coercion:
        let mut m: u32 = 2;
        let p_mut: *mut u32 = &mut m;

        unsafe {
            let ref_imm: &u32 = &*p_imm;
            let ref_mut: &mut u32 = &mut *p_mut;

            assert_eq!(*ref_imm, 1u32);
            assert_eq!(*ref_mut, 2u32);
        }
        assert_eq!(p_imm.wrapping_offset(1), p_mut);
    }

    #[test]
    fn null_pointer() {
        let p: *mut i32 = std::ptr::null_mut();
        assert!(p.is_null());

        let i = 0;
        let p: *const i32 = std::ptr::addr_of!(i);
        assert!(!p.is_null());
    }



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
    fn lifetime_test() {
        let session = Session::new();
        let buffer = session.buffer();

        assert_eq!(buffer.data(), "Buffer data");
    }

    //#[test]
    // fn lifetime_test2() {
    //     fn create_buffer<'a>() -> Buffer {
    //         let session = Session::new();
    //         session.buffer()
    //     }
    //     let buffer = create_buffer();

    //     assert_eq!(buffer.data(), "Buffer data");
    // }
}
