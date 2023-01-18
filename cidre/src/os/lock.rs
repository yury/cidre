#[repr(C)]
pub struct UnfairLock {
    internal: u32,
}

impl UnfairLock {
    pub fn new() -> Self {
        Self { internal: 0 }
    }

    #[inline]
    pub fn lock(&self) {
        unsafe { os_unfair_lock_lock(self) }
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        unsafe { os_unfair_lock_trylock(self) }
    }

    #[inline]
    pub fn unlock(&self) {
        unsafe { os_unfair_lock_unlock(self) }
    }

    #[inline]
    pub fn assert_owner(&self) {
        unsafe { os_unfair_lock_assert_owner(self) }
    }

    #[inline]
    pub fn assert_not_owner(&self) {
        unsafe { os_unfair_lock_assert_not_owner(self) }
    }
}

impl Default for UnfairLock {
    fn default() -> Self {
        Self::new()
    }
}

extern "C" {
    fn os_unfair_lock_lock(lock: &UnfairLock);
    fn os_unfair_lock_trylock(lock: &UnfairLock) -> bool;
    fn os_unfair_lock_unlock(lock: &UnfairLock);
    fn os_unfair_lock_assert_owner(lock: &UnfairLock);
    fn os_unfair_lock_assert_not_owner(lock: &UnfairLock);
}
