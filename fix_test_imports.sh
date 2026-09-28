sed -i 's/use tempfile::tempdir;//g' paperpilot-cli/src/commands/group_b.rs
sed -i 's/use tempfile::tempdir;//g' paperpilot-cli/src/commands/group_c.rs
sed -i '/mod tests {/a \    use tempfile::tempdir;' paperpilot-cli/src/commands/group_b.rs
sed -i '/mod tests {/a \    use tempfile::tempdir;' paperpilot-cli/src/commands/group_c.rs
