pub trait Builder<TBuilt> {
    fn build(&self) -> TBuilt;
}