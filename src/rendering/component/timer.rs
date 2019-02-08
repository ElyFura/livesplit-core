use crate::{
    component::timer::State,
    rendering::{Backend, RenderContext, MARGIN},
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    [width, height]: [f32; 2],
    component: &State,
) -> f32 {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    let x = context.render_timer(
        &component.fraction,
        [width - MARGIN, 0.85 * height],
        0.8 * height,
        [component.bottom_color, component.top_color],
    );
    context.render_timer(
        &component.time,
        [x, 0.85 * height],
        1.2 * height,
        [component.bottom_color, component.top_color],
    )
}
