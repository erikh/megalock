#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use crate::{
    bindings::{
        cairo_create, cairo_destroy, cairo_fill, cairo_rectangle, cairo_set_source_rgb,
        cairo_surface_destroy, cairo_surface_t, cairo_t, cairo_xcb_surface_create,
        xcb_atom_enum_t_XCB_ATOM_CARDINAL, xcb_atom_enum_t_XCB_ATOM_STRING,
        xcb_atom_enum_t_XCB_ATOM_WINDOW, xcb_atom_enum_t_XCB_ATOM_WM_CLASS,
        xcb_atom_enum_t_XCB_ATOM_WM_NAME, xcb_atom_t, xcb_aux_sync, xcb_change_property,
        xcb_change_window_attributes, xcb_clear_area, xcb_config_window_t_XCB_CONFIG_WINDOW_HEIGHT,
        xcb_config_window_t_XCB_CONFIG_WINDOW_STACK_MODE,
        xcb_config_window_t_XCB_CONFIG_WINDOW_WIDTH, xcb_configure_window, xcb_connect,
        xcb_connection_t, xcb_create_cursor, xcb_create_gc, xcb_create_pixmap,
        xcb_create_pixmap_from_bitmap_data, xcb_create_window, xcb_cursor_t, xcb_cw_t,
        xcb_cw_t_XCB_CW_BACK_PIXEL, xcb_cw_t_XCB_CW_BACK_PIXMAP, xcb_cw_t_XCB_CW_EVENT_MASK,
        xcb_cw_t_XCB_CW_OVERRIDE_REDIRECT, xcb_depth_iterator_t, xcb_depth_next,
        xcb_depth_visuals_iterator, xcb_destroy_notify_event_t,
        xcb_event_mask_t_XCB_EVENT_MASK_EXPOSURE, xcb_event_mask_t_XCB_EVENT_MASK_KEY_PRESS,
        xcb_event_mask_t_XCB_EVENT_MASK_KEY_RELEASE,
        xcb_event_mask_t_XCB_EVENT_MASK_STRUCTURE_NOTIFY,
        xcb_event_mask_t_XCB_EVENT_MASK_VISIBILITY_CHANGE, xcb_extension_t, xcb_flush, xcb_free_gc,
        xcb_free_pixmap, xcb_gc_t_XCB_GC_FOREGROUND, xcb_gcontext_t, xcb_generate_id,
        xcb_generic_error_t, xcb_get_extension_data, xcb_get_geometry, xcb_get_geometry_reply,
        xcb_get_property_reply, xcb_get_property_reply_t,
        xcb_get_property_type_t_XCB_GET_PROPERTY_TYPE_ANY, xcb_get_property_unchecked,
        xcb_get_property_value, xcb_get_property_value_length, xcb_get_setup, xcb_grab_keyboard,
        xcb_grab_keyboard_cookie_t, xcb_grab_keyboard_reply, xcb_grab_mode_t_XCB_GRAB_MODE_ASYNC,
        xcb_grab_pointer, xcb_grab_pointer_cookie_t, xcb_grab_pointer_reply,
        xcb_grab_status_t_XCB_GRAB_STATUS_SUCCESS, xcb_intern_atom, xcb_intern_atom_reply,
        xcb_intern_atom_reply_t, xcb_key_press_event_t, xcb_map_window, xcb_pixmap_t,
        xcb_poly_fill_rectangle, xcb_prop_mode_t_XCB_PROP_MODE_REPLACE,
        xcb_query_extension_reply_t, xcb_randr_get_monitors,
        xcb_randr_get_monitors_monitors_iterator, xcb_randr_get_monitors_monitors_length,
        xcb_randr_get_monitors_reply, xcb_randr_get_monitors_reply_t, xcb_randr_id,
        xcb_randr_monitor_info_iterator_t, xcb_randr_monitor_info_next,
        xcb_randr_notify_mask_t_XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE,
        xcb_randr_notify_mask_t_XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE,
        xcb_randr_notify_mask_t_XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY,
        xcb_randr_notify_mask_t_XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE, xcb_randr_select_input,
        xcb_rectangle_t, xcb_screen_allowed_depths_iterator, xcb_screen_t,
        xcb_setup_roots_iterator, xcb_stack_mode_t_XCB_STACK_MODE_ABOVE, xcb_unmap_notify_event_t,
        xcb_visibility_notify_event_t, xcb_visibility_t_XCB_VISIBILITY_UNOBSCURED,
        xcb_visualtype_iterator_t, xcb_visualtype_next, xcb_visualtype_t, xcb_wait_for_event,
        xcb_window_class_t_XCB_WINDOW_CLASS_COPY_FROM_PARENT,
        xcb_window_class_t_XCB_WINDOW_CLASS_INPUT_OUTPUT, xcb_window_t, xcb_xkb_select_events,
        xcb_xrm_database_free, xcb_xrm_database_from_default, xcb_xrm_resource_get_string,
        xkb_compose_feed_result_XKB_COMPOSE_FEED_ACCEPTED, xkb_compose_state,
        xkb_compose_state_feed, xkb_compose_state_get_one_sym, xkb_compose_state_get_status,
        xkb_compose_state_get_utf8, xkb_compose_state_new, xkb_compose_state_reset,
        xkb_compose_state_unref, xkb_compose_status_XKB_COMPOSE_CANCELLED,
        xkb_compose_status_XKB_COMPOSE_COMPOSED, xkb_compose_table,
        xkb_compose_table_new_from_locale, xkb_compose_table_unref, xkb_context, xkb_context_new,
        xkb_keymap, xkb_keymap_layout_get_name, xkb_keymap_mod_get_name, xkb_keymap_num_layouts,
        xkb_keymap_num_mods, xkb_keymap_unref, xkb_keysym_to_utf8, xkb_mod_index_t, xkb_state,
        xkb_state_component_XKB_STATE_MODS_EFFECTIVE, xkb_state_key_get_one_sym,
        xkb_state_mod_index_is_active, xkb_state_unref, xkb_x11_get_core_keyboard_device_id,
        xkb_x11_keymap_new_from_device, xkb_x11_setup_xkb_extension, xkb_x11_state_new_from_device,
        XKB_KEY_BackSpace, XKB_KEY_Escape, XKB_KEY_KP_Enter, XKB_KEY_Return,
        XKB_KEY_XF86ScreenSaver, XCB_CONFIGURE_NOTIFY, XCB_COPY_FROM_PARENT, XCB_CURRENT_TIME,
        XCB_DESTROY_NOTIFY, XCB_KEY_PRESS, XCB_MAP_NOTIFY, XCB_NONE,
        XCB_RANDR_SCREEN_CHANGE_NOTIFY, XCB_UNMAP_NOTIFY, XCB_VISIBILITY_NOTIFY, XKB_MOD_NAME_CAPS,
        XKB_MOD_NAME_NUM, XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION,
    },
    clear_password, const_string_ptr, free, load_atomic, replace_option, store_atomic,
    string_from_ptr,
    types::{Modifier, Rect, ScreenQuery, UnlockState},
    utils::get_locale,
    wm::{
        statics::{AUTH_STATE, PASSWORD, UNLOCK_STATE},
        Broker, Call,
    },
    x11::{consts, statics},
};
use anyhow::{anyhow, Result};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tracing::{debug, trace};

