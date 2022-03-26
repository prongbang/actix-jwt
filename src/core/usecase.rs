use async_trait::async_trait;

#[async_trait]
pub trait UseCase<P, R> {
    fn execute(&self, param: P) -> R;
}