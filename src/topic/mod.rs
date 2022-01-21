use crate::question::Question;
use crate::slide::Slide;
use crate::utils::queue::Queue;

pub trait Topic<T, U>
where
    T: Slide,
    U: Question,
{
    fn name(&self) -> &str;
    fn slides(&self) -> Queue<T>;
    fn question(&self) -> Queue<U>;
}
