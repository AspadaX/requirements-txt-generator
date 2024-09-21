pub trait VirtualEnvironmentPackage {
    // get the name of the package
    // clone operation, no move
    fn get_name(&self) -> String;
    // get the version of the package
    // clone operation, no move
    fn get_version(&self) -> String;
}
