// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ColorStop;
use crate::RenderNode;
use glib::translate::*;
use glib::StaticType;
use std::fmt;
use std::mem;

glib::wrapper! {
    #[doc(alias = "GskConicGradientNode")]
    pub struct ConicGradientNode(Shared<ffi::GskConicGradientNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for ConicGradientNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_conic_gradient_node_get_type()) }
    }
}

impl ConicGradientNode {
    #[doc(alias = "gsk_conic_gradient_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        center: &graphene::Point,
        rotation: f32,
        color_stops: &[ColorStop],
    ) -> ConicGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_conic_gradient_node_new(
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                rotation,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    #[doc(alias = "gsk_conic_gradient_node_get_angle")]
    #[doc(alias = "get_angle")]
    pub fn angle(&self) -> f32 {
        unsafe { ffi::gsk_conic_gradient_node_get_angle(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_conic_gradient_node_get_center")]
    #[doc(alias = "get_center")]
    pub fn center(&self) -> Option<graphene::Point> {
        unsafe {
            from_glib_none(ffi::gsk_conic_gradient_node_get_center(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_conic_gradient_node_get_color_stops")]
    #[doc(alias = "get_color_stops")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        unsafe {
            let mut n_stops = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gsk_conic_gradient_node_get_color_stops(
                    self.to_glib_none().0,
                    n_stops.as_mut_ptr(),
                ),
                n_stops.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "gsk_conic_gradient_node_get_n_color_stops")]
    #[doc(alias = "get_n_color_stops")]
    pub fn n_color_stops(&self) -> usize {
        unsafe { ffi::gsk_conic_gradient_node_get_n_color_stops(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_conic_gradient_node_get_rotation")]
    #[doc(alias = "get_rotation")]
    pub fn rotation(&self) -> f32 {
        unsafe { ffi::gsk_conic_gradient_node_get_rotation(self.to_glib_none().0) }
    }
}

impl fmt::Display for ConicGradientNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConicGradientNode")
    }
}
