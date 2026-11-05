#[derive(Debug, Clone, PartialEq)]
pub struct HermesMessage {
    pub role: String,
    pub content: String,
}

pub struct HermesContext {
    pub messages: Vec<HermesMessage>,
}

impl Default for HermesContext {
    fn default() -> Self {
        Self::new()
    }
}

impl HermesContext {
    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub fn push_user(&mut self, msg: impl Into<String>) {
        self.messages.push(HermesMessage { role: "user".into(), content: msg.into() });
    }

    pub fn push_assistant(&mut self, msg: impl Into<String>) {
        self.messages.push(HermesMessage { role: "assistant".into(), content: msg.into() });
    }

    pub fn total_messages(&self) -> usize {
        self.messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hermes_context() {
        // Arrange
        let mut ctx = HermesContext::new();
        // Act
        ctx.push_user("Run diagnostic");
        ctx.push_assistant("System healthy");
        // Assert
        assert_eq!(ctx.total_messages(), 2);
    }
}