fn skip_without_validation() -> bool {
    if PASSWORD.lock().unwrap().len() != 0 {
        return false;
    }

    return load_atomic!(statics::SKIP_REPEATED_EMPTY_PASSWORD)
        || load_atomic!(statics::IGNORE_EMPTY_PASSWORD);
}

pub struct Connection {
    pub(crate) xcb: *mut xcb_connection_t,
    pub(crate) xkb: Option<*mut xkb_context>,
    pub(crate) xkb_keymap: Option<*mut xkb_keymap>,
    pub(crate) xkb_state: Option<*mut xkb_state>,
    pub(crate) xkb_compose_table: Option<*mut xkb_compose_table>,
    pub(crate) xkb_compose_state: Option<*mut xkb_compose_state>,
    pub(crate) xkb_base_event: u8,
    pub(crate) xkb_base_error: u8,
    pub(crate) dpi: i32,
    pub(crate) randr_base: u8,
    pub(crate) screen: Option<*mut xcb_screen_t>,
    pub(crate) vistype: Option<*mut xcb_visualtype_t>,
    pub(crate) net_active_window: Option<xcb_atom_t>,
    pub(crate) net_bypass_compositor: Option<xcb_atom_t>,
    pub(crate) modifier_set: Option<Arc<Mutex<HashSet<Modifier>>>>,
    pub(crate) layout_set: Option<Arc<Mutex<HashSet<String>>>>,
    pub(crate) focused_window: Option<xcb_window_t>,
    pub(crate) my_window: Option<xcb_window_t>,
    pub(crate) receiver: Arc<Mutex<Option<std::sync::mpsc::Receiver<Call>>>>,
    pub(crate) pam: Option<std::sync::mpsc::Sender<()>>,
    pub(crate) pam_return: Arc<Mutex<Option<std::sync::mpsc::Receiver<()>>>>,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            xcb: std::ptr::null_mut(),
            xkb: None,
            xkb_keymap: None,
            xkb_state: None,
            xkb_compose_table: None,
            xkb_compose_state: None,
            xkb_base_event: 0,
            xkb_base_error: 0,
            dpi: 0,
            randr_base: 0,
            screen: None,
            vistype: None,
            net_active_window: None,
            net_bypass_compositor: None,
            modifier_set: None,
            layout_set: None,
            focused_window: None,
            my_window: None,
            pam: None,
            pam_return: Arc::new(Mutex::new(None)),
            receiver: Arc::new(Mutex::new(None)),
        }
    }
}

impl Connection {
    pub fn init(screen_number: i32) -> Result<Self> {
        let mut obj = Self::connect(screen_number)?;
        obj.clear_xkb_extension()?;
        obj.select_xkb_events()?;
        obj.load_keymap()?;

        debug!("locale: {}", get_locale());
        obj.load_compose_table(get_locale())?;

        let dpi = obj.get_dpi()?;
        debug!("dpi: {}", dpi);

        obj.randr_init()?;
        let s: [u32; 1] = [xcb_event_mask_t_XCB_EVENT_MASK_STRUCTURE_NOTIFY];
        let root = unsafe { (*obj.get_root_screen()).root };
        obj.change_window_attributes(root, xcb_cw_t_XCB_CW_EVENT_MASK, &s);
        Ok(obj)
    }

    fn connect(mut screen_number: i32) -> Result<Self> {
        let conn = unsafe { xcb_connect(std::ptr::null(), &mut screen_number) };
        if !conn.is_null() {
            Ok(Connection {
                xcb: conn,
                ..Default::default()
            })
        } else {
            return Err(anyhow!("Could not initiate XCB connection"));
        }
    }

    pub fn screen(&self) -> Option<*mut xcb_screen_t> {
        self.screen.clone()
    }

    fn clear_xkb_extension(&mut self) -> Result<()> {
        let result = unsafe {
            xkb_x11_setup_xkb_extension(
                self.xcb,
                XKB_X11_MIN_MAJOR_XKB_VERSION.try_into()?,
                XKB_X11_MIN_MINOR_XKB_VERSION.try_into()?,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut self.xkb_base_event,
                &mut self.xkb_base_error,
            )
        };

        if result != 1 {
            Err(anyhow!("Could not setup XKB extension"))
        } else {
            Ok(())
        }
    }

