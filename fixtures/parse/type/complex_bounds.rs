impl for<'a> Fn(&'a mut dyn Iterator<Item = &'a str>) -> Result<Box<dyn Display + Send + 'a>, <T as Trait>::Assoc> + Send
