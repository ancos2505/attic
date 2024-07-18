use anyhow::{anyhow, Result};
use crossbeam_channel::{bounded, select, Receiver};

pub type AppResult<T> = anyhow::Result<T>;

pub fn create_ctrl_channel() -> AppResult<Receiver<()>> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

pub fn check_ctrlc(ctrl_c_events: &Receiver<()>) -> Result<()> {
    select! {
        recv(ctrl_c_events) -> _ => {
            return Err(anyhow!("Ctrl-C pressed!"));
        }
        default => ()
    }

    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