    fn select_xkb_events(&mut self) -> Result<()> {
        unsafe {
            xcb_xkb_select_events(
                self.xcb,
                xkb_x11_get_core_keyboard_device_id(self.xcb).try_into()?,
                consts::REQUIRED_EVENTS.try_into()?,
                0,
                consts::REQUIRED_EVENTS.try_into()?,
                consts::REQUIRED_MAP_PARTS.try_into()?,
                consts::REQUIRED_MAP_PARTS.try_into()?,
                std::ptr::null(),
            )
        };

        Ok(())
    }

    pub fn load_keymap(&mut self) -> Result<()> {
        if self.xkb.is_none() {
            let context = unsafe { xkb_context_new(0) };

            if context.is_null() {
                return Err(anyhow!("Could not create XKB context"));
            }

            self.xkb = Some(context);
        }

        if let Some(keymap) = self.xkb_keymap {
            unsafe { xkb_keymap_unref(keymap) }
        }

        let device_id = unsafe { xkb_x11_get_core_keyboard_device_id(self.xcb) };
        let keymap =
            unsafe { xkb_x11_keymap_new_from_device(self.xkb.unwrap(), self.xcb, device_id, 0) };

        if keymap.is_null() {
            return Err(anyhow!("xkb_x11_keymap_new_from_device failed"));
        }

        self.xkb_keymap = Some(keymap);

        let state =
            unsafe { xkb_x11_state_new_from_device(self.xkb_keymap.unwrap(), self.xcb, device_id) };

        if state.is_null() {
            return Err(anyhow!("xkb_x11_state_new_from_device failed"));
        }

        if let Some(xkb_state) = self.xkb_state {
            unsafe { xkb_state_unref(xkb_state) };
        }

        self.xkb_state = Some(state);

        Ok(())
    }

    fn load_compose_table(&mut self, locale: String) -> Result<()> {
        if let Some(compose_table) = self.xkb_compose_table {
            unsafe { xkb_compose_table_unref(compose_table) };
        }

        let compose_table = unsafe {
            xkb_compose_table_new_from_locale(self.xkb.unwrap(), const_string_ptr!(locale), 0)
        };

        if compose_table.is_null() {
            return Err(anyhow!("xkb_compose_table_new_from_locale failed"));
        }

        self.xkb_compose_table = Some(compose_table);

        let compose_state = unsafe { xkb_compose_state_new(compose_table, 0) };

        if compose_state.is_null() {
            return Err(anyhow!("xkb_compose_state_new failed"));
        }

        if let Some(state) = self.xkb_compose_state {
            unsafe { xkb_compose_state_unref(state) };
        }

        self.xkb_compose_state = Some(compose_state);

        Ok(())
    }

    pub fn get_root_screen(&mut self) -> *mut xcb_screen_t {
        if let Some(screen) = self.screen {
            return screen;
        }

        let screen = unsafe { xcb_setup_roots_iterator(xcb_get_setup(self.xcb)) }.data;
        self.screen = Some(screen);
        screen
    }

    fn get_dpi(&mut self) -> Result<i32> {
        let database = unsafe { xcb_xrm_database_from_default(self.xcb) };
        if database.is_null() {
            return Ok(self.default_dpi());
        }

        let mut resource: *mut i8 = std::ptr::null_mut();

        unsafe {
            xcb_xrm_resource_get_string(
                database,
                const_string_ptr!(consts::DPI_RESOURCE),
                std::ptr::null(),
                &mut resource,
            )
        };

        if resource.is_null() {
            unsafe { xcb_xrm_database_free(database) };
            return Ok(self.default_dpi());
        }

        let res = string_from_ptr!(resource)?;

        let dpi: i32 = match res.parse() {
            Ok(x) => x,
            Err(_) => self.default_dpi(),
        };

        unsafe {
            free!(resource);
            xcb_xrm_database_free(database);
        };

        self.dpi = dpi;
        Ok(self.default_dpi())
    }

    fn default_dpi(&mut self) -> i32 {
        if self.dpi == 0 {
            let screen = self.get_root_screen();
            self.dpi = unsafe {
                let pixels: f32 = (*screen).height_in_pixels.into();
                let mm: f32 = (*screen).height_in_millimeters.into();
                (pixels * 25.4 / mm).trunc() as i32
            }
        }

        self.dpi
    }

    fn randr_init(&mut self) -> Result<()> {
        let extreply: *const xcb_query_extension_reply_t = unsafe {
            let randr_id: &mut xcb_extension_t = &mut xcb_randr_id;
            xcb_get_extension_data(self.xcb, randr_id)
        };

        let screen = self.get_root_screen();
        let root = unsafe { (*screen).root };

        if extreply.is_null() || unsafe { (*extreply).present == 0 } {
            return Err(anyhow!("randr not present"));
        }

        self.randr_base = unsafe { (*extreply).first_event };

        unsafe {
            xcb_randr_select_input(
                self.xcb,
                root,
                (xcb_randr_notify_mask_t_XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE
                    | xcb_randr_notify_mask_t_XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE
                    | xcb_randr_notify_mask_t_XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE
                    | xcb_randr_notify_mask_t_XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY)
                    .try_into()?,
            );
        };

        self.flush();

        Ok(())
    }

    pub fn randr_query(&mut self) -> Result<ScreenQuery> {
        let screen = self.get_root_screen();
        let root = unsafe { (*screen).root };
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();

        let monitors: *const xcb_randr_get_monitors_reply_t = unsafe {
            xcb_randr_get_monitors_reply(
                self.xcb,
                xcb_randr_get_monitors(self.xcb, root, 1),
                &mut err,
            )
        };

        if !err.is_null() {
            let code = unsafe { (*err).error_code };
            free!(err);
            return Err(anyhow!("xrandr error code: {}", code));
        }

        let screens: usize =
            unsafe { xcb_randr_get_monitors_monitors_length(monitors) }.try_into()?;

        let mut resolutions: Vec<Rect> = Vec::new();

        let mut iter: xcb_randr_monitor_info_iterator_t =
            unsafe { xcb_randr_get_monitors_monitors_iterator(monitors) };

        while iter.rem > 0 {
            let data = unsafe { *iter.data };

            resolutions.push(Rect {
                x: data.x,
                y: data.y,
                width: data.width,
                height: data.height,
            });

            unsafe { xcb_randr_monitor_info_next(&mut iter) };
        }

        free!(monitors);

        Ok(ScreenQuery {
            resolutions,
            screens,
        })
    }

