use std::error::Error;

pub trait Lifecycle {
    fn on_start(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn on_run(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn on_stop(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn on_cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct LifecycleManager<T: Lifecycle> {
    app: T,
}

impl<T: Lifecycle> LifecycleManager<T> {
    pub fn new(app: T) -> Self {
        Self { app }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.app.on_start()?;
        self.app.on_run()?;
        self.app.on_stop()?;
        self.app.on_cleanup()?;
        Ok(())
    }
}
