#[macro_export]
macro_rules! init_dirs {
    () => {
        directories::ProjectDirs::from("com", "jewelexx", "yanvm")
    };
}

#[macro_export]
macro_rules! init_pb {
    ($total:expr) => {{
        use indicatif::{ProgressBar, ProgressStyle};

        // Indicatif setup
        let pb = ProgressBar::new($total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]")
                .progress_chars("#>-"),
        );

        pb.enable_steady_tick(100);

        pb
    }};

    ($total:expr, $msg:expr) => {{
        let pb = init_pb!($total);
        pb.set_message($msg);

        pb
    }};
}