    pub fn change_window_attributes(&mut self, win: xcb_window_t, cw: xcb_cw_t, controls: &[u32]) {
        unsafe { xcb_change_window_attributes(self.xcb, win, cw, controls.as_ptr().cast()) };
    }

    pub fn create_bg_pixmap(&mut self, resolution: Rect, color: u32) -> xcb_pixmap_t {
        let bg_pixmap: xcb_pixmap_t = unsafe { xcb_generate_id(self.xcb) };
        let screen = self.get_root_screen();
        unsafe {
            xcb_create_pixmap(
                self.xcb,
                (*screen).root_depth,
                bg_pixmap,
                (*screen).root,
                resolution.width,
                resolution.height,
            )
        };

        let gc: xcb_gcontext_t = unsafe { xcb_generate_id(self.xcb) };
        let values: [u32; 1] = [color];
        unsafe {
            xcb_create_gc(
                self.xcb,
                gc,
                bg_pixmap,
                xcb_gc_t_XCB_GC_FOREGROUND,
                values.as_ptr().cast(),
            )
        };

        let rect = xcb_rectangle_t {
            width: resolution.width,
            height: resolution.height,
            x: 0,
            y: 0,
        };

        unsafe {
            xcb_poly_fill_rectangle(self.xcb, bg_pixmap, gc, 1, &rect);
            xcb_free_gc(self.xcb, gc);
        };

        bg_pixmap
    }

    fn get_root_visual_type(&mut self) -> Option<*mut xcb_visualtype_t> {
        if self.vistype.is_some() {
            return self.vistype;
        }

        let screen = self.get_root_screen();
        let mut depth_iter: xcb_depth_iterator_t =
            unsafe { xcb_screen_allowed_depths_iterator(screen) };

        while depth_iter.rem != 0 {
            let mut visual_iter: xcb_visualtype_iterator_t =
                unsafe { xcb_depth_visuals_iterator(depth_iter.data) };

            while visual_iter.rem != 0 {
                if unsafe { (*screen).root_visual != (*visual_iter.data).visual_id } {
                    unsafe { xcb_visualtype_next(&mut visual_iter) };
                    continue;
                }

                self.vistype = Some(visual_iter.data);
                return Some(visual_iter.data);
            }

            unsafe { xcb_depth_next(&mut depth_iter) };
        }

        None
    }

    pub fn draw_image(&mut self, bg_pixmap: xcb_pixmap_t, resolution: Rect) -> Result<()> {
        let vistype = self.get_root_visual_type();
        if vistype.is_none() {
            return Err(anyhow!("Could not get visual type"));
        }

        let output: *mut cairo_surface_t = unsafe {
            cairo_xcb_surface_create(
                self.xcb,
                bg_pixmap,
                vistype.unwrap(),
                resolution.width.into(),
                resolution.height.into(),
            )
        };

        let ctx: *mut cairo_t = unsafe { cairo_create(output) };
        unsafe {
            cairo_set_source_rgb(ctx, 0.0, 0.0, 0.0);
            cairo_rectangle(
                ctx,
                0.0,
                0.0,
                resolution.width.into(),
                resolution.height.into(),
            );
            cairo_fill(ctx);
            cairo_surface_destroy(output);
            cairo_destroy(ctx);
        };

        Ok(())
    }

    pub fn find_focused_window(&mut self) -> Result<Option<xcb_window_t>> {
        let mut result = None;

        if self.net_active_window.is_none() {
            let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
            let atom_reply: *mut xcb_intern_atom_reply_t = unsafe {
                xcb_intern_atom_reply(
                    self.xcb,
                    xcb_intern_atom(
                        self.xcb,
                        0,
                        consts::NET_ACTIVE_WINDOW.len().try_into()?,
                        const_string_ptr!(consts::NET_ACTIVE_WINDOW),
                    ),
                    &mut err,
                )
            };

            if atom_reply.is_null() {
                let code = unsafe { (*err).error_code };
                free!(err);
                return Err(anyhow!("X11 Error {}", code));
            }

            self.net_active_window = Some(unsafe { (*atom_reply).atom });
            free!(atom_reply);
        }

        let prop_reply: *mut xcb_get_property_reply_t = unsafe {
            xcb_get_property_reply(
                self.xcb,
                xcb_get_property_unchecked(
                    self.xcb,
                    0,
                    (*self.get_root_screen()).root,
                    self.net_active_window.unwrap(),
                    xcb_get_property_type_t_XCB_GET_PROPERTY_TYPE_ANY,
                    0,
                    1,
                ),
                std::ptr::null_mut(),
            )
        };

        if !prop_reply.is_null() {
            if unsafe { xcb_get_property_value_length(prop_reply) } != 0
                && unsafe { (*prop_reply).type_ } == xcb_atom_enum_t_XCB_ATOM_WINDOW
            {
                let value: *mut xcb_window_t = unsafe { xcb_get_property_value(prop_reply).cast() };
                result = Some(unsafe { *value });
            }

            free!(prop_reply);
        }

        Ok(result)
    }

    pub fn free_pixmap(&self, bg_pixmap: xcb_pixmap_t) {
        unsafe { xcb_free_pixmap(self.xcb, bg_pixmap) };
    }

