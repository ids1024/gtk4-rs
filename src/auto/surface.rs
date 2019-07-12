// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AnchorHints;
use CairoContext;
use Cursor;
use Device;
use Display;
use Error;
use Event;
use FrameClock;
use FullscreenMode;
use GLContext;
use Geometry;
use Gravity;
use ModifierType;
use Monitor;
use Rectangle;
use SurfaceEdge;
use SurfaceHints;
use SurfaceState;
use SurfaceType;
use SurfaceTypeHint;
use Texture;
use VulkanContext;
use WMDecoration;
use WMFunction;
use cairo;
use cairo_sys;
use gdk_sys;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Surface(Object<gdk_sys::GdkSurface, gdk_sys::GdkSurfaceClass, SurfaceClass>);

    match fn {
        get_type => || gdk_sys::gdk_surface_get_type(),
    }
}

impl Surface {
    pub fn new_child<P: IsA<Surface>>(parent: &P, position: &Rectangle) -> Surface {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_surface_new_child(parent.as_ref().to_glib_none().0, position.to_glib_none().0))
        }
    }

    pub fn new_popup(display: &Display, position: &Rectangle) -> Surface {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_surface_new_popup(display.to_glib_none().0, position.to_glib_none().0))
        }
    }

    pub fn new_temp(display: &Display) -> Surface {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_surface_new_temp(display.to_glib_none().0))
        }
    }

    pub fn new_toplevel(display: &Display, width: i32, height: i32) -> Surface {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_surface_new_toplevel(display.to_glib_none().0, width, height))
        }
    }

    pub fn constrain_size(geometry: &mut Geometry, flags: SurfaceHints, width: i32, height: i32) -> (i32, i32) {
        assert_initialized_main_thread!();
        unsafe {
            let mut new_width = mem::uninitialized();
            let mut new_height = mem::uninitialized();
            gdk_sys::gdk_surface_constrain_size(geometry.to_glib_none_mut().0, flags.to_glib(), width, height, &mut new_width, &mut new_height);
            (new_width, new_height)
        }
    }
}

pub const NONE_SURFACE: Option<&Surface> = None;

