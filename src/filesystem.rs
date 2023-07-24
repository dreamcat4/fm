use gtk::{gio, glib, prelude::*};
use relm4::{gtk, Sender};

use crate::component::app::AppMsg;
use crate::ops;

/// Move a dropped file into the destination directory.
pub fn handle_drop(value: &glib::Value, destination: &gio::File, sender: Sender<AppMsg>) {
    let file = value.get::<gio::File>().unwrap();

    let destination_file = destination.child(file.basename().unwrap());

    if destination_file.equal(&file) {
        return;
    }

    relm4::spawn_local(ops::move_(file, destination_file, sender));
}
