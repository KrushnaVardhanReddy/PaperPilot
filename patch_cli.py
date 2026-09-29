with open("paperpilot-cli/src/cli.rs", "r") as f:
    text = f.read()

cli_commands = """
    // --- Group D: Conversion ---
    Convert {
        #[arg(long, required = true)]
        format: String,
        #[arg(long, required = true)]
        input: std::path::PathBuf,
        #[arg(long)]
        output: Option<std::path::PathBuf>,
    },
    Hash {
        #[arg(long, required = true)]
        input: std::path::PathBuf,
    },
"""

text = text.replace("    Validate {\n        #[arg(long)]\n        input: PathBuf,\n    },\n}", "    Validate {\n        #[arg(long)]\n        input: PathBuf,\n    },\n" + cli_commands + "}")

with open("paperpilot-cli/src/cli.rs", "w") as f:
    f.write(text)

with open("paperpilot-cli/src/commands/mod.rs", "r") as f:
    text = f.read()

mod_add = """pub mod group_d;\n"""
text = text.replace("pub mod validate;", "pub mod validate;\n" + mod_add)

handlers = """        // Group D
        Commands::Convert { format, input, output } => group_d::handle_convert(format, input, output.as_deref()),
        Commands::Hash { input } => group_d::handle_hash(input),
"""
text = text.replace("        Commands::Validate { input } => validate::handle_validate(input),\n    }\n}", "        Commands::Validate { input } => validate::handle_validate(input),\n\n" + handlers + "    }\n}")

with open("paperpilot-cli/src/commands/mod.rs", "w") as f:
    f.write(text)
