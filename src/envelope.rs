use prost::Message;
use prost_types::Any;

use crate::error::KiCadError;
use crate::proto::kiapi::common::{
    ApiRequest, ApiRequestHeader, ApiResponse, ApiStatusCode,
};

pub(crate) fn type_url(type_name: &str) -> String {
    format!("type.googleapis.com/{type_name}")
}

pub(crate) fn pack_any<T: Message>(message: &T, type_name: &str) -> Any {
    Any {
        type_url: type_url(type_name),
        value: message.encode_to_vec(),
    }
}

pub(crate) fn unpack_any<T: Message + Default>(
    response: &ApiResponse,
    expected_type_name: &str,
) -> Result<T, KiCadError> {
    let expected_type_url = type_url(expected_type_name);
    let payload = response
        .message
        .as_ref()
        .ok_or_else(|| KiCadError::MissingPayload {
            expected_type_url: expected_type_url.clone(),
        })?;

    if payload.type_url != expected_type_url {
        return Err(KiCadError::UnexpectedPayloadType {
            expected_type_url,
            actual_type_url: payload.type_url.clone(),
        });
    }

    T::decode(payload.value.as_slice())
        .map_err(|err| KiCadError::ProtobufDecode(err.to_string()))
}

pub(crate) fn encode_request(
    token: &str,
    client_name: &str,
    command: Any,
) -> Result<Vec<u8>, KiCadError> {
    let request = ApiRequest {
        header: Some(ApiRequestHeader {
            kicad_token: token.to_string(),
            client_name: client_name.to_string(),
        }),
        message: Some(command),
    };

    Ok(request.encode_to_vec())
}

pub(crate) fn decode_response(bytes: &[u8]) -> Result<ApiResponse, KiCadError> {
    ApiResponse::decode(bytes).map_err(|err| KiCadError::ProtobufDecode(err.to_string()))
}

pub(crate) fn status_error(response: &ApiResponse) -> Option<KiCadError> {
    let status = response.status.as_ref()?;
    let code = ApiStatusCode::try_from(status.status).unwrap_or(ApiStatusCode::AsUnknown);

    if code == ApiStatusCode::AsOk {
        return None;
    }

    Some(KiCadError::ApiStatus {
        code: code.as_str_name().to_string(),
        message: status.error_message.clone(),
    })
}

#[cfg(test)]
mod tests {
    use crate::proto::kiapi::common::{ApiResponse, ApiResponseStatus};

    use super::status_error;

    #[test]
    fn status_error_returns_none_for_ok() {
        let response = ApiResponse {
            header: None,
            status: Some(ApiResponseStatus {
                status: 1,
                error_message: String::new(),
            }),
            message: None,
        };

        assert!(status_error(&response).is_none());
    }

    #[test]
    fn status_error_returns_error_for_non_ok() {
        let response = ApiResponse {
            header: None,
            status: Some(ApiResponseStatus {
                status: 6,
                error_message: "token mismatch".to_string(),
            }),
            message: None,
        };

        let err = status_error(&response)
            .expect("non-ok API status should map to KiCadError::ApiStatus");
        let message = err.to_string();
        assert!(message.contains("AS_TOKEN_MISMATCH"));
    }
}
