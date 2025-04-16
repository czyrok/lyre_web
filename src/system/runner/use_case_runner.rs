use http::StatusCode;
use leptos::prelude::expect_context;
use leptos_axum::ResponseOptions;

use crate::core::{
    behaviors::use_case::UseCase,
    error::{
        named::internal_server_error::InternalServerError,
        server_function_error::{ServerFunctionError, ServerFunctionException},
    },
};

pub async fn run_use_case<
    TUseCase: UseCase<TInput, TOutput>,
    TInput,
    TOutput,
>(
    mut use_case: TUseCase,
    options: TInput,
) -> Result<TOutput, ServerFunctionException> {
    let response = expect_context::<ResponseOptions>();

    let ok_result = match use_case.run(options).await {
        Ok(ok_result) => ok_result,
        Err(error) => match error {
            ServerFunctionError::WrappedServerError(value) => {
                response.set_status(
                    StatusCode::from_u16(value.status_code)
                        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                );

                return Err(ServerFunctionException::WrappedServerError(value));
            }
            _ => {
                return Err(InternalServerError::new(Some(
                    "Very unknown error".into(),
                ))
                .into())
            }
        },
    };

    Ok(ok_result)
}
