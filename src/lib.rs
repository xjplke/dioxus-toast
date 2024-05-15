#![allow(non_snake_case)]

mod id;

use std::collections::BTreeMap;

use dioxus::prelude::*;
use id::ID;

#[derive(Debug, Clone, PartialEq)]
struct ToastManagerItem {
    info: ToastInfo,
    hide_after: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastManager {
    list: BTreeMap<usize, ToastManagerItem>,
    maximum_toast: u8,
    id_manager: ID,
}

impl ToastManager {
    pub fn new(maximum_toast: u8) -> Self {
        Self {
            list: BTreeMap::new(),
            maximum_toast,
            id_manager: ID::new(),
        }
    }

    pub fn popup(&mut self, info: ToastInfo) -> usize {
        let toast_id = self.id_manager.add();

        if self.list.len() >= self.maximum_toast.into() {
            if let Some(result) = self.list.first_key_value() {
                let id = *result.0;
                println!("Deleted Toast ID: {:?}", id);
                self.list.remove(&id);
            }
        }

        let hide_after = info
            .hide_after
            .map(|duration| chrono::Local::now().timestamp() + duration as i64);

        self.list
            .insert(toast_id, ToastManagerItem { info, hide_after });

        toast_id
    }

    pub fn remove(&mut self, id: usize) {
        self.list.remove(&id);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Default for ToastManager {
    fn default() -> Self {
        Self {
            list: Default::default(),
            maximum_toast: 6,
            id_manager: ID::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Position {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Icon {
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastInfo {
    pub heading: Option<String>,
    pub context: String,
    pub allow_toast_close: bool,
    pub position: Position,
    pub icon: Option<Icon>,
    pub hide_after: Option<usize>,
}

impl ToastInfo {
    pub fn simple(text: &str) -> Self {
        Self {
            heading: None,
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: None,
            hide_after: Some(6),
        }
    }

    pub fn success(text: &str, heading: &str) -> Self {
        Self {
            heading: Some(heading.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: Some(Icon::Success),
            hide_after: Some(6),
        }
    }

    pub fn warning(text: &str, heading: &str) -> Self {
        Self {
            heading: Some(heading.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: Some(Icon::Warning),
            hide_after: Some(6),
        }
    }

    pub fn info(text: &str, heading: &str) -> Self {
        Self {
            heading: Some(heading.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: Some(Icon::Info),
            hide_after: Some(6),
        }
    }

    pub fn error(text: &str, heading: &str) -> Self {
        Self {
            heading: Some(heading.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            icon: Some(Icon::Error),
            hide_after: Some(6),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToastFrameProps {
    manager: Signal<ToastManager>,
}

pub fn ToastFrame(props: ToastFrameProps) -> Element {
    let mut manager = props.manager;

    let toast_list = &manager.read().list;

    let mut bottom_left_ele: Vec<VNode> = vec![];
    let mut bottom_right_ele: Vec<VNode> = vec![];
    let mut top_left_ele: Vec<VNode> = vec![];
    let mut top_right_ele: Vec<VNode> = vec![];

    for (_, (id, item)) in toast_list.iter().enumerate() {
        let current_id = *id;

        let icon_class = if let Some(icon) = &item.info.icon {
            let mut class = String::from("has-icon ");

            match icon {
                Icon::Success => class.push_str("icon-success"),
                Icon::Warning => class.push_str("icon-warning"),
                Icon::Error => class.push_str("icon-error"),
                Icon::Info => class.push_str("icon-info"),
            }

            class
        } else {
            String::new()
        };

        let element = rsx! {
            div {
                class: "toast-single {icon_class}",
                id: "{id}",
                if item.info.allow_toast_close {
                    div {
                        class: "close-toast-single",
                        onclick: move |_| {
                            manager.write().list.remove(&current_id);
                        },
                        "×",
                    }
                }
                if let Some(v) = &item.info.heading {
                    h2 {
                        class: "toast-heading",
                        "{v}"
                    }
                }

                span {
                    dangerous_inner_html: "{item.info.context}",
                }
            }
        }
        .unwrap();

        if item.info.position == Position::BottomLeft {
            bottom_left_ele.push(element);
        } else if item.info.position == Position::BottomRight {
            bottom_right_ele.push(element);
        } else if item.info.position == Position::TopLeft {
            top_left_ele.push(element);
        } else if item.info.position == Position::TopRight {
            top_right_ele.push(element);
        }
    }

    let _res = use_resource(move || {
        let mut toast_manager = manager.clone();
        async move {
            loop {
                let timer_list = toast_manager.read().list.clone();
                for (id, item) in &timer_list {
                    if let Some(hide_after) = item.hide_after {
                        if chrono::Local::now().timestamp() >= hide_after {
                            toast_manager.write().list.remove(id);
                        }
                    }
                }
                time_sleep(100).await;
            }
        }
    });

    rsx! {
        div {
            class: "toast-scope",
            //style {  include_str!("./assets/toast.css")  },
            div {
                class: "toast-wrap bottom-left",
                id: "wrap-bottom-left",
                {bottom_left_ele.into_iter()}
            }
            div {
                class: "toast-wrap bottom-right",
                id: "wrap-bottom-right",
                {bottom_right_ele.into_iter()}
            }
            div {
                class: "toast-wrap top-left",
                id: "wrap-top-left",
                {top_left_ele.into_iter()}
            }
            div {
                class: "toast-wrap top-right",
                id: "wrap-top-right",
                {top_right_ele.into_iter()}
            }
        }
    }
}

#[cfg(feature = "web")]
async fn time_sleep(interval: usize) {
    gloo_timers::future::TimeoutFuture::new(interval as u32).await;
}

#[cfg(feature = "desktop")]
async fn time_sleep(interval: usize) {
    tokio::time::sleep(tokio::time::Duration::from_millis(interval as u64)).await;
}
