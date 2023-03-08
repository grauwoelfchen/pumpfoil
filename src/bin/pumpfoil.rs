extern crate gio;
extern crate gtk;
extern crate webkit2gtk;

use std::env::args;

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{AccelGroup, Application, ApplicationWindow, Builder};
use gtk::prelude::{BuilderExtManual, ContainerExt, GtkWindowExt, WidgetExt};
use webkit2gtk::{
    CookieAcceptPolicy, CookiePersistentStorage, TLSErrorsPolicy,
    UserContentManager, WebContext, WebView, WebViewExtManual,
};
use webkit2gtk::traits::{
    CookieManagerExt, SettingsExt, WebContextExt, WebsiteDataManagerExt,
    WebViewExt,
};

// DevDocs (See a note about secure cookies on localhost)
// const DEFAULT_URI: &str = "http://localhost:9292";
const DEFAULT_URI: &str = "https://devdocs.io";
const COOKIE_FILE: &str = "./cookie.txt";

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
    let mgr_u = UserContentManager::new();
    let web = WebView::new_with_context_and_user_content_manager(&ctx, &mgr_u);

    let stg = WebViewExt::settings(&web).unwrap();
    stg.set_allow_file_access_from_file_urls(true);
    stg.set_allow_universal_access_from_file_urls(true);
    stg.set_enable_developer_extras(true);
    stg.set_enable_javascript(true);
    stg.set_enable_offline_web_application_cache(true);
    stg.set_enable_page_cache(true);
    stg.set_enable_smooth_scrolling(true);
    stg.set_enable_write_console_messages_to_stdout(true);
    stg.set_javascript_can_open_windows_automatically(true);
    stg.set_javascript_can_access_clipboard(true);

    // NOTE:
    // Unfortunately, It seems that WebKit does not support handling secure
    // cookies via HTTP on localhost, and also CSP is applied to it.
    // So there is probably no way to enable it at the moment :'(
    //
    // Secure Cookie related:
    //   * https://bugzilla.mozilla.org/show_bug.cgi?id=1618113
    //   * https://github.com/tauri-apps/tauri/issues/2604
    //   * https://github.com/tauri-apps/wry/issues/444
    // CSP related:
    //   * https://bugzilla.mozilla.org/show_bug.cgi?id=1447784
    //   * https://bugs.webkit.org/show_bug.cgi?id=250776

    let mgr_d = ctx.website_data_manager().unwrap();
    // ITP (Intelligent Tracking Prevention) treats cookie policy as `Always``
    mgr_d.set_itp_enabled(false);
    mgr_d.set_tls_errors_policy(TLSErrorsPolicy::Ignore);

    let mgr_c = ctx.cookie_manager().unwrap();
    // for non-session cookies
    mgr_c.set_persistent_storage(COOKIE_FILE, CookiePersistentStorage::Text);
    mgr_c.set_accept_policy(CookieAcceptPolicy::NoThirdParty);

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
