use crate::gesture::{self, Id};
use crate::{Bounds, Event, Output, Size, View};
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
    fn event(&self, event: Event, bounds: Bounds) {
        if let Ok((gesture, offset)) = event.try_into() {
            if let Some(gesture::Response::UpInside) = gesture::recognizer(
                self.id,
                gesture,
                offset.get(),
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

pub struct Target<V> {
    pub(crate) view: V,
    pub(crate) minimum: Size,
}

impl<V: View> View for Target<V> {
    #[inline(always)]
    fn size(&self) -> Size {
        self.view.size()
    }

    #[inline]
    fn event(&self, event: Event, bounds: Bounds) {
        let mut target = bounds;
        target.min -= (self.minimum - self.size()) / 2.0;
        target.set_size(self.minimum.max(self.size()));

        match &event {
            Event::Gesture(_, location) if target.contains_inclusive(location.get()) => {
                location.set(bounds.min);
            }
            _ => {}
        }

        self.view.event(event, bounds)
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.view.draw(bounds, onto)
    }
}
