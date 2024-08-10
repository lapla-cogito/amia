#[derive(Debug)]
pub struct Mutex<T> {
    value: core::cell::UnsafeCell<T>,
    locked: core::sync::atomic::AtomicBool,
}

impl<T: Sized> Mutex<T> {
    pub const fn new(val: T) -> Self {
        Self {
            value: core::cell::UnsafeCell::new(val),
            locked: core::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn try_lock(&self) -> crate::Result<MutexGuard<T>> {
        if self
            .locked
            .compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::Acquire,
                core::sync::atomic::Ordering::Relaxed,
            )
            .is_ok()
        {
            return Ok(unsafe { MutexGuard::new(self, &self.value) });
        }

        Err(crate::Error::MutexErr("lock failed"))
    }

    pub fn free(&self) {
        self.locked
            .store(false, core::sync::atomic::Ordering::Relaxed);
    }
}

unsafe impl<T> Sync for Mutex<T> {}

#[derive(Debug)]
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
    value: &'a mut T,
}

impl<'a, T> MutexGuard<'a, T> {
    unsafe fn new(mutex: &'a Mutex<T>, val: &core::cell::UnsafeCell<T>) -> Self {
        Self {
            mutex,
            value: &mut *val.get(),
        }
    }
}

unsafe impl<'a, T> Sync for MutexGuard<'a, T> {}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex
            .locked
            .store(false, core::sync::atomic::Ordering::Relaxed);
    }
}

#[test_case]
fn test_mutex_write() {
    let mutex = Mutex::new(0);
    let mut guard = mutex.try_lock().unwrap();
    *guard += 1;

    assert_eq!(*guard, 1);
}

#[test_case]
fn test_mutex_lock() {
    let mutex = Mutex::new(0);
    let _lock = mutex.try_lock().unwrap();

    assert!(mutex.try_lock().is_err());
}

#[test_case]
fn test_mutex_free() {
    let mutex = Mutex::new(0);
    let mut guard = mutex.try_lock().unwrap();
    *guard += 1;
    assert_eq!(*guard, 1);

    mutex.free();
    let mut guard = mutex.try_lock().unwrap();
    *guard += 1;
    assert_eq!(*guard, 2);
}
