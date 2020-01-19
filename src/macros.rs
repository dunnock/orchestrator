/// reusable handler of future's result which even if succeds will turn into error
/// this also erases result and converts error into anyhow
#[macro_export]
macro_rules! should_not_complete {
    ( $text:expr, $res:expr ) => {
        match $res {
            Ok(_) => {
                info!("All the {} completed", $text);
                Err(anyhow!("All the {} exit", $text))
            }
            Err(err) => {
                error!("{} failure: {}", $text, err);
                Err(anyhow::Error::from(err))
            }
        }
    };
}

/// reusable handler of future's result which may succed
/// this also erases result and converts error into anyhow
#[macro_export]
macro_rules! may_complete {
    ( $text:expr, $res:expr ) => {
        match $res {
            Ok(_) => {
                info!("All the {} completed", $text);
                Ok(())
            }
            Err(err) => {
                error!("{} failure: {}", $text, err);
                Err(anyhow::Error::from(err))
            }
        }
    };
}

/// reusable handler of future's result which may fail where we log and drop failure
/// this also erases result
#[macro_export]
macro_rules! never_fail {
    ( $text:expr, $res:expr ) => {
        match $res {
            Ok(_) => {
                info!("All the {} completed", $text);
                Ok(()) as anyhow::Result<()>
            }
            Err(err) => {
                error!("{} failure: {}", $text, err);
                Ok(()) as anyhow::Result<()>
            }
        }
    };
}