pub trait SurfaceExt: 'static {
    fn beep(&self);

    fn begin_move_drag(&self, button: i32, x: i32, y: i32, timestamp: u32);

    fn begin_move_drag_for_device(&self, device: &Device, button: i32, x: i32, y: i32, timestamp: u32);

    fn begin_resize_drag(&self, edge: SurfaceEdge, button: i32, x: i32, y: i32, timestamp: u32);

    fn begin_resize_drag_for_device(&self, edge: SurfaceEdge, device: &Device, button: i32, x: i32, y: i32, timestamp: u32);

    fn coords_from_parent(&self, parent_x: f64, parent_y: f64) -> (f64, f64);

    fn coords_to_parent(&self, x: f64, y: f64) -> (f64, f64);

    fn create_cairo_context(&self) -> Option<CairoContext>;

    fn create_gl_context(&self) -> Result<GLContext, Error>;

    //fn create_similar_surface(&self, content: /*Ignored*/cairo::Content, width: i32, height: i32) -> Option<cairo::Surface>;

    fn create_vulkan_context(&self) -> Result<VulkanContext, Error>;

    fn deiconify(&self);

    fn destroy(&self);

    fn focus(&self, timestamp: u32);

    fn freeze_updates(&self);

    fn fullscreen(&self);

    fn fullscreen_on_monitor(&self, monitor: &Monitor);

    fn get_accept_focus(&self) -> bool;

    fn get_children(&self) -> Vec<Surface>;

    fn get_cursor(&self) -> Option<Cursor>;

    fn get_decorations(&self) -> Option<WMDecoration>;

    fn get_device_cursor(&self, device: &Device) -> Option<Cursor>;

    fn get_device_position(&self, device: &Device) -> (Option<Surface>, f64, f64, ModifierType);

    fn get_display(&self) -> Option<Display>;

    fn get_focus_on_map(&self) -> bool;

    fn get_frame_clock(&self) -> Option<FrameClock>;

    fn get_frame_extents(&self) -> Rectangle;

    fn get_fullscreen_mode(&self) -> FullscreenMode;

    fn get_geometry(&self) -> (i32, i32, i32, i32);

    fn get_height(&self) -> i32;

    fn get_modal_hint(&self) -> bool;

    fn get_origin(&self) -> (i32, i32, i32);

    fn get_parent(&self) -> Option<Surface>;

    fn get_pass_through(&self) -> bool;

    fn get_position(&self) -> (i32, i32);

    fn get_root_coords(&self, x: i32, y: i32) -> (i32, i32);

    fn get_root_origin(&self) -> (i32, i32);

    fn get_scale_factor(&self) -> i32;

    fn get_state(&self) -> SurfaceState;

    fn get_support_multidevice(&self) -> bool;

    fn get_surface_type(&self) -> SurfaceType;

    fn get_toplevel(&self) -> Option<Surface>;

    fn get_type_hint(&self) -> SurfaceTypeHint;

    fn get_width(&self) -> i32;

    fn has_native(&self) -> bool;

    fn hide(&self);

    fn iconify(&self);

    fn input_shape_combine_region(&self, shape_region: &cairo::Region, offset_x: i32, offset_y: i32);

    fn is_destroyed(&self) -> bool;

    fn is_input_only(&self) -> bool;

    fn is_viewable(&self) -> bool;

    fn is_visible(&self) -> bool;

    fn lower(&self);

    fn maximize(&self);

    fn merge_child_input_shapes(&self);

    fn move_(&self, x: i32, y: i32);

    fn move_resize(&self, x: i32, y: i32, width: i32, height: i32);

    fn move_to_rect(&self, rect: &Rectangle, rect_anchor: Gravity, surface_anchor: Gravity, anchor_hints: AnchorHints, rect_anchor_dx: i32, rect_anchor_dy: i32);

    fn peek_children(&self) -> Vec<Surface>;

    fn queue_expose(&self);

    fn raise(&self);

    fn register_dnd(&self);

    fn resize(&self, width: i32, height: i32);

    fn restack<P: IsA<Surface>>(&self, sibling: Option<&P>, above: bool);

    fn set_accept_focus(&self, accept_focus: bool);

    fn set_child_input_shapes(&self);

    fn set_cursor(&self, cursor: Option<&Cursor>);

    fn set_decorations(&self, decorations: WMDecoration);

    fn set_device_cursor(&self, device: &Device, cursor: &Cursor);

    fn set_focus_on_map(&self, focus_on_map: bool);

    fn set_fullscreen_mode(&self, mode: FullscreenMode);

    fn set_functions(&self, functions: WMFunction);

    fn set_geometry_hints(&self, geometry: &Geometry, geom_mask: SurfaceHints);

    fn set_icon_list(&self, surfaces: &[Texture]);

    fn set_icon_name(&self, name: Option<&str>);

    fn set_keep_above(&self, setting: bool);

    fn set_keep_below(&self, setting: bool);

    fn set_modal_hint(&self, modal: bool);

    fn set_opacity(&self, opacity: f64);

    fn set_opaque_region(&self, region: Option<&cairo::Region>);

    fn set_pass_through(&self, pass_through: bool);

    fn set_shadow_width(&self, left: i32, right: i32, top: i32, bottom: i32);

    fn set_startup_id(&self, startup_id: &str);

    fn set_support_multidevice(&self, support_multidevice: bool);

    fn set_title(&self, title: &str);

    fn set_transient_for<P: IsA<Surface>>(&self, parent: &P);

    fn set_type_hint(&self, hint: SurfaceTypeHint);

    fn show(&self);

    fn show_unraised(&self);

    fn show_window_menu(&self, event: &Event) -> bool;

    fn stick(&self);

    fn thaw_updates(&self);

    fn unfullscreen(&self);

    fn unmaximize(&self);

    fn unstick(&self);

    fn get_property_mapped(&self) -> bool;

    fn connect_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_moved_to_rect<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_render<F: Fn(&Self, &cairo::Region) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_size_changed<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cursor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mapped_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Surface>> SurfaceExt for O {
    fn beep(&self) {
        unsafe {
            gdk_sys::gdk_surface_beep(self.as_ref().to_glib_none().0);
        }
    }

    fn begin_move_drag(&self, button: i32, x: i32, y: i32, timestamp: u32) {
        unsafe {
            gdk_sys::gdk_surface_begin_move_drag(self.as_ref().to_glib_none().0, button, x, y, timestamp);
        }
    }

    fn begin_move_drag_for_device(&self, device: &Device, button: i32, x: i32, y: i32, timestamp: u32) {
        unsafe {
            gdk_sys::gdk_surface_begin_move_drag_for_device(self.as_ref().to_glib_none().0, device.to_glib_none().0, button, x, y, timestamp);
        }
    }

    fn begin_resize_drag(&self, edge: SurfaceEdge, button: i32, x: i32, y: i32, timestamp: u32) {
        unsafe {
            gdk_sys::gdk_surface_begin_resize_drag(self.as_ref().to_glib_none().0, edge.to_glib(), button, x, y, timestamp);
        }
    }

    fn begin_resize_drag_for_device(&self, edge: SurfaceEdge, device: &Device, button: i32, x: i32, y: i32, timestamp: u32) {
        unsafe {
            gdk_sys::gdk_surface_begin_resize_drag_for_device(self.as_ref().to_glib_none().0, edge.to_glib(), device.to_glib_none().0, button, x, y, timestamp);
        }
    }

    fn coords_from_parent(&self, parent_x: f64, parent_y: f64) -> (f64, f64) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            gdk_sys::gdk_surface_coords_from_parent(self.as_ref().to_glib_none().0, parent_x, parent_y, &mut x, &mut y);
            (x, y)
        }
    }

    fn coords_to_parent(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe {
            let mut parent_x = mem::uninitialized();
            let mut parent_y = mem::uninitialized();
            gdk_sys::gdk_surface_coords_to_parent(self.as_ref().to_glib_none().0, x, y, &mut parent_x, &mut parent_y);
            (parent_x, parent_y)
        }
    }

    fn create_cairo_context(&self) -> Option<CairoContext> {
        unsafe {
            from_glib_full(gdk_sys::gdk_surface_create_cairo_context(self.as_ref().to_glib_none().0))
        }
    }

    fn create_gl_context(&self) -> Result<GLContext, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_sys::gdk_surface_create_gl_context(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn create_similar_surface(&self, content: /*Ignored*/cairo::Content, width: i32, height: i32) -> Option<cairo::Surface> {
    //    unsafe { TODO: call gdk_sys:gdk_surface_create_similar_surface() }
    //}

    fn create_vulkan_context(&self) -> Result<VulkanContext, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_sys::gdk_surface_create_vulkan_context(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn deiconify(&self) {
        unsafe {
            gdk_sys::gdk_surface_deiconify(self.as_ref().to_glib_none().0);
        }
    }

    fn destroy(&self) {
        unsafe {
            gdk_sys::gdk_surface_destroy(self.as_ref().to_glib_none().0);
        }
    }

    fn focus(&self, timestamp: u32) {
        unsafe {
            gdk_sys::gdk_surface_focus(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    fn freeze_updates(&self) {
        unsafe {
            gdk_sys::gdk_surface_freeze_updates(self.as_ref().to_glib_none().0);
        }
    }

    fn fullscreen(&self) {
        unsafe {
            gdk_sys::gdk_surface_fullscreen(self.as_ref().to_glib_none().0);
        }
    }

    fn fullscreen_on_monitor(&self, monitor: &Monitor) {
        unsafe {
            gdk_sys::gdk_surface_fullscreen_on_monitor(self.as_ref().to_glib_none().0, monitor.to_glib_none().0);
        }
    }

    fn get_accept_focus(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_accept_focus(self.as_ref().to_glib_none().0))
        }
    }

    fn get_children(&self) -> Vec<Surface> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_surface_get_children(self.as_ref().to_glib_none().0))
        }
    }

    fn get_cursor(&self) -> Option<Cursor> {
        unsafe {
            from_glib_none(gdk_sys::gdk_surface_get_cursor(self.as_ref().to_glib_none().0))
        }
    }

    fn get_decorations(&self) -> Option<WMDecoration> {
        unsafe {
            let mut decorations = mem::uninitialized();
            let ret = from_glib(gdk_sys::gdk_surface_get_decorations(self.as_ref().to_glib_none().0, &mut decorations));
            if ret { Some(from_glib(decorations)) } else { None }
        }
    }

    fn get_device_cursor(&self, device: &Device) -> Option<Cursor> {
        unsafe {
            from_glib_none(gdk_sys::gdk_surface_get_device_cursor(self.as_ref().to_glib_none().0, device.to_glib_none().0))
        }
    }

    fn get_device_position(&self, device: &Device) -> (Option<Surface>, f64, f64, ModifierType) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut mask = mem::uninitialized();
            let ret = from_glib_none(gdk_sys::gdk_surface_get_device_position(self.as_ref().to_glib_none().0, device.to_glib_none().0, &mut x, &mut y, &mut mask));
            (ret, x, y, from_glib(mask))
        }
    }

    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(gdk_sys::gdk_surface_get_display(self.as_ref().to_glib_none().0))
        }
    }

    fn get_focus_on_map(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_focus_on_map(self.as_ref().to_glib_none().0))
        }
    }

    fn get_frame_clock(&self) -> Option<FrameClock> {
        unsafe {
            from_glib_none(gdk_sys::gdk_surface_get_frame_clock(self.as_ref().to_glib_none().0))
        }
    }

    fn get_frame_extents(&self) -> Rectangle {
        unsafe {
            let mut rect = Rectangle::uninitialized();
            gdk_sys::gdk_surface_get_frame_extents(self.as_ref().to_glib_none().0, rect.to_glib_none_mut().0);
            rect
        }
    }

    fn get_fullscreen_mode(&self) -> FullscreenMode {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_fullscreen_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_geometry(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            gdk_sys::gdk_surface_get_geometry(self.as_ref().to_glib_none().0, &mut x, &mut y, &mut width, &mut height);
            (x, y, width, height)
        }
    }

    fn get_height(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_surface_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn get_modal_hint(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_modal_hint(self.as_ref().to_glib_none().0))
        }
    }

    fn get_origin(&self) -> (i32, i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = gdk_sys::gdk_surface_get_origin(self.as_ref().to_glib_none().0, &mut x, &mut y);
            (ret, x, y)
        }
    }

    fn get_parent(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(gdk_sys::gdk_surface_get_parent(self.as_ref().to_glib_none().0))
        }
    }

    fn get_pass_through(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_pass_through(self.as_ref().to_glib_none().0))
        }
    }

    fn get_position(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            gdk_sys::gdk_surface_get_position(self.as_ref().to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    fn get_root_coords(&self, x: i32, y: i32) -> (i32, i32) {
        unsafe {
            let mut root_x = mem::uninitialized();
            let mut root_y = mem::uninitialized();
            gdk_sys::gdk_surface_get_root_coords(self.as_ref().to_glib_none().0, x, y, &mut root_x, &mut root_y);
            (root_x, root_y)
        }
    }

    fn get_root_origin(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            gdk_sys::gdk_surface_get_root_origin(self.as_ref().to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    fn get_scale_factor(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_surface_get_scale_factor(self.as_ref().to_glib_none().0)
        }
    }

    fn get_state(&self) -> SurfaceState {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_state(self.as_ref().to_glib_none().0))
        }
    }

    fn get_support_multidevice(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_support_multidevice(self.as_ref().to_glib_none().0))
        }
    }

    fn get_surface_type(&self) -> SurfaceType {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_surface_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_toplevel(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(gdk_sys::gdk_surface_get_toplevel(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_hint(&self) -> SurfaceTypeHint {
        unsafe {
            from_glib(gdk_sys::gdk_surface_get_type_hint(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_surface_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn has_native(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_has_native(self.as_ref().to_glib_none().0))
        }
    }

    fn hide(&self) {
        unsafe {
            gdk_sys::gdk_surface_hide(self.as_ref().to_glib_none().0);
        }
    }

    fn iconify(&self) {
        unsafe {
            gdk_sys::gdk_surface_iconify(self.as_ref().to_glib_none().0);
        }
    }

    fn input_shape_combine_region(&self, shape_region: &cairo::Region, offset_x: i32, offset_y: i32) {
        unsafe {
            gdk_sys::gdk_surface_input_shape_combine_region(self.as_ref().to_glib_none().0, shape_region.to_glib_none().0, offset_x, offset_y);
        }
    }

    fn is_destroyed(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_is_destroyed(self.as_ref().to_glib_none().0))
        }
    }

    fn is_input_only(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_is_input_only(self.as_ref().to_glib_none().0))
        }
    }

    fn is_viewable(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_is_viewable(self.as_ref().to_glib_none().0))
        }
    }

    fn is_visible(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_is_visible(self.as_ref().to_glib_none().0))
        }
    }

    fn lower(&self) {
        unsafe {
            gdk_sys::gdk_surface_lower(self.as_ref().to_glib_none().0);
        }
    }

    fn maximize(&self) {
        unsafe {
            gdk_sys::gdk_surface_maximize(self.as_ref().to_glib_none().0);
        }
    }

    fn merge_child_input_shapes(&self) {
        unsafe {
            gdk_sys::gdk_surface_merge_child_input_shapes(self.as_ref().to_glib_none().0);
        }
    }

    fn move_(&self, x: i32, y: i32) {
        unsafe {
            gdk_sys::gdk_surface_move(self.as_ref().to_glib_none().0, x, y);
        }
    }

    fn move_resize(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gdk_sys::gdk_surface_move_resize(self.as_ref().to_glib_none().0, x, y, width, height);
        }
    }

    fn move_to_rect(&self, rect: &Rectangle, rect_anchor: Gravity, surface_anchor: Gravity, anchor_hints: AnchorHints, rect_anchor_dx: i32, rect_anchor_dy: i32) {
        unsafe {
            gdk_sys::gdk_surface_move_to_rect(self.as_ref().to_glib_none().0, rect.to_glib_none().0, rect_anchor.to_glib(), surface_anchor.to_glib(), anchor_hints.to_glib(), rect_anchor_dx, rect_anchor_dy);
        }
    }

    fn peek_children(&self) -> Vec<Surface> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gdk_sys::gdk_surface_peek_children(self.as_ref().to_glib_none().0))
        }
    }

    fn queue_expose(&self) {
        unsafe {
            gdk_sys::gdk_surface_queue_expose(self.as_ref().to_glib_none().0);
        }
    }

    fn raise(&self) {
        unsafe {
            gdk_sys::gdk_surface_raise(self.as_ref().to_glib_none().0);
        }
    }

    fn register_dnd(&self) {
        unsafe {
            gdk_sys::gdk_surface_register_dnd(self.as_ref().to_glib_none().0);
        }
    }

    fn resize(&self, width: i32, height: i32) {
        unsafe {
            gdk_sys::gdk_surface_resize(self.as_ref().to_glib_none().0, width, height);
        }
    }

    fn restack<P: IsA<Surface>>(&self, sibling: Option<&P>, above: bool) {
        unsafe {
            gdk_sys::gdk_surface_restack(self.as_ref().to_glib_none().0, sibling.map(|p| p.as_ref()).to_glib_none().0, above.to_glib());
        }
    }

    fn set_accept_focus(&self, accept_focus: bool) {
        unsafe {
            gdk_sys::gdk_surface_set_accept_focus(self.as_ref().to_glib_none().0, accept_focus.to_glib());
        }
    }

    fn set_child_input_shapes(&self) {
        unsafe {
            gdk_sys::gdk_surface_set_child_input_shapes(self.as_ref().to_glib_none().0);
        }
    }

    fn set_cursor(&self, cursor: Option<&Cursor>) {
        unsafe {
            gdk_sys::gdk_surface_set_cursor(self.as_ref().to_glib_none().0, cursor.to_glib_none().0);
        }
    }

    fn set_decorations(&self, decorations: WMDecoration) {
        unsafe {
            gdk_sys::gdk_surface_set_decorations(self.as_ref().to_glib_none().0, decorations.to_glib());
        }
    }

    fn set_device_cursor(&self, device: &Device, cursor: &Cursor) {
        unsafe {
            gdk_sys::gdk_surface_set_device_cursor(self.as_ref().to_glib_none().0, device.to_glib_none().0, cursor.to_glib_none().0);
        }
    }

    fn set_focus_on_map(&self, focus_on_map: bool) {
        unsafe {
            gdk_sys::gdk_surface_set_focus_on_map(self.as_ref().to_glib_none().0, focus_on_map.to_glib());
        }
    }

    fn set_fullscreen_mode(&self, mode: FullscreenMode) {
        unsafe {
            gdk_sys::gdk_surface_set_fullscreen_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_functions(&self, functions: WMFunction) {
        unsafe {
            gdk_sys::gdk_surface_set_functions(self.as_ref().to_glib_none().0, functions.to_glib());
        }
    }

    fn set_geometry_hints(&self, geometry: &Geometry, geom_mask: SurfaceHints) {
        unsafe {
            gdk_sys::gdk_surface_set_geometry_hints(self.as_ref().to_glib_none().0, geometry.to_glib_none().0, geom_mask.to_glib());
        }
    }

    fn set_icon_list(&self, surfaces: &[Texture]) {
        unsafe {
            gdk_sys::gdk_surface_set_icon_list(self.as_ref().to_glib_none().0, surfaces.to_glib_none().0);
        }
    }

    fn set_icon_name(&self, name: Option<&str>) {
        unsafe {
            gdk_sys::gdk_surface_set_icon_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_keep_above(&self, setting: bool) {
        unsafe {
            gdk_sys::gdk_surface_set_keep_above(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_keep_below(&self, setting: bool) {
        unsafe {
            gdk_sys::gdk_surface_set_keep_below(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_modal_hint(&self, modal: bool) {
        unsafe {
            gdk_sys::gdk_surface_set_modal_hint(self.as_ref().to_glib_none().0, modal.to_glib());
        }
    }

    fn set_opacity(&self, opacity: f64) {
        unsafe {
            gdk_sys::gdk_surface_set_opacity(self.as_ref().to_glib_none().0, opacity);
        }
    }

    fn set_opaque_region(&self, region: Option<&cairo::Region>) {
        unsafe {
            gdk_sys::gdk_surface_set_opaque_region(self.as_ref().to_glib_none().0, mut_override(region.to_glib_none().0));
        }
    }

    fn set_pass_through(&self, pass_through: bool) {
        unsafe {
            gdk_sys::gdk_surface_set_pass_through(self.as_ref().to_glib_none().0, pass_through.to_glib());
        }
    }

    fn set_shadow_width(&self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe {
            gdk_sys::gdk_surface_set_shadow_width(self.as_ref().to_glib_none().0, left, right, top, bottom);
        }
    }

    fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            gdk_sys::gdk_surface_set_startup_id(self.as_ref().to_glib_none().0, startup_id.to_glib_none().0);
        }
    }

    fn set_support_multidevice(&self, support_multidevice: bool) {
        unsafe {
            gdk_sys::gdk_surface_set_support_multidevice(self.as_ref().to_glib_none().0, support_multidevice.to_glib());
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            gdk_sys::gdk_surface_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_transient_for<P: IsA<Surface>>(&self, parent: &P) {
        unsafe {
            gdk_sys::gdk_surface_set_transient_for(self.as_ref().to_glib_none().0, parent.as_ref().to_glib_none().0);
        }
    }

    fn set_type_hint(&self, hint: SurfaceTypeHint) {
        unsafe {
            gdk_sys::gdk_surface_set_type_hint(self.as_ref().to_glib_none().0, hint.to_glib());
        }
    }

    fn show(&self) {
        unsafe {
            gdk_sys::gdk_surface_show(self.as_ref().to_glib_none().0);
        }
    }

    fn show_unraised(&self) {
        unsafe {
            gdk_sys::gdk_surface_show_unraised(self.as_ref().to_glib_none().0);
        }
    }

    fn show_window_menu(&self, event: &Event) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_surface_show_window_menu(self.as_ref().to_glib_none().0, event.to_glib_none().0))
        }
    }

    fn stick(&self) {
        unsafe {
            gdk_sys::gdk_surface_stick(self.as_ref().to_glib_none().0);
        }
    }

    fn thaw_updates(&self) {
        unsafe {
            gdk_sys::gdk_surface_thaw_updates(self.as_ref().to_glib_none().0);
        }
    }

    fn unfullscreen(&self) {
        unsafe {
            gdk_sys::gdk_surface_unfullscreen(self.as_ref().to_glib_none().0);
        }
    }

    fn unmaximize(&self) {
        unsafe {
            gdk_sys::gdk_surface_unmaximize(self.as_ref().to_glib_none().0);
        }
    }

    fn unstick(&self) {
        unsafe {
            gdk_sys::gdk_surface_unstick(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_mapped(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"mapped\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<P, F: Fn(&P, &Event) -> bool + 'static>(this: *mut gdk_sys::GdkSurface, event: *mut gdk_sys::GdkEvent, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<Surface>
        {
            let f: &F = &*(f as *const F);
            f(&Surface::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(event)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"event\0".as_ptr() as *const _,
                Some(transmute(event_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    //fn connect_moved_to_rect<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented flipped_rect: *.Pointer
    //    Unimplemented final_rect: *.Pointer
    //}

    fn connect_render<F: Fn(&Self, &cairo::Region) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn render_trampoline<P, F: Fn(&P, &cairo::Region) -> bool + 'static>(this: *mut gdk_sys::GdkSurface, region: *mut cairo_sys::cairo_region_t, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<Surface>
        {
            let f: &F = &*(f as *const F);
            f(&Surface::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(region)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"render\0".as_ptr() as *const _,
                Some(transmute(render_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_size_changed<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn size_changed_trampoline<P, F: Fn(&P, i32, i32) + 'static>(this: *mut gdk_sys::GdkSurface, width: libc::c_int, height: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<Surface>
        {
            let f: &F = &*(f as *const F);
            f(&Surface::from_glib_borrow(this).unsafe_cast(), width, height)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"size-changed\0".as_ptr() as *const _,
                Some(transmute(size_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_cursor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cursor_trampoline<P, F: Fn(&P) + 'static>(this: *mut gdk_sys::GdkSurface, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Surface>
        {
            let f: &F = &*(f as *const F);
            f(&Surface::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::cursor\0".as_ptr() as *const _,
                Some(transmute(notify_cursor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_mapped_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mapped_trampoline<P, F: Fn(&P) + 'static>(this: *mut gdk_sys::GdkSurface, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Surface>
        {
            let f: &F = &*(f as *const F);
            f(&Surface::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mapped\0".as_ptr() as *const _,
                Some(transmute(notify_mapped_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(this: *mut gdk_sys::GdkSurface, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Surface>
        {
            let f: &F = &*(f as *const F);
            f(&Surface::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::state\0".as_ptr() as *const _,
                Some(transmute(notify_state_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Surface")
    }
}
