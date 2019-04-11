use fibonacci_sys as fibo;
use libc;

pub struct Fibonacci {
    handle: *mut fibo::Fibonacci_t,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            handle: unsafe { fibo::fibo_new() }
        }
    }

    pub fn next(&mut self) -> u32 {
        unsafe { fibo::fibo_next(self.handle) }
    }
}

impl Drop for Fibonacci {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.handle as *mut core::ffi::c_void);
        }
        println!("free()");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fibonacci_test() {
        let expects: [u32; 10] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        let mut fibo = Fibonacci::new();
        for &e in expects.iter() {
            assert_eq!(fibo.next(), e);
        }
    }
}

