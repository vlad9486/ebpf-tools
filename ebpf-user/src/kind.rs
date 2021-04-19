use super::skeleton::{MapRef, ProgRef};

pub enum AppItemKindMut<'a> {
    Map(&'a mut MapRef),
    Prog(&'a mut ProgRef),
}

pub trait AppItem {
    const MAP: usize;
    const PROG: usize;

    fn named(name: &'static str) -> Self;
    fn kind_mut(&mut self) -> AppItemKindMut<'_>;
}
