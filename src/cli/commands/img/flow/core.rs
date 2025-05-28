use std::marker::PhantomData;

use crate::img::Img;

pub struct Start;
pub struct ImagesResolved;
pub struct ImagesTransformed;

pub struct ImgFlow<S> {
    pub(super) _state: PhantomData<S>,
    pub(super) images: Vec<Img>,
}
