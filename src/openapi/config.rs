use aide::openapi::{Info, OpenApi, Server as OpenApiServer};

/// Configuration for OpenAPI documentation generation.
///
/// # Example
///
/// ```no_run
/// use uncover::openapi::OpenApiConfig;
///
/// let config = OpenApiConfig::new("My API", "1.0.0")
///     .description("A comprehensive API")
///     .server("https://api.example.com", "Production")
///     .server("http://localhost:3000", "Development");
/// ```
#[derive(Clone, Debug)]
pub struct OpenApiConfig {
    pub(crate) title: String,
    pub(crate) version: String,
    pub(crate) description: Option<String>,
    pub(crate) terms_of_service: Option<String>,
    pub(crate) contact_name: Option<String>,
    pub(crate) contact_email: Option<String>,
    pub(crate) contact_url: Option<String>,
    pub(crate) license_name: Option<String>,
    pub(crate) license_url: Option<String>,
    pub(crate) servers: Vec<(String, String)>,
}

impl OpenApiConfig {
    /// Creates a new OpenAPI configuration.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of your API
    /// * `version` - The version of your API
    ///
    /// # Example
    ///
    /// ```
    /// use uncover::openapi::OpenApiConfig;
    ///
    /// let config = OpenApiConfig::new("My API", "1.0.0");
    /// ```
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            version: version.into(),
            description: None,
            terms_of_service: None,
            contact_name: None,
            contact_email: None,
            contact_url: None,
            license_name: None,
            license_url: None,
            servers: Vec::new(),
        }
    }

    /// Sets the API description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the terms of service URL.
    pub fn terms_of_service(mut self, url: impl Into<String>) -> Self {
        self.terms_of_service = Some(url.into());
        self
    }

    /// Sets contact information.
    pub fn contact(
        mut self,
        name: impl Into<String>,
        email: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        self.contact_name = Some(name.into());
        self.contact_email = Some(email.into());
        self.contact_url = Some(url.into());
        self
    }

    /// Sets license information.
    pub fn license(mut self, name: impl Into<String>, url: impl Into<String>) -> Self {
        self.license_name = Some(name.into());
        self.license_url = Some(url.into());
        self
    }

    /// Adds a server to the OpenAPI specification.
    ///
    /// # Arguments
    ///
    /// * `url` - The server URL
    /// * `description` - A description of the server
    pub fn server(mut self, url: impl Into<String>, description: impl Into<String>) -> Self {
        self.servers.push((url.into(), description.into()));
        self
    }

    /// Builds the OpenAPI specification.
    pub(crate) fn build(&self) -> OpenApi {
        let mut info = Info {
            title: self.title.clone(),
            version: self.version.clone(),
            description: self.description.clone(),
            terms_of_service: self.terms_of_service.clone(),
            ..Default::default()
        };

        if let (Some(name), Some(email), Some(url)) =
            (&self.contact_name, &self.contact_email, &self.contact_url)
        {
            info.contact = Some(aide::openapi::Contact {
                name: Some(name.clone()),
                email: Some(email.clone()),
                url: Some(url.clone()),
                ..Default::default()
            });
        }

        if let (Some(name), Some(url)) = (&self.license_name, &self.license_url) {
            info.license = Some(aide::openapi::License {
                name: name.clone(),
                identifier: None,
                url: Some(url.clone()),
                ..Default::default()
            });
        }

        let servers = self
            .servers
            .iter()
            .map(|(url, description)| OpenApiServer {
                url: url.clone(),
                description: Some(description.clone()),
                ..Default::default()
            })
            .collect();

        OpenApi {
            info,
            servers,
            ..Default::default()
        }
    }
}
