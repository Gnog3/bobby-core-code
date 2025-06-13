use critical_section::RawRestoreState;

struct MyCriticalSection;
critical_section::set_impl!(MyCriticalSection);

unsafe impl critical_section::Impl for MyCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // No critical section
    }

    unsafe fn release(_token: RawRestoreState) {
        // No critical section
    }
}