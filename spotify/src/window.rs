use xcb;

pub struct SpotifyWindow {
    conn: xcb::base::Connection,
    window: Option<xcb::xproto::Window>,
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

impl SpotifyWindow {
    pub fn new() -> SpotifyWindow {
        let (conn, screen_num) = xcb::Connection::connect(None).expect("couldn't connect to X");
        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(screen_num as usize)
            .expect("couldn't get X screen");

        let window = find_spotify(&conn, screen.root());
        println!("spotify: {:?}", window);
        SpotifyWindow { conn, window }
    }

    pub fn get_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(window) = self.window {
            let property = xcb::xproto::get_property(
                &self.conn,
                false,
                window,
                xcb::ATOM_WM_NAME,
                xcb::ATOM_STRING,
                0,
                1024,
            )
            .get_reply()?;
            // println!("{:?}", property.value_len());
            // println!("{:?}", property.value::<u8>());
            Ok(String::from_utf8(property.value().to_vec())?)
        } else {
            Ok(String::from(""))
        }
    }
}

fn find_spotify(
    conn: &xcb::Connection,
    window: xcb::xproto::Window,
) -> Option<xcb::xproto::Window> {
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
    .expect("couldn't get reply");

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
            .expect("couldn't get reply");
            let title = std::str::from_utf8(property.value())
                .expect("couldn't build UTF-8 string from reply");
            // println!("{:?}", title.as_bytes());

            if title != "spotify" {
                // println!("{}", title);
                return Some(window);
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