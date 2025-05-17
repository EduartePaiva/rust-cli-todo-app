use clap::Parser;
use clap::Subcommand;

/// Simple todo App
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        visible_alias = "a",
        about("- Adds new task/s\n  Example: todo add \"buy carrots\"")
    )]
    Add {
        #[arg(required = true, help = "The name of the task to add")]
        tasks: Vec<String>,
    },

    #[command(
        visible_alias = "e",
        about("- Edits an existing task\n  Example: todo edit 1 banana")
    )]
    Edit {
        #[arg(required = true)]
        index: u32,
        #[arg(required = true)]
        content: String,
    },

    #[command(visible_alias = "l", about("- Lists all tasks\n  Example: todo list"))]
    List,

    #[command(
        visible_alias = "d",
        about(
            "- Marks task as done\n  Example: todo done 2 3 (marks second and third tasks as completed)"
        )
    )]
    Done {
        #[arg(required = true)]
        index: Vec<u32>,
    },

    #[command(visible_alias = "r", about("- Removes a task\n  Example: todo rm 4"))]
    Rm {
        #[arg(required = true)]
        index: u32,
    },

    #[command(about("- Delete all tasks"))]
    Reset,

    #[command(about("- Restore recent backup after reset"))]
    Restore,

    #[command(
        visible_alias = "s",
        about("- Sorts completed and uncompleted tasks\n  Example: todo sort")
    )]
    Sort,
}

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
