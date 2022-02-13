extern crate gio;
extern crate gtk;

use std::env::args;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{AccelGroup, Application, ApplicationWindow, Builder};

fn build_ui(app: &Application) {
    let awin_src = include_str!("../ui/window.xml");
    let awin: ApplicationWindow = Builder::from_string(awin_src)
        .get_object("window")
        .expect("couldn't get window");
    awin.set_application(Some(app));

    let accl_grp = AccelGroup::new();
    awin.add_accel_group(&accl_grp);
    awin.show_all();
}

fn main() {
    let application =
        Application::new(Some("local.pumpfoil.desktop"), Default::default())
            .expect("couldn't get initialized");
    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
