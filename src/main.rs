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
    /// Move container to workspace
    #[clap(short, long = "move", value_parser)]
    mv: Option<usize>,
    #[clap(long, exclusive(true))]
    list_workspaces: bool,
    #[clap(long, exclusive(true))]
    list_outputs: bool,
}

fn get_output(connection: &mut I3Connection, name: &str) -> Result<(Output, usize)> {
    let os = connection.get_outputs()?;

    let position = os
        .outputs
        .iter()
        .filter(|output| output.name != "xroot-0")
        .position(|output| output.name == name)
        .ok_or_else(|| anyhow!("Error finding correct output"))?;

    Ok((
        os.outputs
            .into_iter()
            .filter(|output| output.name != "xroot-0")
            .nth(position)
            .unwrap(),
        position,
    ))
}

fn get_focused_output(connection: &mut I3Connection) -> Result<(Output, usize)> {
    let ws = connection.get_workspaces()?;

    let focused = ws
        .workspaces
        .iter()
        .find(|workspace| workspace.focused)
        .ok_or_else(|| anyhow!("Error finiding workspace with focus"))?;

    get_output(connection, &focused.output)
}

fn main() {
    if std::env::args().len() == 1 {
        let mut app = Args::command();
        app.print_help().expect("Error on print help");
        process::exit(1);
    }
    let args = Args::parse();
    let mut connection = I3Connection::connect().expect("Cannot connect to I3 ipc");

    if let Some(ref output) = args.output {
        if !connection
            .get_outputs()
            .expect("Cannot get outputs")
            .outputs
            .iter_mut()
            .any(|o| &o.name == output)
        {
            println!("Wrong output");
            process::exit(1);
        }
    }

    if args.list_workspaces {
        println!(
            "{:?}",
            connection
                .get_workspaces()
                .expect("Error getting workspaces")
                .workspaces
                .iter()
                .map(|w| w.name.clone())
                .collect::<Vec<String>>()
        );
        process::exit(0);
    }
    if args.list_outputs {
        println!(
            "{:?}",
            connection
                .get_outputs()
                .expect("Error getting output")
                .outputs
                .iter()
                .filter(|o| o.name != "xroot-0")
                .map(|w| w.name.clone())
                .collect::<Vec<String>>()
        );
        process::exit(0);
    }

    // -w ... + optional -o
    if let Some(workspace_id) = args.workspace {
        let (output, output_idx) = if let Some(output_arg) = args.output {
            get_output(&mut connection, &output_arg).expect("Error getting named output")
        } else {
            get_focused_output(&mut connection).expect("Error retrieving focused output ")
        };

        let workspace_number = NUM_WORKSPACES_PER_OUTPUT * output_idx + workspace_id;

        connection
            .run_command(&format!(
                "workspace {}, move workspace to output {}",
                workspace_number, output.name
            ))
            .expect("Error on command -w -o");

    } else if let Some(output_name) = args.output {
        let (output, _) =
            get_output(&mut connection, &output_name).expect("Error getting named workspace");
        let workspace_name = output.current_workspace.unwrap();
        let workspace = connection
            .get_workspaces()
            .expect("Error getting workspaces")
            .workspaces
            .into_iter()
            .find(|ws| ws.name == workspace_name)
            .expect("Error on find() -o workspace");

        connection
            .run_command(&format!("workspace {}", workspace.name))
            .expect("Error on command -o");

    } else if let Some(mv_id) = args.mv {
        let (_, output_idx) =
            get_focused_output(&mut connection).expect("Error getting focused output");
        let workspace_number = NUM_WORKSPACES_PER_OUTPUT * output_idx + mv_id;
        connection
            .run_command(&format!(
                "move container to workspace number {}",
                workspace_number
            ))
            .expect("Error on move");
    }
}
