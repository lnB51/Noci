use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior, NSWindowStyleMask};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

#[tauri::command]
pub fn exit_app() -> Result<(), String> {
    std::process::exit(0);
}

impl std::ops::BitOr for crate::params::NSTrackingAreaOptions {
    type Output = u64;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u64 | rhs as u64
    }
}

static mut TRACK_VIEW_CLASS_REGISTERED: bool = false;

unsafe fn register_track_view_class() -> *const Class {
    if TRACK_VIEW_CLASS_REGISTERED {
        return Class::get("TrackView").unwrap();
    }

    let superclass = class!(NSView);
    let mut decl = ClassDecl::new("TrackView", superclass).unwrap();

    decl.add_method(
        sel!(mouseEntered:),
        mouse_entered as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(mouseExited:),
        mouse_exited as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(updateTrackingAreas),
        update_tracking_areas as extern "C" fn(&Object, Sel),
    );

    TRACK_VIEW_CLASS_REGISTERED = true;
    decl.register()
}

unsafe fn get_window_and_screen(this: &Object) -> Option<(id, id)> {
    let window: id = msg_send![this, window];
    if window == nil {
        return None;
    }
    let screen: id = msg_send![window, screen];
    if screen == nil {
        return None;
    }
    Some((window, screen))
}

unsafe fn calculate_initial_frame(screen: id) -> NSRect {
    let frame: NSRect = msg_send![screen, frame];
    let backing_scale_factor: f64 = msg_send![screen, backingScaleFactor];
    let logical_width = frame.size.width / backing_scale_factor;
    let width = logical_width * crate::params::INIT_WINDOW_WIDTH_RATIO;
    let physical_width = width * backing_scale_factor;
    let physical_height = crate::params::INIT_WINDOW_HEIGHT;
    let x = (frame.size.width - physical_width) / 2.0;
    let y = frame.size.height - physical_height;
    NSRect::new(
        NSPoint::new(x, y),
        NSSize::new(physical_width, physical_height),
    )
}

unsafe fn calculate_resized_frame(screen: id) -> NSRect {
    let frame: NSRect = msg_send![screen, frame];
    let width = crate::params::RESIZED_WINDOW_WIDTH;
    let height = crate::params::RESIZED_WINDOW_HEIGHT;
    let x = (frame.size.width - width) / 2.0;
    let y = frame.size.height - height;
    NSRect::new(NSPoint::new(x, y), NSSize::new(width, height))
}

extern "C" fn mouse_exited(this: &Object, _: Sel, _event: id) {
    unsafe {
        if let Some((window, screen)) = get_window_and_screen(this) {
            let new_frame = calculate_initial_frame(screen);
            let _: () = msg_send![window, setFrame: new_frame display: YES animate: YES];
        }
    }
}

extern "C" fn mouse_entered(this: &Object, _: Sel, _event: id) {
    unsafe {
        if let Some((window, screen)) = get_window_and_screen(this) {
            let new_frame = calculate_resized_frame(screen);
            let animator: id = msg_send![window, animator];
            let _: () = msg_send![animator, setFrame: new_frame display: YES];
        }
    }
}

extern "C" fn update_tracking_areas(this: &Object, _: Sel) {
    unsafe {
        let existing_areas: id = msg_send![this, trackingAreas];
        let count: usize = msg_send![existing_areas, count];
        for i in 0..count {
            let area: id = msg_send![existing_areas, objectAtIndex: i];
            let _: () = msg_send![this, removeTrackingArea: area];
        }

        let frame: NSRect = msg_send![this, bounds];

        let options = crate::params::NSTrackingAreaOptions::NSTrackingMouseEnteredAndExited as u64
            | crate::params::NSTrackingAreaOptions::NSTrackingActiveAlways as u64
            | crate::params::NSTrackingAreaOptions::NSTrackingInVisibleRect as u64;

        let tracking_area: id = msg_send![class!(NSTrackingArea), alloc];
        let tracking_area: id = msg_send![tracking_area,
            initWithRect: frame
            options: options
            owner: this
            userInfo: nil
        ];
        let _: () = msg_send![this, addTrackingArea: tracking_area];
    }
}

pub fn create_native_notch_window(window: &tauri::WebviewWindow) {
    unsafe {
        let ns_window_ptr = window.ns_window().expect("Failed to get ns_window");
        let ns_window: id = ns_window_ptr as id;

        ns_window.setStyleMask_(NSWindowStyleMask::NSBorderlessWindowMask);
        let _: () = msg_send![ns_window, setOpaque: NO];
        let clear_color: id = msg_send![class!(NSColor), clearColor];
        let _: () = msg_send![ns_window, setBackgroundColor: clear_color];
        let _: () = msg_send![ns_window, setHasShadow: NO];
        ns_window.setLevel_(crate::params::NOTCH_WINDOW_LEVEL);

        let _: () = msg_send![ns_window, setIgnoresMouseEvents: NO];
        let _: () = msg_send![ns_window, setAcceptsMouseMovedEvents: YES];

        ns_window.setCollectionBehavior_(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
        );

        let content_view: id = ns_window.contentView();
        let bounds: NSRect = msg_send![content_view, bounds];

        let track_view_class = register_track_view_class();
        let custom_view: id = msg_send![track_view_class, alloc];
        let custom_view: id = msg_send![custom_view, initWithFrame: bounds];

        // Transparent custom view setup
        let _: () = msg_send![custom_view, setWantsLayer: YES];
        let layer: id = msg_send![custom_view, layer];
        let clear_color: id = msg_send![class!(NSColor), clearColor];
        let _: () = msg_send![layer, setBackgroundColor: clear_color];
        let _: () = msg_send![layer, setOpaque: NO];

        let _: () = msg_send![content_view, addSubview: custom_view];
        let _: () = msg_send![custom_view, updateTrackingAreas];

        ns_window.makeKeyAndOrderFront_(nil);
    }
}