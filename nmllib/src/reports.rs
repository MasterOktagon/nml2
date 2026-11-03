use std::collections::HashMap;

pub struct H(pub HashMap<String, ariadne::Source>);

impl ariadne::Cache<&str> for &mut H {
    type Storage = String;

    fn fetch(&mut self, id: &&str) -> Result<&ariadne::Source<Self::Storage>, impl std::fmt::Debug> {
        return self.0.get(*id).ok_or(())
    }

    fn display<'a>(&self, id: &'a &str) -> Option<impl std::fmt::Display + 'a> {
        Some(id.to_string())
    }
}

