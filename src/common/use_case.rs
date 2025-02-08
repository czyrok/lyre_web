use std::future::Future;

use super::error::server_function_error::ServerFunctionError;

pub trait UseCase<TOptions, TOkResult> {
    fn run(
        &mut self,
        options: TOptions,
    ) -> impl Future<Output = Result<TOkResult, ServerFunctionError>>;
}
