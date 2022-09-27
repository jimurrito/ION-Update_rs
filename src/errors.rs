// No Internet Connection or IP endpoint is offline
pub const ERR0: &str =
    r#"[1x0] Unable to retieve current IP address. Please Check Internet Connection."#;
// No Internet Connection or IONOS outage or endpoint changed
pub const ERR1: &str = r#"[1x1] Unable to contact IONOS endpoints. Please Check Internet Connection or check the endpoint manually."#;
// Response from IONOS couldn't be converted to an object. Likely issue is with AuthN/Z when accessing IONOS
pub const ERR2: &str =
    r#"[1x2] Failed to Deserialize json into struct. Please check your docker configuration."#;
// A required Env var was not set when app was ran. Most likely to happen if ran outside of the container.
pub const ERR3: &str = r#"[1x3] Env var was not defined during docker build. Please contact developer. This error is fatal."#;
// Unlikely to trigger - app object failed to convert back to json. Error handled for ease of bug fixing.
pub const ERR4: &str = r#"[1x4] Unable to Serialize DNS Record back into json. Please rerun with '-e LOG_LEVEL = "debug"' for more information. This error is fatal."#;
// Required Variable was not provided in the docker build
pub const ERR5: &str = r#"[1x5] Please check your docker configuration. This error is fatal."#;
// Log level provided was undefined
pub const ERR6: &str = r#"[1x6] Log Level provided is not defined. Options are 'info' and 'debug'. This error is fatal."#;
