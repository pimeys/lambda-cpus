use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};

#[derive(Deserialize)]
struct CpuQuestion {
    dumb: u16,
}

#[derive(Serialize)]
struct CpuAnswer {
    logical: usize,
    physical: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(cpu_handler);
    Ok(())
}

fn cpu_handler(_: CpuQuestion, _: Context) -> Result<CpuAnswer, HandlerError> {
    Ok(CpuAnswer {
        logical: num_cpus::get(),
        physical: num_cpus::get_physical(),
    })
}