    pub fn open_fullscreen_window(&mut self, name: &str) -> Result<xcb_window_t> {
        let mut mask: u32 = xcb_cw_t_XCB_CW_BACK_PIXEL;
        let mut values: [u32; 3] = [0, 0, 0];
        let win = unsafe { xcb_generate_id(self.xcb) };

        mask |= xcb_cw_t_XCB_CW_OVERRIDE_REDIRECT;
        values[1] = 1;

        mask |= xcb_cw_t_XCB_CW_EVENT_MASK;
        values[2] = xcb_event_mask_t_XCB_EVENT_MASK_EXPOSURE
            | xcb_event_mask_t_XCB_EVENT_MASK_KEY_PRESS
            | xcb_event_mask_t_XCB_EVENT_MASK_KEY_RELEASE
            | xcb_event_mask_t_XCB_EVENT_MASK_VISIBILITY_CHANGE
            | xcb_event_mask_t_XCB_EVENT_MASK_STRUCTURE_NOTIFY;

        let screen = self.get_root_screen();

        unsafe {
            xcb_create_window(
                self.xcb,
                XCB_COPY_FROM_PARENT.try_into()?,
                win,
                (*screen).root,
                0,
                0,
                (*screen).width_in_pixels,
                (*screen).height_in_pixels,
                0,
                xcb_window_class_t_XCB_WINDOW_CLASS_INPUT_OUTPUT.try_into()?,
                xcb_window_class_t_XCB_WINDOW_CLASS_COPY_FROM_PARENT,
                mask,
                values.as_ptr().cast(),
            );

            xcb_change_property(
                self.xcb,
                xcb_prop_mode_t_XCB_PROP_MODE_REPLACE.try_into()?,
                win,
                xcb_atom_enum_t_XCB_ATOM_WM_NAME,
                xcb_atom_enum_t_XCB_ATOM_STRING,
                8,
                name.len().try_into()?,
                const_string_ptr!(name),
            );

            xcb_change_property(
                self.xcb,
                xcb_prop_mode_t_XCB_PROP_MODE_REPLACE.try_into()?,
                win,
                xcb_atom_enum_t_XCB_ATOM_WM_CLASS,
                xcb_atom_enum_t_XCB_ATOM_STRING,
                8,
                (2 as usize * (name.len() + 1)).try_into()?,
                const_string_ptr!(format!("{}\0{}\0", name, name)),
            )
        };

        if self.net_bypass_compositor.is_none() {
            let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
            let atom_reply: *mut xcb_intern_atom_reply_t = unsafe {
                xcb_intern_atom_reply(
                    self.xcb,
                    xcb_intern_atom(
                        self.xcb,
                        0,
                        consts::NET_WM_BYPASS_COMPOSITOR.len().try_into()?,
                        const_string_ptr!(consts::NET_WM_BYPASS_COMPOSITOR),
                    ),
                    &mut err,
                )
            };

            if atom_reply.is_null() {
                let code = unsafe { (*err).error_code };
                free!(err);
                return Err(anyhow!("X11 Error {}", code));
            }

            self.net_bypass_compositor = Some(unsafe { (*atom_reply).atom });
            free!(atom_reply);
        }

        unsafe {
            let bc: u32 = 1;
            let bypass_compositor = std::ptr::addr_of!(bc);
            xcb_change_property(
                self.xcb,
                xcb_prop_mode_t_XCB_PROP_MODE_REPLACE.try_into()?,
                win,
                self.net_bypass_compositor.unwrap(),
                xcb_atom_enum_t_XCB_ATOM_CARDINAL,
                32,
                1,
                bypass_compositor.cast(),
            );

            xcb_map_window(self.xcb, win);

            values[0] = xcb_stack_mode_t_XCB_STACK_MODE_ABOVE;
            xcb_configure_window(
                self.xcb,
                win,
                xcb_config_window_t_XCB_CONFIG_WINDOW_STACK_MODE.try_into()?,
                values.as_ptr().cast(),
            );

            xcb_aux_sync(self.xcb);
        };

        self.my_window = Some(win);
        Ok(win)
    }

    pub fn create_cursor(&mut self) -> xcb_cursor_t {
        let screen = self.get_root_screen();
        let bitmap = unsafe {
            xcb_create_pixmap_from_bitmap_data(
                self.xcb,
                self.my_window.unwrap(),
                consts::CURS_INVISIBLE_BITS.clone().as_mut_ptr().cast(),
                consts::CURS_INVISIBLE_WIDTH,
                consts::CURS_INVISIBLE_HEIGHT,
                1,
                (*screen).white_pixel,
                (*screen).black_pixel,
                std::ptr::null_mut(),
            )
        };

        let mask = unsafe {
            xcb_create_pixmap_from_bitmap_data(
                self.xcb,
                self.my_window.unwrap(),
                consts::CURS_INVISIBLE_BITS.clone().as_mut_ptr().cast(),
                consts::CURS_INVISIBLE_WIDTH,
                consts::CURS_INVISIBLE_HEIGHT,
                1,
                (*screen).white_pixel,
                (*screen).black_pixel,
                std::ptr::null_mut(),
            )
        };

        let cursor = unsafe { xcb_generate_id(self.xcb) };

        unsafe {
            xcb_create_cursor(
                self.xcb, cursor, bitmap, mask, 65535, 65535, 65535, 0, 0, 0, 0, 0,
            );

            xcb_free_pixmap(self.xcb, bitmap);
            xcb_free_pixmap(self.xcb, mask);
        }

        return cursor;
    }

