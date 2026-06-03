use super::*;

pub struct Rule {
    name: &'static str,
    method: std::pin::Pin<Box<dyn Fn(&mut Formatter) -> Result<(), FormatError>>>,
}

impl Rule {
    pub fn new<F: Fn(&mut Formatter) -> Result<(), FormatError> + 'static>(name: &'static str, rule: F) -> Self {
        Self {
            name,
            method: Box::pin(rule),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl Fmt for Rule {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        (self.method)(f)
    }
}
