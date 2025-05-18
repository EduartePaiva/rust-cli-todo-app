use rust_cli_todo::{Cli, Commands, Parser, todo};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { tasks } => todo::add(tasks),
        Commands::Rm { index } => todo::rm(index),
        Commands::Edit { index, content } => todo::edit(index, content),
        Commands::List => todo::list(),
        Commands::Done { indexs } => todo::done(indexs),
        Commands::Reset => todo::reset(),
        Commands::Restore => todo::restore(),
        Commands::Sort => todo::sort(),
    }
}
