use rust_cli_todo::{Cli, Commands, Parser, todo};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { tasks } => {
            todo::add(tasks);
        }
        Commands::Rm { index } => {
            todo::rm(index);
        }
        Commands::Edit { index, content } => todo!(),
        Commands::List => todo!(),
        Commands::Done { index } => todo!(),
        Commands::Reset => todo!(),
        Commands::Restore => todo!(),
        Commands::Sort => todo!(),
    }
}
