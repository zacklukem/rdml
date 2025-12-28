use syn::{
    Result,
    parse::{Parse, ParseBuffer},
};

pub(crate) trait ParseHelpers {
    fn parse_all<T: Parse>(&self) -> Result<Vec<T>>;
}

impl<'a> ParseHelpers for ParseBuffer<'a> {
    fn parse_all<T: Parse>(&self) -> Result<Vec<T>> {
        let mut result = Vec::new();
        while !self.is_empty() {
            result.push(self.parse()?)
        }
        Ok(result)
    }
}
