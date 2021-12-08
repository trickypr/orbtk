use crate::{api::prelude::*, proc_macros::*};

widget!(
    /// The `Stack` defines a layout that is used to stack its children vertical
    /// or horizontal, depending on the value of `orientation`. It will default
    /// to vertical.
    ///
    /// ## Example
    /// ```rs
    /// use orbtk::prelude::*;
    ///
    /// fn build_stack(ctx: &mut BuildContext) -> Entity {
    ///     Stack::new()
    ///         .orientation(Orientation::Horizontal)
    ///         .child(TextBlock::new().text("Hello, ").build(ctx))
    ///         .child(TextBlock::new().text("World!").build(ctx))
    ///         .build(ctx);
    /// }
    /// ```
    ///
    /// **style:** `stack`
    Stack {
        /// Sets or shares the orientation property.
        orientation: Orientation,

        /// Margin between widgets in the stack.
        spacing: f64
    }
);

impl Template for Stack {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Stack").orientation("vertical").style("stack")
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}
