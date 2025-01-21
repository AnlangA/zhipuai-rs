#[macro_export]
macro_rules! simple_message {
    ($role:expr, $content:expr) => {
        Message::new(
            $role.into(),
            Some(Context::SimpleContexts($content.to_string())),
            None,
        )
    };
}
