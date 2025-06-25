
use crate::prelude::*;

use super::AnnotationType;

#[derive(Clone, Copy, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start: ByteIdx,
    pub end: ByteIdx,
}
