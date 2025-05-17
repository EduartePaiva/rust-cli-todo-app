use rust_cli_todo::{Cli, Commands, Parser};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { tasks } => {
            println!("{:?}", tasks);
        }
        Commands::Rm { index } => println!("{index}"),
        Commands::Edit { index, content } => todo!(),
        Commands::List => todo!(),
        Commands::Done { index } => todo!(),
        Commands::Reset => todo!(),
        Commands::Restore => todo!(),
        Commands::Sort => todo!(),
    }
}
