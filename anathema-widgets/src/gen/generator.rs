use super::expressions::Expression;
use super::scope::Scope;
use super::store::Store;
use crate::error::Result;
use crate::template::Template;
use crate::{Lookup, WidgetContainer};

// -----------------------------------------------------------------------------
//   - Direction -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
}

// -----------------------------------------------------------------------------
//   - Generator -
// -----------------------------------------------------------------------------
pub struct Generator<'tpl, 'parent> {
    scope: Scope<'tpl, 'parent>,
}

impl<'tpl, 'parent> Generator<'tpl, 'parent> {
    pub fn new(templates: &'tpl [Template], factory: &'parent Lookup, values: &mut Store<'parent>) -> Self {
        Self {
            scope: Scope::new(templates, factory, values, Direction::Forward),
        }
    }

    pub fn reverse(&mut self) {
        self.scope.reverse();
    }

    pub fn next(
        &mut self,
        values: &mut Store<'parent>,
    ) -> Option<Result<WidgetContainer<'tpl>>> {
        self.scope.next(values)
    }
}
