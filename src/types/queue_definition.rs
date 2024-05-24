pub struct QueueDefinition {
    pub queue_name: String,
    pub passive: bool,
    pub durable: bool,
    pub auto_delete: bool,
    pub exclusive: bool,
    pub no_wait: bool,
}

impl QueueDefinition {
    pub fn builder() -> QueueDefinitionBuilder {
        QueueDefinitionBuilder {
            queue_name: None,
            passive: None,
            durable: None,
            auto_delete: None,
            exclusive: None,
            no_wait: None,
        }
    }

    pub fn new(
        queue_name: String,
        passive: bool,
        durable: bool,
        auto_delete: bool,
        exclusive: bool,
        no_wait: bool,
    ) -> Self {
        Self {
            queue_name,
            passive,
            durable,
            auto_delete,
            exclusive,
            no_wait,
        }
    }
}

pub struct QueueDefinitionBuilder {
    queue_name: Option<String>,
    passive: Option<bool>,
    durable: Option<bool>,
    auto_delete: Option<bool>,
    exclusive: Option<bool>,
    no_wait: Option<bool>,
}

impl QueueDefinitionBuilder {
    pub fn queue_name(mut self, name: String) -> Self {
        self.queue_name = Some(name);
        self
    }
    pub fn passive(mut self, passive: bool) -> Self {
        self.passive = Some(passive);
        self
    }
    pub fn durable(mut self, durable: bool) -> Self {
        self.durable = Some(durable);
        self
    }
    pub fn auto_delete(mut self, auto_delete: bool) -> Self {
        self.auto_delete = Some(auto_delete);
        self
    }
    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }
    pub fn no_wait(mut self, no_wait: bool) -> Self {
        self.no_wait = Some(no_wait);
        self
    }

    pub fn build(self) -> QueueDefinition {
        QueueDefinition {
            queue_name: self.queue_name.unwrap_or("".into()),
            passive: self.passive.unwrap_or(false),
            durable: self.durable.unwrap_or(false),
            auto_delete: self.auto_delete.unwrap_or(false),
            exclusive: self.auto_delete.unwrap_or(false),
            no_wait: self.no_wait.unwrap_or(false),
        }
    }
}
