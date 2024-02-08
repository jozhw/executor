use std::process::Command;

/// Execute the file of interest within the current working directory that has been traversed.
///
/// # Arguments
///
/// * 'path' - Referenced str: The path to the executable file.
///
/// # Returns
///
/// ExitStatus object
pub fn execute_file(path: &str) -> Result<std::process::ExitStatus, std::io::Error> {
    // create a command to run the executable
    let mut command: Command = Command::new(path);

    // for future development: argument implementation for execution of script
    // need to make sure that args is an argument for execute_file and the type
    // casting is a vector of borrowed strings (Vec<&str>)
    //
    // for arg in args {
    //      command.arg(arg);
    // }

    // execute the command and return the result
    command.status()
}
