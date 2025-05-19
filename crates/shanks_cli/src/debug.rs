use crate::cli::DebugArgs;

pub fn debug(args: DebugArgs) {
    println!("Debugging...");
    println!("{:?}", args);
}
