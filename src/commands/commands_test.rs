// use mockall::mock;
// use mockall_double::double;

// use super::execute;
// use super::ExecutableCommand;
// #[double]
// use super::Command::Backup;

// mock!{
//     pub ArgsBackup{}
//     impl ExecutableCommand for ArgsBackup{
//         fn execute(&self);
//     }
// }

// #[test]
// fn command_backup() {
//     let cmd = MockArgsBackup::new();
//     cmd.expect_execute().once();

//     execute(Backup(cmd));
// }
