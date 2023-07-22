use std::error::Error;

pub type ByteVec = Vec<u8>;

pub type StringVec = Vec<String>;

pub type OptionalStr<'a> = Option<&'a str>;

pub type OptionalString = Option<String>;

pub type OptionalByteVec = Option<ByteVec>;

pub type BoxedError = Box<dyn Error>;
