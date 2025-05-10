pub trait ToManager<'a, S> {
    fn to_manager(&'a self) -> S;
}
