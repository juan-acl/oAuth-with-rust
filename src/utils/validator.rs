use validator::Validate;

pub fn validate_and_extract_errors<T: Validate>(data: &T) -> Result<(), Vec<String>> {
    if let Err(errors) = data.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!(
                        "{}: {}",
                        field,
                        error
                            .message
                            .clone()
                            .unwrap_or_else(|| "Error desconocido".to_string().into())
                    )
                })
            })
            .collect();
        Err(error_messages)
    } else {
        Ok(())
    }
}
