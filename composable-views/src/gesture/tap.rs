use crate::gesture::{self, Id};
use crate::{Bounds, Event, Output, Point, Size, View};
use composable::Effects;

pub struct TapGesture<V, A, E> {
    pub(crate) id: Id,
    pub(crate) view: V,
    pub(crate) action: A,
    pub(crate) send: E,
}

impl<V, A, E> View for TapGesture<V, A, E>
where
    V: View,
    A: Clone,
    E: Effects<A>,
{
    #[inline(always)]
    fn size(&self) -> Size {
        self.view.size()
    }

    #[inline]
    fn event(&self, event: Event, offset: Point, bounds: Bounds) {
        if let Ok(gesture) = event.try_into() {
            if let Some(gesture::Response::UpInside) = gesture::recognizer(
                self.id,
                gesture,
                offset,
                Bounds::from_origin_and_size(bounds.min, self.size()),
            ) {
                self.send.action(self.action.clone())
            }
        }
    }

    #[inline(always)]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.view.draw(bounds, onto)
    }
}
