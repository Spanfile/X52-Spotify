use std::cell::RefCell;
use xcb;

pub struct SpotifyWindow {
    conn: xcb::base::Connection,
    root_window: xcb::xproto::Window,
    window: RefCell<Option<xcb::xproto::Window>>,
}

#[derive(Debug)]
struct ClassHint<'a> {
    name_bytes: &'a [u8],
    class_bytes: &'a [u8],
}

impl<'a> ClassHint<'a> {
    pub fn name(&self) -> &str {
        std::str::from_utf8(self.name_bytes).expect("couldn't interpret name_bytes as UTF-8")
    }

    pub fn class(&self) -> &str {
        std::str::from_utf8(self.class_bytes).expect("couldn't interpret class_bytes as UTF-8")
    }
}

#[derive(Debug)]
pub enum SpotifyError {
    GetPropertyFailed { source: xcb::base::GenericError },
    TitleNotUTF8 { source: std::string::FromUtf8Error },
}

impl std::fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpotifyError::GetPropertyFailed { .. } => write!(f, "get_property failed"),
            SpotifyError::TitleNotUTF8 { .. } => write!(f, "window title not UTF-8"),
        }
    }
}

impl SpotifyWindow {
    pub fn new() -> SpotifyWindow {
        let (conn, screen_num) = xcb::Connection::connect(None).expect("couldn't connect to X");
        let root_window = conn
            .get_setup()
            .roots()
            .nth(screen_num as usize)
            .expect("couldn't get X screen")
            .root();

        SpotifyWindow {
            conn,
            root_window,
            window: RefCell::new(None),
        }
    }

    pub fn get_title(&self) -> Result<Option<String>, SpotifyError> {
        let mut window = self.window.borrow_mut();
        if let Some(win) = *window {
            match self.get_window_title(win) {
                Ok(t) => Ok(Some(t)),
                Err(e) => {
                    if let SpotifyError::GetPropertyFailed { .. } = e {
                        println!("spotify disappeared most likely");
                        *window = None;
                        Ok(None)
                    } else {
                        Err(e)
                    }
                }
            }
        } else {
            match find_spotify(&self.conn, self.root_window) {
                Some((w, title)) => {
                    println!("spotify: {}", w);
                    *window = Some(w);
                    Ok(Some(title))
                }
                None => Ok(None),
            }
        }
    }

    fn get_window_title(&self, window: xcb::xproto::Window) -> Result<String, SpotifyError> {
        let property = match xcb::xproto::get_property(
            &self.conn,
            false,
            window,
            xcb::ATOM_WM_NAME,
            xcb::ATOM_STRING,
            0,
            1024,
        )
        .get_reply()
        {
            Ok(p) => p,
            Err(e) => return Err(SpotifyError::GetPropertyFailed { source: e }),
        };
        // println!("{:?}", property.value_len());
        // println!("{:?}", property.value::<u8>());
        match String::from_utf8(property.value().to_vec()) {
            Ok(s) => Ok(s),
            Err(e) => Err(SpotifyError::TitleNotUTF8 { source: e }),
        }
    }
}

fn find_spotify(
    conn: &xcb::Connection,
    window: xcb::xproto::Window,
) -> Option<(xcb::xproto::Window, String)> {
    let reply = xcb::xproto::get_property(
        &conn,
        false,
        window,
        xcb::ATOM_WM_CLASS,
        xcb::ATOM_STRING,
        0,
        1024,
    )
    .get_reply()
    .unwrap_or_else(|_| panic!("couldn't get reply for ATOM_WM_CLASS for {}", window));

    if reply.value_len() != 0 {
        let class = reply.value::<u8>();
        let class_index = class
            .iter()
            .position(|&b| b == 0)
            .expect("couldn't find end of name");

        let name_bytes = &class[0..class_index];
        let class_bytes = &class[class_index + 1..class.len() - 1];
        let class_hint = ClassHint {
            name_bytes,
            class_bytes,
        };

        // println!("{}: {} -> {}", window, reply.value_len(), class.len());
        // println!("{}: {:?}", window, class);
        // println!("{}: {} {}", window, class_hint.name(), class_hint.class());

        if class_hint.name() == "spotify" {
            let property = xcb::xproto::get_property(
                &conn,
                false,
                window,
                xcb::ATOM_WM_NAME,
                xcb::ATOM_STRING,
                0,
                1024,
            )
            .get_reply()
            .unwrap_or_else(|_| panic!("couldn't get reply for ATOM_WM_NAME for {}", window));
            let title = String::from_utf8(property.value().to_vec())
                .expect("couldn't build UTF-8 string from reply");
            // println!("{:?}", title.as_bytes());

            if title != "spotify" {
                // println!("{}", title);
                return Some((window, title));
            }
        }
    }

    for child in xcb::xproto::query_tree(&conn, window)
        .get_reply()
        .expect("couldn't query window tree")
        .children()
    {
        if let Some(child) = find_spotify(conn, *child) {
            return Some(child);
        }
    }

    None
}