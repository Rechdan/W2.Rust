use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use crate::{last_id::LastId, state::State};

pub static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

pub static LAST_ID: Lazy<LastId> = Lazy::new(|| LastId::default());

pub static STATE: Lazy<State> = Lazy::new(|| State::default());
