impl<T: Copy> T for Wrapper<T> { fn value(&self) -> T { self.0 } }
