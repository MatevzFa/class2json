
use std::fmt;
use class_file::*;

impl fmt::Debug for Utf8Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Utf8Info")
        .field("tag", &self.tag)
        .field("length", &self.length)
        .field("bytes", String::from_utf8(&self.bytes))
    }
}