use crate::prelude::*;

/// EntityEvent handler. Must be observed on the given entity.
pub fn on_scroll_handler(
    mut scroll: On<Pointer<Scroll>>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let (mut dx, mut dy) = (scroll.x, scroll.y);
    if node.overflow.x == OverflowAxis::Scroll && dx != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if dx > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !max {
            scroll_position.x += dx;
            // Consume the X portion of the scroll delta.
            dx = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && dy != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if dy > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += dy;
            // Consume the Y portion of the scroll delta.
            dy = 0.;
        }
    }

    // Stop propagating when the delta is fully consumed.
    if Vec2::new(dx, dy) == Vec2::ZERO {
        scroll.propagate(false);
    }
}

fn on_append_to_console(
    trigger: On<AppendToConsole>,
    q: Query<Entity, With<ConsoleBodyTextWrapper>>,
    mut commands: Commands,
) {
    for e in q {
        let text_entt = commands
            .spawn((ConsoleBodyText, Text::new(trigger.0.clone())))
            .id();
        commands.entity(e).add_child(text_entt);
        // this will scroll to the bottom _after_ appending the new line
        // so the computed node has time to recalculate
        commands.write_message(trigger.event().clone());
    }
}
fn on_clear_console(
    _: On<ClearConsole>,
    q: Query<Entity, With<ConsoleBodyText>>,
    mut commands: Commands,
) {
    for e in q {
        commands.entity(e).despawn();
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_clear_console);
    app.add_observer(on_append_to_console);
}
