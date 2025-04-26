use std::future::Future;

use crate::core::error::server_function_error::ServerFunctionError;

pub trait UseCase<TInput, TOutput> {
    fn run(
        &mut self,
        options: TInput,
    ) -> impl Future<Output = Result<TOutput, ServerFunctionError>>;
}
