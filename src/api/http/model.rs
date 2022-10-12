use std::sync::Mutex;

pub(crate) struct AppState {
    pub app_name: String,
}

pub(crate) struct AppStateWithCounter {
    pub counter: Mutex<i32>,
}
