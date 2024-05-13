pub enum Mechanism {
    Plain,
}

impl Mechanism {
    pub fn as_str(&self) -> &str {
        match self {
            Mechanism::Plain => "PLAIN",
        }
    }
}

pub struct ConnectionParameters<'a> {
    pub host: &'a str,
    pub port: u16,
    pub username: &'a str,
    pub password: &'a str,
    pub mechanism: Mechanism,
    pub virtual_host: &'a str,
}

pub struct ConnectionParametersBuilder<'a> {
    host: Option<&'a str>,
    port: u16,
    username: Option<&'a str>,
    password: Option<&'a str>,
    mechanism: Mechanism,
    virtual_host: &'a str,
}

impl<'a> ConnectionParametersBuilder<'a> {
    pub fn builder() -> Self {
        Self {
            host: None,
            port: 5672_u16,
            username: None,
            password: None,
            mechanism: Mechanism::Plain,
            virtual_host: "/".into(),
        }
    }
    pub fn host(mut self, host: &'a str) -> Self {
        self.host = Some(host);
        self
    }
    pub fn username(mut self, username: &'a str) -> Self {
        self.username = Some(username);
        self
    }
    pub fn password(mut self, password: &'a str) -> Self {
        self.password = Some(password);
        self
    }
    pub fn virtual_host(mut self, virtual_host: &'a str) -> Self {
        self.virtual_host = virtual_host;
        self
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn build(self) -> ConnectionParameters<'a> {
        ConnectionParameters {
            host: self
                .host
                .expect("Must provide host in ConnectionParametersBuilder"),
            port: self.port,
            username: self
                .username
                .expect("Must provide `username` in ConnectionParametersBuilder"),
            password: self
                .password
                .expect("Must provided `password` in ConnectionParametersBuilder"),
            mechanism: self.mechanism,
            virtual_host: self.virtual_host,
        }
    }
}
