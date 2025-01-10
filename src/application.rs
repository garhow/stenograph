/* application.rs
 *
 * Copyright 2025 Garrett Howard
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gettextrs::gettext;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::StenographWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct StenographApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for StenographApplication {
        const NAME: &'static str = "StenographApplication";
        type Type = super::StenographApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for StenographApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for StenographApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = StenographWindow::new(&*application);
                window.upcast()
            });

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for StenographApplication {}
    impl AdwApplicationImpl for StenographApplication {}
}

glib::wrapper! {
    pub struct StenographApplication(ObjectSubclass<imp::StenographApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl StenographApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("stenograph")
            .application_icon("com.mersh.Stenograph")
            .developer_name("Garrett Howard")
            .version(VERSION)
            .developers(vec!["Garrett Howard"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2025 Garrett Howard")
            .build();

        about.present(Some(&window));
    }
}
