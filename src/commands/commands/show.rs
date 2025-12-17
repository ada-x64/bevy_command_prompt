use clap::Parser;
use regex::Regex;

use crate::prelude::*;

/// Provides static information about the world.
#[derive(Parser, Message, Clone)]
#[command(name = "show")]
pub struct ShowCmd {
    #[arg(required = true)]
    pub kind: ShowKind,
    /// Filter for the request. By default will act like `grep`, i.e.
    /// will match substrings. Pass `-e` to search as a regular expression.
    #[arg(short, long, global = true, required_if_eq("use_expression", "true"))]
    pub filter: Option<String>,
    /// Pass this flag to interpret the filter as a regular expression.
    #[arg(short = 'e')]
    pub use_expression: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum ShowKind {
    Commands,
    Components,
    Archetypes,
    Types,
    Resources,
}

fn find_inner(input: In<CommandMsg<ShowCmd>>, world: &mut World) {
    let mut vec: Vec<String> = match input.command.kind {
        ShowKind::Commands => world
            .resource::<ConsoleCommands>()
            .values()
            .map(|v| v.cmd.clone().render_usage().ansi().to_string())
            .collect(),
        ShowKind::Components => world
            .components()
            .iter_registered()
            .map(|c| {
                let required = c
                    .required_components()
                    .iter_ids()
                    .map(|c| {
                        world
                            .components()
                            .iter_registered()
                            .find(|i| i.id() == c)
                            .unwrap()
                            .name()
                            .to_string()
                            + ", "
                    })
                    .collect::<String>();
                format!(
                    "{} | {:?} | {} | {:?} | Requires ({})\n",
                    c.name(),
                    c.id(),
                    if c.mutable() { "mutable" } else { "immutable" },
                    c.storage_type(),
                    required,
                )
            })
            .collect(),

        ShowKind::Archetypes => world
            .archetypes()
            .iter()
            .map(|a| {
                let mut entity_table = a.entities_with_location().collect::<Vec<_>>();
                entity_table.sort_by(|(_, l), (_, l2)| {
                    l.archetype_row.index().cmp(&l2.archetype_row.index())
                });
                let component_table = a
                    .iter_components()
                    .map(|c| unsafe { world.components().get_info_unchecked(c) });
                let table = world.storages().tables.get(a.table_id()).unwrap();
                let component_header = component_table
                    .clone()
                    .fold("  row | entity".to_string(), |header, info| {
                        format!("{header} | {}", info.name())
                    });
                let entity_table = entity_table
                    .iter()
                    .map(|(e, l)| {
                        let row = component_table.clone().fold(String::new(), |row, info| {
                            format!(
                                "{row} | {1:^0$}",
                                info.name().len(),
                                if table.has_column(info.id()) { "x" } else { "" }
                            )
                        });
                        format!("{:>4} | {:>6}{row}\n", l.archetype_row.index(), e)
                    })
                    .collect::<String>();
                let header = format!(
                    "Archetype {:?} :: {} entities, {} components",
                    a.table_id(),
                    a.entities().len(),
                    a.component_count()
                );
                let separator = (0..header.len()).map(|_| '-').collect::<String>();
                format!("{header}\n{separator}\n{component_header}\n{entity_table}",)
            })
            .collect::<Vec<String>>(),
        ShowKind::Types => world
            .resource::<AppTypeRegistry>()
            .0
            .read()
            .iter()
            .map(|t| format!("{:?}", t.type_info()))
            .collect(),
        ShowKind::Resources => world
            .storages()
            .resources
            .iter()
            .map(|(id, _data)| {
                let c = world.components().get_info(id).unwrap();
                format!(
                    "{} | {:?} | {} | {:?} \n",
                    c.name(),
                    c.id(),
                    if c.mutable() { "mutable" } else { "immutable" },
                    c.storage_type(),
                )
            })
            .collect(),
    };
    if let Some(filter) = input.command.filter.as_ref() {
        vec = if input.command.use_expression {
            vec.into_iter().filter(|val| val.contains(filter)).collect()
        } else {
            let expr = Regex::new(filter);
            if let Err(e) = expr {
                input.println(
                    &mut world.commands(),
                    format!("Failed to parse regular expression.\nError: {e:?}"),
                );
                return;
            }
            let expr = expr.unwrap();
            vec.into_iter().filter(|val| expr.is_match(val)).collect()
        };
    }
    let str = vec
        .iter()
        .fold(String::new(), |prev, next| format!("{prev}\n{next}"));
    input.println(&mut world.commands(), str);
}

fn on_find_msg(mut reader: MessageReader<CommandMsg<ShowCmd>>, mut commands: Commands) {
    for msg in reader.read() {
        commands.run_system_cached_with(find_inner, msg.clone());
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, on_find_msg);
    app.add_console_command::<ShowCmd>();
}
