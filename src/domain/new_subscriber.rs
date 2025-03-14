use super::subscriber_name::SubscriberName;
use super::subscriber_email::SubscriberEmail;

pub struct NewSubscriber {
  pub name: SubscriberName,
  pub email: SubscriberEmail,
}
