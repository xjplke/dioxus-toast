use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastInfo, ToastManager};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch_web(App)
}

fn app() -> Element {
    let mut toast = use_signal(ToastManager::default);

    rsx! {
        ToastFrame {
            manager: toast
        },
        button {
            onclick: move |_| {
                toast.popup(ToastInfo::simple("123"));
            },
            "T"
        }
    }
}
