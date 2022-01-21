use crate::utils::queue::QueueError;

pub enum Error {
    QueueError(QueueError),
}
