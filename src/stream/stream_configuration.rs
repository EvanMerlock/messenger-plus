#[derive(Debug, Clone, PartialEq)]
pub struct StreamConfiguration {
    pub(crate) delimiter_string: String,
    pub(crate) beginning_boundary: String,
    pub(crate) ending_boundary: String,
    pub(crate) hashing_enabled: bool,
}

impl StreamConfiguration {
    pub fn new<T: Into<String>>(delimiter_string: T, beginning_boundary: T, ending_boundary: T, hashing_enabled: bool) -> StreamConfiguration {
        StreamConfiguration {
            delimiter_string: delimiter_string.into(),
            beginning_boundary: beginning_boundary.into(),
            ending_boundary: ending_boundary.into(),
            hashing_enabled: hashing_enabled,
        }
    }

}