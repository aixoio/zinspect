#[macro_export]
macro_rules! print_error {
    ($err:expr) => {
        use owo_colors::OwoColorize;

        eprintln!("{} {}", "error:".bright_red().bold(), $err.bold());
    };
}

#[macro_export]
macro_rules! return_error {
    ($err:expr) => {
        use std::process::ExitCode;
        use $crate::print_error;

        print_error!($err);
        return ExitCode::FAILURE;
    };
}

#[macro_export]
macro_rules! handle_error {
    ($ex:expr) => {
        if let Err(err) = $ex {
            use $crate::return_error;

            return_error!(err);
        };
    };
}

#[macro_export]
macro_rules! match_error {
    ($ex:expr) => {
        match $ex {
            Ok(r) => r,
            Err(err) => {
                use $crate::return_error;

                return_error!(err);
            }
        }
    };
}

#[macro_export]
macro_rules! getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}
