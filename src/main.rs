use std::process;

use i3ipc::{reply::Output, I3Connection};

use anyhow::{anyhow, Result};
use clap::{CommandFactory, Parser};

const NUM_WORKSPACES_PER_OUTPUT: usize = 10;

/// Program that switches workspaces in a way that is decoupled from displays
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Workspace to change on current output
    #[clap(short, long, value_parser)]
    workspace: Option<usize>,
    /// Change to current workspace on given output
    #[clap(short, long, value_parser)]
    output: Option<String>,
}

fn get_focused_output(connection: &mut I3Connection) -> Result<(Output, usize)> {
    let ws = connection.get_workspaces()?;

    let focused = ws
        .workspaces
        .iter()
        .find(|workspace| workspace.focused)
        .ok_or_else(|| anyhow!("Error finiding workspace with focus"))?;

    let os = connection.get_outputs()?;

    let position = os
        .outputs
        .iter()
        .filter(|output| output.name != "xroot-0")
        .position(|output| output.name == focused.output)
        .ok_or_else(|| anyhow!("Error finding correct ouput"))?;

    Ok((
        os.outputs
            .into_iter()
            .filter(|output| output.name != "xroot-0")
            .nth(position)
            .unwrap(),
        position,
    ))
}

fn main() {
    if std::env::args().len() == 1 {
        let mut app = Args::command();
        app.print_help().expect("Error on print help");
        process::exit(1);
    }

    let args = Args::parse();

    let mut connection = I3Connection::connect().unwrap();

    let (focused_output, output_idx) =
        get_focused_output(&mut connection).expect("Cannot retrieve focused output");

    let outcome = if let Some(workspace_idx) = args.workspace {
        let workspace_number = NUM_WORKSPACES_PER_OUTPUT * output_idx + workspace_idx;
        eprintln!(
            "WORKSPACE {} output {}",
            workspace_number, focused_output.name
        );

        connection.run_command(&format!(
            "workspace {} output {}",
            workspace_number, focused_output.name
        ))
    } else if let Some(_output) = args.output {
        todo!()
    } else {
        panic!("You can't be here");
    };

    if let Err(msg_error) = outcome {
        eprintln!("COMMAND ERROR: {}", msg_error);
    }
}
