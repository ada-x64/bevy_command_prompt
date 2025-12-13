use bevy::{
    color::palettes::tailwind,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_simple_text_input::{TextInput, TextInputPlugin, TextInputSubmitMessage, TextInputSystem};

#[derive(Component, Debug, Reflect, Clone)]
pub struct ConsoleFont(pub Handle<Font>);

#[derive(Component, Debug, Reflect, Clone, Copy)]
pub struct ConsoleTextColor(pub Color);
impl Default for ConsoleTextColor {
    fn default() -> Self {
        Self(tailwind::SLATE_100.into())
    }
}

#[derive(Component, Debug, Reflect, Clone, Copy)]
#[component(on_insert=ConsoleBackground::add)]
pub struct ConsoleBackground(pub Color);
impl Default for ConsoleBackground {
    fn default() -> Self {
        Self(tailwind::SLATE_950.with_alpha(75.).into())
    }
}
impl ConsoleBackground {
    fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        info!("add");
        let data = {
            let entt = world.get_entity(ctx.entity).unwrap();
            *entt.get::<Self>().unwrap()
        };
        world
            .commands()
            .entity(ctx.entity)
            .insert(BackgroundColor(data.0));
    }
}

#[derive(Component, Debug, Reflect, Clone, Copy)]
#[component(on_add=ConsoleNode::add)]
#[require(ConsoleFontSize, ConsoleBackground)]
pub struct ConsoleNode {
    left: Val,
    right: Val,
    top: Val,
    bottom: Val,
    width: Val,
    height: Val,
    min_width: Val,
    min_height: Val,
    max_width: Val,
    max_height: Val,
}
impl Default for ConsoleNode {
    fn default() -> Self {
        Self {
            left: Val::Vw(20.),
            right: Default::default(),
            top: Val::Vh(20.),
            bottom: Default::default(),
            width: Val::Vw(60.),
            height: Val::Vh(60.),
            min_width: Val::Vw(25.),
            min_height: Val::Vh(25.),
            max_width: Val::Vw(75.),
            max_height: Val::Vh(75.),
        }
    }
}
impl ConsoleNode {
    fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        info!("add");
        let data = {
            let entt = world.get_entity(ctx.entity).unwrap();
            *entt.get::<Self>().unwrap()
        };
        let font_size = {
            let entt = world.get_entity(ctx.entity).unwrap();
            *entt.get::<ConsoleFontSize>().unwrap()
        };
        let console_text_wrapper = (
            Node {
                overflow: Overflow::scroll_y(),
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: Val::Px(font_size.0).all(),
                display: Display::Flex,
                grid_column: GridPlacement::start_span(1, 1),
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::NoWrap,
                ..Default::default()
            },
            BackgroundColor(tailwind::RED_500.into()),
            ConsoleBodyTextWrapper,
            children![
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
                (Node::default(), ConsoleBodyText, Text::new("hi")),
            ],
        );
        let wrapper_wrapper = (
            Node {
                grid_column: GridPlacement::start_span(1, 1),
                grid_row: GridPlacement::start_span(1, 1),
                ..Default::default()
            },
            children![console_text_wrapper],
        );

        let bundle = (
            BackgroundColor(tailwind::SLATE_950.with_alpha(0.75).into()),
            Node {
                left: data.left,
                right: data.right,
                top: data.top,
                bottom: data.bottom,
                width: data.width,
                height: data.height,
                min_width: data.min_width,
                min_height: data.min_height,
                max_width: data.max_width,
                border: UiRect::all(Val::Px(1.)),
                display: Display::Grid,
                grid_template_rows: vec![GridTrack::auto(), GridTrack::px(*font_size * 2.)],
                grid_template_columns: vec![GridTrack::auto()],
                ..Default::default()
            },
            children![(wrapper_wrapper, ConsoleInput,)],
        );
        world.commands().entity(ctx.entity).insert(bundle);
    }
}

#[derive(Component, Debug, Reflect, Clone, Copy, Deref, DerefMut)]
pub struct ConsoleFontSize(f32);
impl Default for ConsoleFontSize {
    fn default() -> Self {
        Self(12.)
    }
}

#[derive(Component, Default, Debug, Reflect, Clone, Copy)]
#[require(ConsoleNode, ConsoleFontSize)]
pub struct Console;

#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
pub struct ConsoleBodyTextWrapper;
#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
pub struct ConsoleBodyText;

#[derive(Event, Clone, Debug)]
pub struct AppendToConsole(pub String);
#[derive(Event, Clone, Debug)]
pub struct ClearConsole;
#[derive(Event, Clone, Debug)]
pub struct CallConsoleCommand(pub String);

pub fn on_append_to_console(
    trigger: On<AppendToConsole>,
    q: Query<Entity, With<ConsoleBodyTextWrapper>>,
    mut commands: Commands,
) {
    for e in q {
        let text_entt = commands
            .spawn((
                Node {
                    grid_column: GridPlacement::start_span(1, 1),
                    grid_row: GridPlacement::start_span(2, 1),
                    ..Default::default()
                },
                ConsoleBodyText,
                Text::new(trigger.0.clone()),
            ))
            .id();
        commands.entity(e).add_child(text_entt);
    }
}
pub fn on_clear_console(
    _: On<ClearConsole>,
    q: Query<Entity, With<ConsoleBodyText>>,
    mut commands: Commands,
) {
    for e in q {
        commands.entity(e).despawn();
    }
}
pub fn on_call_console_command(trigger: On<CallConsoleCommand>) {
    debug!("MOCK: Calling command {}", trigger.0);
}

#[derive(Component, Debug, Default, Reflect, Clone, Copy)]
#[component(on_add = ConsoleInput::add)]
pub struct ConsoleInput;
impl ConsoleInput {
    fn add<'w>(mut world: DeferredWorld<'w>, ctx: HookContext) {
        info!("add");
        let bundle = (
            Node {
                grid_column: GridPlacement::start_span(1, 1),
                ..Default::default()
            },
            TextInput,
        );
        world.commands().entity(ctx.entity).insert(bundle);
    }
}

// submission event reader
fn on_submit_msg(
    mut reader: MessageReader<TextInputSubmitMessage>,
    filter: Query<Entity, With<ConsoleInput>>,
    mut commands: Commands,
) {
    for msg in reader.read() {
        // note: txtinput value cleared on submit
        if filter.iter().any(|e| e == msg.entity) {
            commands.trigger(CallConsoleCommand(msg.value.clone()));
            commands.trigger(AppendToConsole(msg.value.clone()));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins(TextInputPlugin);
    app.add_observer(on_append_to_console);
    app.add_observer(on_clear_console);
    app.add_observer(on_call_console_command);
    app.add_systems(Update, on_submit_msg.after(TextInputSystem));
}
