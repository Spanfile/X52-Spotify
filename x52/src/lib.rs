#![allow(non_upper_case_globals)]

mod libx52;

use libx52::*;
use std::cell::RefCell;
use std::fmt;
use std::ptr;

pub struct X52 {
    device: RefCell<*mut libx52_device>,
}

pub enum MFDLine {
    Line0 = 0,
    Line1 = 1,
    Line2 = 2,
}

#[derive(Debug, Clone)]
pub struct X52Error {
    code: libx52_error_code,
    message: String,
}

impl X52Error {
    fn from_error_code(code: i32) -> X52Error {
        let code = code as libx52_error_code;
        let message = unsafe { std::ffi::CStr::from_ptr(libx52_strerror(code)) }
            .to_str()
            .expect("couldn't build UTF-8 string from libx52_strerror")
            .to_string();
        X52Error { code, message }
    }
}

impl fmt::Display for X52Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for X52Error {}

impl X52 {
    pub fn new() -> X52 {
        let mut device: *mut libx52_device = ptr::null_mut();
        let error = unsafe { libx52_init(&mut device) };
        println!("init: {:?}", X52Error::from_error_code(error));
        X52 {
            device: RefCell::new(device),
        }
    }

    pub fn reinitialise(&self) -> Result<(), X52Error> {
        let error = unsafe { libx52_init(&mut *self.device.borrow_mut()) };
        if error == 0 {
            Ok(())
        } else {
            println!("reinitialisation failed");
            self.free();
            Err(X52Error::from_error_code(error))
        }
    }

    fn free(&self) {
        if !self.device.borrow().is_null() {
            unsafe { libx52_exit(*self.device.borrow()) };
            *self.device.borrow_mut() = ptr::null_mut();
        }
    }

    fn update(&self) -> Result<(), X52Error> {
        let error = unsafe { libx52_update(*self.device.borrow()) };
        if error == 0 {
            Ok(())
        } else {
            self.handle_error(X52Error::from_error_code(error))
        }
    }

    fn handle_error(&self, error: X52Error) -> Result<(), X52Error> {
        println!("X52 error handler: {:?}", error);
        match error.code {
            // INVALID_PARAM = device pointer is null
            // NO_DEVICE = device pointer is invalid
            libx52_error_code_LIBX52_ERROR_INVALID_PARAM
            | libx52_error_code_LIBX52_ERROR_NO_DEVICE => {
                println!("handling missing device error -> attempting to reinitialise");
                self.reinitialise()
            }
            _ => Err(error),
        }
    }

    pub fn set_lines(&self, lines: [String; 3]) -> Result<(), X52Error> {
        for line in 0..=2 {
            loop {
                let error = unsafe {
                    libx52_set_text(
                        *self.device.borrow(),
                        line as u8,
                        lines[line].as_ptr() as *const std::os::raw::c_char,
                        lines[line].len() as u8,
                    )
                };
                if error != 0 {
                    // error happened; try to handle it
                    match self.handle_error(X52Error::from_error_code(error)) {
                        Err(e) => return Err(e), // couldn't handle it ourselves, return it
                        _ => continue,           // could handle it, retry the operation
                    };
                }

                break;
            }
        }
        self.update()
    }
}

impl Drop for X52 {
    fn drop(&mut self) {
        self.free();
    }
}

unsafe impl Sync for X52 {}