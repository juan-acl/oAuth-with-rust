use validator::Validate;

pub fn fields_validator<T: Validate>(data: &T) -> Result<(), Vec<String>> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(e) => {
            let mut errors: Vec<String> = Vec::new();
            for (field, field_errors) in e.field_errors() {
                for error in field_errors {
                    let message = error
                        .message
                        .clone()
                        .unwrap_or_else(|| "Error desconocido".to_string().into());
                    errors.push(format!("{}: {}", field, message));
                }
            }
            Err(errors)
        }
    }
}
