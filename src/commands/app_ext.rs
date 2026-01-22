use crate::prelude::*;

pub trait ConsoleCommand: clap::Parser + Send + Sync + 'static {}
impl<T> ConsoleCommand for T where T: clap::Parser + Send + Sync + 'static {}

pub trait CommandExt {
    /// Registers a console command to the application.
    fn add_console_command<T: ConsoleCommand>(&mut self) -> &mut Self;
}
impl CommandExt for App {
    // TODO: If a command is not initialized, it will panic.
    // Make sure to add a nice error message if possible.
    fn add_console_command<T: ConsoleCommand>(&mut self) -> &mut Self {
        self.add_message::<CommandMsg<T>>();
        let cmd = T::command().no_binary_name(true);
        let name = cmd.get_name();
        let dispatch = self.world_mut().register_system(dispatch_cmd::<T>);
        let mut cmds = self.world_mut().resource_mut::<ConsoleCommands>();
        cmds.insert(name.to_string(), ConcreteConsoleCommand { cmd, dispatch });
        self
    }
}

fn dispatch_cmd<T: ConsoleCommand>(
    input: In<SubmitEvent>,
    mut writer: MessageWriter<CommandMsg<T>>,
    mut commands: Commands,
) {
    let res: Result<(), String> = (|| {
        let res = T::try_parse_from(input.args().iter()).map_err(|e| format!("{e}"))?;
        writer.write(CommandMsg {
            command: res,
            console_id: input.console_id(),
        });
        Ok(())
    })();
    if let Err(e) = res {
        commands.write_message(ConsoleWriteMsg {
            message: e + "\n",
            console_id: input.console_id(),
        });
    }
}
