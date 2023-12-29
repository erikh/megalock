use crate::bindings::{
    xcb_xkb_event_type_t_XCB_XKB_EVENT_TYPE_MAP_NOTIFY,
    xcb_xkb_event_type_t_XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY,
    xcb_xkb_event_type_t_XCB_XKB_EVENT_TYPE_STATE_NOTIFY,
    xcb_xkb_map_part_t_XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS,
    xcb_xkb_map_part_t_XCB_XKB_MAP_PART_KEY_ACTIONS, xcb_xkb_map_part_t_XCB_XKB_MAP_PART_KEY_SYMS,
    xcb_xkb_map_part_t_XCB_XKB_MAP_PART_KEY_TYPES,
    xcb_xkb_map_part_t_XCB_XKB_MAP_PART_MODIFIER_MAP,
    xcb_xkb_map_part_t_XCB_XKB_MAP_PART_VIRTUAL_MODS,
    xcb_xkb_map_part_t_XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP,
};

pub const REQUIRED_MAP_PARTS: u32 = xcb_xkb_map_part_t_XCB_XKB_MAP_PART_KEY_TYPES
    | xcb_xkb_map_part_t_XCB_XKB_MAP_PART_KEY_SYMS
    | xcb_xkb_map_part_t_XCB_XKB_MAP_PART_MODIFIER_MAP
    | xcb_xkb_map_part_t_XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS
    | xcb_xkb_map_part_t_XCB_XKB_MAP_PART_KEY_ACTIONS
    | xcb_xkb_map_part_t_XCB_XKB_MAP_PART_VIRTUAL_MODS
    | xcb_xkb_map_part_t_XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP;

pub const REQUIRED_EVENTS: u32 = xcb_xkb_event_type_t_XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY
    | xcb_xkb_event_type_t_XCB_XKB_EVENT_TYPE_MAP_NOTIFY
    | xcb_xkb_event_type_t_XCB_XKB_EVENT_TYPE_STATE_NOTIFY;

pub const NET_ACTIVE_WINDOW: &str = "_NET_ACTIVE_WINDOW";
pub const NET_WM_BYPASS_COMPOSITOR: &str = "_NET_WM_BYPASS_COMPOSITOR";
pub const DPI_RESOURCE: &str = "Xft.dpi";
pub const CURS_INVISIBLE_BITS: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
pub const CURS_INVISIBLE_WIDTH: u32 = 8;
pub const CURS_INVISIBLE_HEIGHT: u32 = 8;

pub static GRAB_RETRY_DURATION: std::time::Duration = std::time::Duration::new(0, 500);
pub static GRAB_REDRAW_TIMEOUT: std::time::Duration = std::time::Duration::new(0, 100000);