    pub fn grab_pointer_and_keyboard(
        &mut self,
        mut tries: u32,
    ) -> Result<(xcb_grab_pointer_cookie_t, xcb_grab_keyboard_cookie_t)> {
        let cursor = self.create_cursor();
        let screen = self.get_root_screen();
        let mut pcookie: Option<xcb_grab_pointer_cookie_t> = None;
        let mut kcookie: Option<xcb_grab_keyboard_cookie_t> = None;
        let mut redrawn = std::time::Instant::now();

        while tries > 0 {
            let now = std::time::Instant::now();

            pcookie = Some(unsafe {
                xcb_grab_pointer(
                    self.xcb,
                    0,
                    (*screen).root,
                    XCB_NONE.try_into()?,
                    xcb_grab_mode_t_XCB_GRAB_MODE_ASYNC.try_into()?,
                    xcb_grab_mode_t_XCB_GRAB_MODE_ASYNC.try_into()?,
                    XCB_NONE,
                    cursor,
                    XCB_CURRENT_TIME,
                )
            });

            let preply =
                unsafe { xcb_grab_pointer_reply(self.xcb, pcookie.unwrap(), std::ptr::null_mut()) };
            let status = if !preply.is_null() {
                let result = unsafe { (*preply).status };
                free!(preply);
                Some(result)
            } else {
                None
            };

            if let Some(status) = status {
                if status == xcb_grab_status_t_XCB_GRAB_STATUS_SUCCESS.try_into()? {
                    break;
                }
            }

            std::thread::sleep(consts::GRAB_RETRY_DURATION);

            if now - redrawn > consts::GRAB_REDRAW_TIMEOUT {
                self.redraw_screen()?;
                redrawn = now;
            }

            tries -= 1;
        }

        while tries > 0 {
            let now = std::time::Instant::now();

            kcookie = Some(unsafe {
                xcb_grab_keyboard(
                    self.xcb,
                    1,
                    (*screen).root,
                    XCB_CURRENT_TIME,
                    xcb_grab_mode_t_XCB_GRAB_MODE_ASYNC.try_into()?,
                    xcb_grab_mode_t_XCB_GRAB_MODE_ASYNC.try_into()?,
                )
            });

            let kreply = unsafe {
                xcb_grab_keyboard_reply(self.xcb, kcookie.unwrap(), std::ptr::null_mut())
            };

            let status = if !kreply.is_null() {
                let status = unsafe { (*kreply).status };
                free!(kreply);
                Some(status)
            } else {
                None
            };

            if let Some(status) = status {
                if status == xcb_grab_status_t_XCB_GRAB_STATUS_SUCCESS.try_into()? {
                    break;
                }
            }

            std::thread::sleep(consts::GRAB_RETRY_DURATION);

            if now - redrawn > consts::GRAB_REDRAW_TIMEOUT {
                self.redraw_screen()?;
                redrawn = now;
            }

            tries -= 1;
        }

        if pcookie.is_none() || kcookie.is_none() {
            return Err(anyhow!("Failed to grab both keyboard and mouse"));
        }

        Ok((pcookie.unwrap(), kcookie.unwrap()))
    }

