mod app;

use app::CompilerDriver;

fn main() -> Result<(), String> {
    let driver = CompilerDriver::new()?;
    driver.run()
}
