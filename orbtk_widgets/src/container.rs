use crate::{api::prelude::*, proc_macros::*};

widget!(
    /// The `Container` layout widget surrounds its child with a padding. Draws a box around the child.
    ///
    /// ## Example:
    /// ```
    /// use orbtk::prelude::*;
    ///
    /// fn build_container(ctx: &mut BuildContext) -> Entity {
    ///     Container::new()
    ///         .child(TextBlock::new("Hello World!"))
    ///         .border_radius(2.0)
    ///         .background("black")
    ///         .build(ctx)
    /// }
    /// ```
    Container {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness
    }
);

impl Template for Container {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Container")
            .padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        PaddingLayout::new().into()
    }
}