    pub fn redraw_screen(&mut self) -> Result<()> {
        trace!(
            "redraw_screen(unlock_state = {:?}, auth_state = {:?})",
            UNLOCK_STATE.lock().unwrap().clone().unwrap(),
            AUTH_STATE.lock().unwrap().clone().unwrap(),
        );

        if self.modifier_set.is_some() {
            self.modifier_set = None;
        }

        let num_mods: xkb_mod_index_t = unsafe { xkb_keymap_num_mods(self.xkb_keymap.unwrap()) };

        for idx in 0..num_mods {
            if unsafe {
                xkb_state_mod_index_is_active(
                    self.xkb_state.unwrap(),
                    idx,
                    xkb_state_component_XKB_STATE_MODS_EFFECTIVE,
                )
            } == 0
            {
                continue;
            }

            let mod_name = unsafe { xkb_keymap_mod_get_name(self.xkb_keymap.unwrap(), idx) };
            if mod_name.is_null() {
                continue;
            }

            let caps = std::ffi::CString::from_vec_with_nul(XKB_MOD_NAME_CAPS.to_vec())?
                .to_str()?
                .to_string();
            let num = std::ffi::CString::from_vec_with_nul(XKB_MOD_NAME_NUM.to_vec())?
                .to_str()?
                .to_string();

            let modifier = match string_from_ptr!(mod_name) {
                Ok(s) => {
                    let s = s.to_string();
                    if s == caps {
                        Some(Modifier::CapsLock)
                    } else if s == num {
                        Some(Modifier::NumLock)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            };

            free!(mod_name);

            if let Some(modifier) = modifier {
                if self.modifier_set.is_none() {
                    self.modifier_set = Some(Arc::new(Mutex::new(HashSet::default())));
                }

                if let Some(modifier_set) = &self.modifier_set {
                    let mut modifier_set = modifier_set.lock().unwrap();
                    modifier_set.insert(modifier);
                }
            }
        }

        if self.layout_set.is_some() {
            self.layout_set = None;
        }

        let num_layouts: xkb_mod_index_t =
            unsafe { xkb_keymap_num_layouts(self.xkb_keymap.unwrap()) };

        for idx in 0..num_layouts {
            if unsafe {
                xkb_state_mod_index_is_active(
                    self.xkb_state.unwrap(),
                    idx,
                    xkb_state_component_XKB_STATE_MODS_EFFECTIVE,
                )
            } != 0
            {
                let name = unsafe { xkb_keymap_layout_get_name(self.xkb_keymap.unwrap(), idx) };
                let n = string_from_ptr!(name)?.to_string();

                if self.layout_set.is_none() {
                    self.layout_set = Some(Arc::new(Mutex::new(HashSet::default())));
                }

                if let Some(layout_set) = &self.layout_set {
                    let mut layout_set = layout_set.lock().unwrap();
                    layout_set.insert(n);
                }

                free!(name);
            }
        }

        let screen = self.get_root_screen();

        let width = unsafe { (*screen).width_in_pixels };
        let height = unsafe { (*screen).height_in_pixels };
        let rect = Rect {
            width,
            height,
            x: 0,
            y: 0,
        };

        let bg_pixmap = self.create_bg_pixmap(rect.clone(), 0);

        self.draw_image(bg_pixmap, rect.clone())?;
        self.change_window_attributes(
            self.my_window.unwrap(),
            xcb_cw_t_XCB_CW_BACK_PIXMAP,
            &[bg_pixmap],
        );

        unsafe {
            xcb_clear_area(self.xcb, 0, self.my_window.unwrap(), 0, 0, width, height);
            xcb_free_pixmap(self.xcb, bg_pixmap);
        };

        self.flush();

        Ok(())
    }

    fn flush(&self) {
        unsafe { xcb_flush(self.xcb) };
    }

    pub fn close_sleep_lock_fd(&self) -> Result<()> {
        if let Ok(sleep_lock_fd) = std::env::var("XSS_SLEEP_LOCK_FD") {
            let fd = sleep_lock_fd.parse()?;
            unsafe { libc::close(fd) };
        }

        Ok(())
    }

    pub fn raise_loop(&mut self) -> Result<()> {
        self.change_window_attributes(
            self.my_window.unwrap(),
            xcb_cw_t_XCB_CW_EVENT_MASK,
            &[
                xcb_event_mask_t_XCB_EVENT_MASK_VISIBILITY_CHANGE,
                xcb_event_mask_t_XCB_EVENT_MASK_STRUCTURE_NOTIFY,
            ],
        );

        self.flush();

        loop {
            let event = unsafe { xcb_wait_for_event(self.xcb) };
            if event.is_null() {
                break;
            }

            if unsafe { (*event).response_type } == 0 {
                let error: *mut xcb_generic_error_t = event.cast();
                debug!(
                    "X11 error received: sequence {}, error_code {}",
                    unsafe { (*error).sequence },
                    unsafe { (*error).error_code }
                );
                free!(event);
                continue;
            }

            let response_type = unsafe { (*event).response_type } & 0x7f;
            trace!("Read event of type {}", response_type);

            match response_type.into() {
                XCB_KEY_PRESS => self.respond_to_key_press(event.cast()),
                XCB_VISIBILITY_NOTIFY => {
                    let event: *mut xcb_visibility_notify_event_t = event.cast();

                    if unsafe { (*event).state }
                        != xcb_visibility_t_XCB_VISIBILITY_UNOBSCURED
                            .try_into()
                            .unwrap()
                    {
                        let values: [u32; 1] = [xcb_stack_mode_t_XCB_STACK_MODE_ABOVE];
                        unsafe {
                            xcb_configure_window(
                                self.xcb,
                                (*event).window,
                                xcb_config_window_t_XCB_CONFIG_WINDOW_STACK_MODE
                                    .try_into()
                                    .unwrap(),
                                values.as_ptr().cast(),
                            )
                        };
                        self.flush();
                    }
                }
                XCB_MAP_NOTIFY => {
                    self.close_sleep_lock_fd().unwrap();
                }
                XCB_UNMAP_NOTIFY => {
                    let event: *mut xcb_unmap_notify_event_t = event.cast();
                    let event_window = unsafe { (*event).window };
                    debug!("unmap notify for {}", event_window);

                    if event_window == self.my_window.unwrap() {
                        std::process::exit(0);
                    }
                }
                XCB_DESTROY_NOTIFY => {
                    let event: *mut xcb_destroy_notify_event_t = event.cast();
                    let event_window = unsafe { (*event).window };
                    debug!("destroy notify for {}", event_window);
                    if event_window == self.my_window.unwrap() {
                        std::process::exit(0);
                    }
                }
                XCB_CONFIGURE_NOTIFY => {
                    self.handle_screen_resize().expect("Couldn't resize screen")
                }
                _ => {
                    if response_type == self.xkb_base_event.into() {
                        // FIXME: process_xkb_event
                        self.redraw_screen().expect("Couldn't redraw screen");
                    }

                    if response_type == self.randr_base + XCB_RANDR_SCREEN_CHANGE_NOTIFY as u8 {
                        self.randr_query().expect("Couldn't query randr");
                        self.handle_screen_resize().expect("Couldn't resize screen");
                    }
                }
            }

            free!(event);
        }

        Ok(())
    }

    pub fn respond_to_key_press(&mut self, event: *mut xcb_key_press_event_t) {
        let mut ksym =
            unsafe { xkb_state_key_get_one_sym(self.xkb_state.unwrap(), (*event).detail.into()) };

        let mut composed = false;
        let mut n = 0;
        let mut buffer: [u8; 128] = [0_u8; 128];

        if unsafe { xkb_compose_state_feed(self.xkb_compose_state.unwrap(), ksym) }
            == xkb_compose_feed_result_XKB_COMPOSE_FEED_ACCEPTED
        {
            match unsafe { xkb_compose_state_get_status(self.xkb_compose_state.unwrap()) } {
                xkb_compose_status_XKB_COMPOSE_COMPOSED => {
                    n = unsafe {
                        xkb_compose_state_get_utf8(
                            self.xkb_compose_state.unwrap(),
                            buffer.as_mut_ptr().cast(),
                            128,
                        )
                    };

                    ksym =
                        unsafe { xkb_compose_state_get_one_sym(self.xkb_compose_state.unwrap()) };

                    composed = true;
                }
                xkb_compose_status_XKB_COMPOSE_CANCELLED => {
                    unsafe { xkb_compose_state_reset(self.xkb_compose_state.unwrap()) };
                }
                _ => {}
            }
        }

        if !composed {
            n = unsafe { xkb_keysym_to_utf8(ksym, buffer.as_mut_ptr().cast(), 128) };
            composed = true;
        }

        trace!("Key pressed: {}", ksym);

        match ksym {
            XKB_KEY_Return | XKB_KEY_KP_Enter | XKB_KEY_XF86ScreenSaver => {
                if skip_without_validation() {
                    clear_password!();
                    return;
                }

                replace_option!(UNLOCK_STATE, UnlockState::KeyPressed);

                self.redraw_screen().expect("Could not redraw screen");

                replace_option!(UNLOCK_STATE, UnlockState::Started);

                self.pam()
                    .unwrap()
                    .send(())
                    .expect("Could not trigger pam check");
                let pam_return = self.pam_return();
                let mut lock = pam_return.lock().unwrap();
                let ch = lock.take().unwrap();
                ch.recv().unwrap();
                lock.replace(ch);

                self.redraw_screen().expect("Could not redraw screen");
                return;
            }
            _ => {
                store_atomic!(statics::SKIP_REPEATED_EMPTY_PASSWORD, false);
                if load_atomic!(statics::RETRY_VERIFICATION) {
                    store_atomic!(statics::RETRY_VERIFICATION, false);
                    clear_password!();
                }
            }
        }

        match ksym {
            XKB_KEY_Escape => {
                debug!("Escape pressed");
                clear_password!();
                return;
            }
            XKB_KEY_BackSpace => {
                let mut lock = PASSWORD.lock().unwrap();
                if lock.is_empty() {
                    replace_option!(UNLOCK_STATE, UnlockState::NothingToDelete);
                }

                // FIXME: start login timer
                lock.pop();

                replace_option!(UNLOCK_STATE, UnlockState::BackspaceActive);

                self.redraw_screen().expect("Could not redraw screen");
                replace_option!(UNLOCK_STATE, UnlockState::KeyPressed);
                composed = false; // lets it fall through to the trace statement
            }
            _ => {}
        }

        if composed {
            PASSWORD.lock().unwrap().push_str(
                String::from_utf8(buffer[0..if n > 0 { (n - 1) as usize } else { 0 }].to_vec())
                    .unwrap()
                    .as_str(),
            );

            replace_option!(UNLOCK_STATE, UnlockState::KeyActive);
            self.redraw_screen().expect("Couldn't redraw screen");
            replace_option!(UNLOCK_STATE, UnlockState::KeyPressed);
        }

        trace!("PASSWORD is now '{}'", PASSWORD.lock().unwrap());

        // FIXME: start password clear timer
    }

    fn handle_screen_resize(&mut self) -> Result<()> {
        let screen = self.get_root_screen();
        let geomc = unsafe { xcb_get_geometry(self.xcb, (*screen).root) };
        let geom = unsafe { xcb_get_geometry_reply(self.xcb, geomc, std::ptr::null_mut()) };
        if geom.is_null() {
            return Ok(());
        }

        let mask = xcb_config_window_t_XCB_CONFIG_WINDOW_WIDTH
            | xcb_config_window_t_XCB_CONFIG_WINDOW_HEIGHT;
        let resolution: [u16; 2] = unsafe { [(*geom).width, (*geom).height] };

        unsafe {
            xcb_configure_window(
                self.xcb,
                self.my_window.unwrap(),
                mask.try_into().unwrap(),
                resolution.as_ptr().cast(),
            )
        };

        self.flush();

        self.randr_query()?;
        self.redraw_screen()?;
        Ok(())
    }
}

impl crate::screen::Screen for Connection {
    fn width(&mut self) -> u32 {
        let screen = self.screen().or(Some(self.get_root_screen()));
        unsafe { (*screen.unwrap()).width_in_pixels.into() }
    }

    fn height(&mut self) -> u32 {
        let screen = self.screen().or(Some(self.get_root_screen()));
        unsafe { (*screen.unwrap()).height_in_pixels.into() }
    }
}

impl Broker for Connection {
    fn set_pam_return(&mut self, pam_return: Arc<Mutex<Option<std::sync::mpsc::Receiver<()>>>>) {
        self.pam_return = pam_return
    }

    fn pam_return(&self) -> Arc<Mutex<Option<std::sync::mpsc::Receiver<()>>>> {
        self.pam_return.clone()
    }

    fn set_pam(&mut self, pam: std::sync::mpsc::Sender<()>) {
        self.pam = Some(pam);
    }

    fn pam(&self) -> Option<std::sync::mpsc::Sender<()>> {
        self.pam.clone()
    }

    fn set_receiver(&mut self, receiver: Arc<Mutex<Option<std::sync::mpsc::Receiver<Call>>>>) {
        self.receiver = receiver;
    }

    fn receiver(&self) -> Arc<Mutex<Option<std::sync::mpsc::Receiver<Call>>>> {
        self.receiver.clone()
    }

    fn listen(&mut self) -> Result<()> {
        while self.receiver().lock().unwrap().is_some() {
            let receiver = self.receiver();
            let mut lock = receiver.lock().unwrap();
            let receiver = lock.take().unwrap();

            if let Ok(msg) = receiver.recv_timeout(std::time::Duration::new(0, 100000)) {
                trace!("Call to X11: {:?}", msg);

                match msg {
                    Call::SelectFocusedWindow => {
                        self.focused_window = self.find_focused_window()?;
                    }
                    Call::OpenFullscreenWindow(program_name) => {
                        self.open_fullscreen_window(&program_name)?;
                    }
                    Call::GrabPointerAndKeyboard => {
                        if let Err(_) = self.grab_pointer_and_keyboard(1000) {
                            if let Err(_) = self.grab_pointer_and_keyboard(9000) {
                                self.redraw_screen()?;
                                std::thread::sleep(std::time::Duration::new(1, 0));
                                return Err(anyhow!("Could not grab pointer or keyboard"));
                            }
                        }
                    }
                    Call::UnlockSleep => {
                        self.close_sleep_lock_fd()?;
                    }
                    Call::RaiseWindow => {
                        self.raise_loop()?;
                    }
                    Call::RedrawScreen => {
                        self.redraw_screen()?;
                    }
                    Call::FlushCommands => {
                        self.flush();
                    }
                }
            }

            lock.replace(receiver);
        }

        trace!("Event loop closed");

        Ok(())
    }
}
