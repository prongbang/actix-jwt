pub trait UseCase<P, R> {
    fn execute(&self, param: P) -> R;
}