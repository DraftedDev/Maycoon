use crate::types::WidgetTree;

pub struct Page<S> {
    pub(crate) state: S,
    pub(crate) widgets: WidgetTree,
    pub(crate) state_changed: bool,
}

impl<S> Page<S> {
    pub fn build(widgets: WidgetTree, state: S) -> Page<S> {
        Self {
            state,
            widgets,
            state_changed: true,
        }
    }

    pub fn set_state<F: FnMut(&mut S)>(&mut self, mut change: F) {
        change(&mut self.state);
        self.state_changed = true;
    }
}
