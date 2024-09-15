#![allow(dead_code)]

use std::marker::PhantomData;

use crate::text::Font;
use crate::ui::{accessibility, Inter};
use composable::dependencies::{Dependency, DependencyDefault};

mod scale;

#[allow(non_upper_case_globals)]
const InterVariable: &[u8] = include_bytes!("../InterVariable.ttf");

#[inline(never)]
fn font<Design>(weight: f32, scale: f32) -> Inter<Design> {
    let accessibility = Dependency::<accessibility::Scale>::new();

    Inter {
        marker: PhantomData,
        font: Font::from(InterVariable)
            .unwrap()
            .weight(weight)
            .size(accessibility.scale(scale)),
    }
}

/// Title `Font` styles.
pub mod title {
    use super::*;

    /// Large variant
    pub struct L;
    /// Medium variant
    pub struct M;
    /// Small variant
    pub struct S;

    impl DependencyDefault for Inter<L> {}
    impl DependencyDefault for Inter<M> {}
    impl DependencyDefault for Inter<S> {}

    impl Default for Inter<L> {
        fn default() -> Self {
            font(650.0, 6.0)
        }
    }

    impl Default for Inter<M> {
        fn default() -> Self {
            font(650.0, 4.0)
        }
    }

    impl Default for Inter<S> {
        fn default() -> Self {
            font(650.0, 3.0)
        }
    }
}

/// Body `Font` styles.
pub mod body {
    use super::*;

    /// Large variant
    pub struct L;
    /// Medium variant
    pub struct M;
    /// Small variant
    pub struct S;

    impl DependencyDefault for Inter<L> {}
    impl DependencyDefault for Inter<M> {}
    impl DependencyDefault for Inter<S> {}

    impl Default for Inter<L> {
        fn default() -> Self {
            font(500.0, 4.0)
        }
    }

    impl Default for Inter<M> {
        fn default() -> Self {
            font(500.0, 3.0)
        }
    }

    impl Default for Inter<S> {
        fn default() -> Self {
            font(400.0, 2.0)
        }
    }
}

/// Label `Font` styles.
pub mod label {
    use super::*;

    /// Large variant
    pub struct L;
    /// Medium variant
    pub struct M;
    /// Small variant
    pub struct S;

    impl DependencyDefault for Inter<L> {}
    impl DependencyDefault for Inter<M> {}
    impl DependencyDefault for Inter<S> {}

    impl Default for Inter<L> {
        fn default() -> Self {
            font(600.0, 2.0)
        }
    }

    impl Default for Inter<M> {
        fn default() -> Self {
            font(600.0, 1.0)
        }
    }

    impl Default for Inter<S> {
        fn default() -> Self {
            font(400.0, 0.0)
        }
    }
}
