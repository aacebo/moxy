trait ReceiverForms {
    fn by_value(self: Self);
    fn by_mut_value(mut self: Self);
    fn by_ref(self: &Self);
    fn by_box(self: Box<Self>);
    fn by_pin(self: Pin<Self>);
}
