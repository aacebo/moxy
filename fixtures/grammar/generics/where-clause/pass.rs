fn f<T>() where T: Clone + Send, for<'a> &'a T: IntoIterator {}
