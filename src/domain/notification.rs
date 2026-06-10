use std::collections::HashSet;

pub struct Nofication {
    user_id: u32,
    channel: Channel,
    message: String,
    notificaiton_type: NotificationType,
    preferences: HashSet<Preference>,
}

pub enum Preference {
    Email,
    Sms,
    Push,
    Whatsapp,
}

pub enum NotificationType {
    Alert,
    Reminder,
    Promotion,
}

pub enum Channel {
    Email,
    SMS,
    Push,
}
