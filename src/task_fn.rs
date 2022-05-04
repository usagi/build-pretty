pub type TaskFn = Box<dyn Fn(&mut String) -> Result<(), Box<dyn std::error::Error>>>;
