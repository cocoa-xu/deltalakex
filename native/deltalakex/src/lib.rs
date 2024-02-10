use tokio::runtime;
use deltalake::errors::DeltaTableError;

mod error;
use error::DeltaLakexError;

#[rustler::nif(schedule = "DirtyIo")]
pub fn read_table(
    table_uri: &str
) -> Result<(), DeltaLakexError> {
    Ok(runtime::Builder::new_current_thread().build().unwrap().block_on(do_read_table(table_uri))?)
}

async fn do_read_table(table_uri: &str) -> Result<(), DeltaTableError> {
    let table = deltalake::open_table(table_uri).await?;
    if let Some(table_state) = table.state {
        let a = table_state.add_actions_table(false)?;
        print!("{:?}", a);
    }
    
    Ok(())
}

rustler::init!("Elixir.DeltaLake.Nif", [read_table]);
