use rocket::serde::json::Json;

use bambangshop_receiver::Result;
use crate::model::notification::Notification;
use crate::xwwwwmodel::subscriber::SubscriberRequest;
use crate::service::notification::NotificationService;