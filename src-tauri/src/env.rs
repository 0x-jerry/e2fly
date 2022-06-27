pub fn is_dev() -> bool {
    return dotenv!("E2FLY_DEVELOPMENT") == "true";
}
