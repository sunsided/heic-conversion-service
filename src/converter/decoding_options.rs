#[derive(Debug, Default)]
pub struct DecodingOptions {
    pub ignore_transformations: bool,
    pub convert_hdr_to_8bit: bool,
}

impl DecodingOptions {
    pub fn set_ignore_transformations(&mut self, value: bool) -> &mut Self {
        self.ignore_transformations = value;
        self
    }

    pub fn set_convert_hdr_to_8bit(&mut self, value: bool) -> &mut Self {
        self.convert_hdr_to_8bit = value;
        self
    }
}
