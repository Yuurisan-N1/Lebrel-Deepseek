pub mod create;
pub mod probe;
pub mod selftest;

pub async fn dispatch(command: &str, rest: &[String]) -> i32 {
    match command {
        "create" => {
            let count: usize = rest.first().and_then(|v| v.parse().ok()).unwrap_or(1);
            match create::run(count.max(1)).await {
                Ok(()) => 0,
                Err(err) => {
                    crate::support::log::lr(&format!("Command failed because of {}", err));
                    1
                }
            }
        }
        "probe" => match probe::run().await {
            Ok(()) => 0,
            Err(err) => {
                crate::support::log::lr(&format!("Command failed because of {}", err));
                1
            }
        },
        "selftest" => selftest::run(),
        "run" => match crate::job::run_loop::cycle().await {
            Ok(()) => 0,
            Err(err) => {
                crate::support::log::lr(&format!("Command failed because of {}", err));
                1
            }
        },
        _ => {
            crate::support::log::lr("Unknown command was given to the bot");
            1
        }
    }
}
