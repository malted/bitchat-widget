use accessibility::{AXAttribute, AXUIElement, TreeVisitor, TreeWalker, TreeWalkerFlow};
use core_foundation::{array::CFArray, string::CFString};
use status_bar::{Menu, MenuItem, StatusItem, async_infinite_event_loop};
use std::cell::Cell;
use std::process::Command;
use tokio::{spawn, time};

struct PrintyBoi {
    peer_count: Cell<usize>,
    children: AXAttribute<CFArray<AXUIElement>>,
}

impl PrintyBoi {
    pub fn new() -> Self {
        Self {
            peer_count: Cell::new(0),
            children: AXAttribute::children(),
        }
    }
}

impl TreeVisitor for PrintyBoi {
    fn enter_element(&self, element: &AXUIElement) -> TreeWalkerFlow {
        let re = regex::Regex::new(r"(?<count>\d+) connected (person|people)").unwrap();

        if let Some(label) = element
            .attribute(&AXAttribute::new(&CFString::new("AXLabel")))
            .ok()
            .map(|t| t.downcast::<CFString>())
            .flatten()
        {
            let re = regex::Regex::new(r"(?<count>\d+) connected (person|people)").unwrap();

            if let Some(caps) = re.captures(&label.to_string()) {
                println!("The connected people count is: {}", &caps["count"]);
            }
        }

        if let Ok(names) = element.attribute_names() {
            for name in names.into_iter() {
                if &*name == self.children.as_CFString()
                    || *name == CFString::new("AXLabel")
                    || *name == CFString::new("AXTitle")
                {
                    continue;
                }
                if let Ok(value) = element.attribute(&AXAttribute::new(&name)) {
                    if let Some(count) = re
                        .captures(&format!("{:?}", value))
                        .map(|r| r["count"].parse::<usize>().ok())
                        .flatten()
                    {
                        self.peer_count.replace(count);
                    }
                }
            }
        }

        TreeWalkerFlow::Continue
    }

    fn exit_element(&self, _element: &AXUIElement) {}
}

static TARGET_PROCESS_NAME: &str = "bitchat";

fn look_up_pid() -> Option<u32> {
    let out = Command::new("pgrep")
        .args(["-f", TARGET_PROCESS_NAME])
        .output()
        .ok()?
        .stdout
        .strip_suffix("\n".as_bytes())?
        .to_vec();

    let pid_str = std::str::from_utf8(&out).unwrap().trim();
    pid_str.parse::<u32>().ok()
}

fn get_count() -> Option<usize> {
    let pid = match look_up_pid() {
        Some(p) => p as i32,
        None => return None,
    };

    let app = AXUIElement::application(pid);
    let printy = PrintyBoi::new();
    let walker = TreeWalker::new();

    walker.walk(&app, &printy);

    Some(printy.peer_count.into_inner())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut status_item = StatusItem::new("", Menu::new(vec![]));

    spawn(async_infinite_event_loop(time::sleep));

    status_item.set_menu(Menu::new(vec![MenuItem::new(
        format!("open bitchat"),
        Some(Box::new(|| {
            let _ = std::process::Command::new("open")
                .arg("/System/Volumes/Data/Applications/bitchat.app")
                .spawn();
        })),
        None,
    )]));

    loop {
        let peer_count = get_count();
        let peer_count_msg = match peer_count {
            Some(count) => {
                format!("{count} peer{}", if count != 1 { "s" } else { "" })
            }
            None => "? peers".to_string(),
        };

        status_item.set_title(peer_count_msg);

        status_item.set_image(if peer_count.unwrap_or(0) == 0 {
            "person.2"
        } else {
            "person.2.fill"
        });

        time::sleep(time::Duration::from_secs(5)).await;
    }
}
