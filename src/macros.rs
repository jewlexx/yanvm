#[macro_export]
macro_rules! init_dirs {
    () => {
        directories::ProjectDirs::from("com", "jewelexx", "yanvm")
    };
}
