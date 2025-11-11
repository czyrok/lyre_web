use http::StatusCode;
use leptos::prelude::expect_context;
use leptos_axum::ResponseOptions;

use crate::core::{
    behaviors::use_case::UseCase,
    error::server_function_error::ServerFunctionError,
};

pub async fn run_use_case<
    TUseCase: UseCase<TInput, TOutput>,
    TInput,
    TOutput,
>(
    mut use_case: TUseCase,
    options: TInput,
) -> Result<TOutput, ServerFunctionError> {
    let ok_result = match use_case.run(options).await {
        Ok(ok_result) => ok_result,
        Err(error) => {
            let response = expect_context::<ResponseOptions>();

            response.set_status(
                StatusCode::from_u16(error.0.status_code)
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            );

            return Err(error);
        }
    };

    Ok(ok_result)
}
