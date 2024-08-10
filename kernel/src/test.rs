pub trait Test {
    fn test(&self);
}

impl<T> Test for T
where
    T: Fn(),
{
    fn test(&self) {
        crate::print!("{}...\t", core::any::type_name::<T>());
        self();
        crate::println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Test]) {
    crate::println!("running {} tests", tests.len());
    for test in tests {
        test.test();
    }

    unsafe { crate::util::exit_qemu() };
}
