extern crate gio;
extern crate gtk;
extern crate webkit2gtk;

use std::env::args;

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{AccelGroup, Application, ApplicationWindow, Builder};
use gtk::prelude::{BuilderExtManual, ContainerExt, GtkWindowExt, WidgetExt};
use webkit2gtk::{UserContentManager, WebContext, WebView, WebViewExtManual};
use webkit2gtk::traits::{SettingsExt, WebViewExt};

const DEFAULT_URI: &str = "https://duckduckgo.com";

fn build_ui(app: &Application) {
    let win_src = include_str!("../ui/window.xml");
    let win: ApplicationWindow = Builder::from_string(win_src)
        .object("application_window")
        .expect("couldn't get application window");
    win.set_application(Some(app));

    let grp = AccelGroup::new();
    win.add_accel_group(&grp);

    // webview
    let ctx = WebContext::default().unwrap();
    let mgr = UserContentManager::new();
    let web = WebView::new_with_context_and_user_content_manager(&ctx, &mgr);

    let ext = WebViewExt::settings(&web).unwrap();
    ext.set_enable_developer_extras(true);

    web.load_uri(DEFAULT_URI);

    win.add(&web);
    win.show_all();
}

fn main() {
    let app =
        Application::new(Some("local.pumpfoil.desktop"), Default::default());
    app.connect_activate(|a| {
        build_ui(a);
    });
    app.run_with_args(&args().collect::<Vec<_>>());
}
