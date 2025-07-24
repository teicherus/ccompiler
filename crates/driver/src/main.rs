mod app;
mod errors;

use app::CompilerDriver;
use errors::DriverError;

fn main() -> Result<(), DriverError> {
    let driver = CompilerDriver::new()?;
    driver.run()
}
