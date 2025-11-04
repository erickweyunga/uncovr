use uncover::prelude::*;

pub fn config() -> AppConfig {
    AppConfig::new("Hello API", "1.0.0")
        .description("A simple example showing user creation with Uncover framework")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development)
        .cors(CorsConfig::development())
        .logging(
            LoggingConfig::default()
                .level(LogLevel::Debug)
                .format(LogFormat::Pretty)
                .log_requests(true)
                .log_responses(false),
        )
        .docs(true)
        .add_server("http://localhost:3000", "Local development")
}
