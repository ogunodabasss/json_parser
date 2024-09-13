// Json Parse and Validate(JsonSchema,Code) trait
pub trait Json<T> {
    fn parse(json: &str) -> Vec<T>;

    fn validate(json: &str) -> Result<(), ()>;

    fn validate_data(vec: Vec<T>) -> bool;
}
