sed -i 's/mod tests {/mod tests {\n    use tempfile::tempdir;/g' paperpilot-cli/src/commands/group_b.rs
sed -i 's/mod tests {/mod tests {\n    use tempfile::tempdir;\n    use std::sync::{Arc, Mutex};/g' paperpilot-cli/src/commands/group_c.rs
