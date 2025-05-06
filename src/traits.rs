pub trait ToService<'a, S> {
    fn to_service(&'a self) -> S;
}
