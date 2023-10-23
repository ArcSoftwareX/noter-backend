use validator::ValidationErrors;

pub fn transform_errors(errors: ValidationErrors) -> Vec<(String, String)> {
    errors
        .field_errors()
        .iter()
        .map(|(&key, &value)| {
            (
                key.to_string(),
                value.first().unwrap().message.as_ref().unwrap().to_string(),
            )
        })
        .collect()
}
