// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{TreeIter, TreeModel, TreeStore};
use glib::object::{Cast, IsA};
use glib::translate::*;
use glib::{ToValue, Type, Value};
use libc::c_int;

impl TreeStore {
    #[doc(alias = "gtk_tree_store_newv")]
    #[doc(alias = "gtk_tree_store_new")]
    pub fn new(column_types: &[Type]) -> TreeStore {
        assert_initialized_main_thread!();
        unsafe {
            let mut column_types = column_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
            from_glib_full(ffi::gtk_tree_store_newv(
                column_types.len() as c_int,
                column_types.as_mut_ptr(),
            ))
        }
    }
}

pub trait TreeStoreExtManual: 'static {
    #[doc(alias = "gtk_tree_store_insert_with_values")]
    #[doc(alias = "gtk_tree_store_insert_with_valuesv")]
    fn insert_with_values(
        &self,
        parent: Option<&TreeIter>,
        position: Option<u32>,
        columns: &[u32],
        values: &[&dyn ToValue],
    ) -> TreeIter;

    #[doc(alias = "gtk_tree_store_reorder")]
    fn reorder(&self, parent: &TreeIter, new_order: &[u32]);

    #[doc(alias = "gtk_tree_store_set")]
    #[doc(alias = "gtk_tree_store_set_valuesv")]
    #[doc(alias = "gtk_tree_store_set_valist")]
    fn set(&self, iter: &TreeIter, columns: &[u32], values: &[&dyn ToValue]);

    #[doc(alias = "gtk_tree_store_set_column_types")]
    fn set_column_types(&self, types: &[glib::Type]);

    #[doc(alias = "gtk_tree_store_set_value")]
    fn set_value(&self, iter: &TreeIter, column: u32, value: &dyn ToValue);
}

impl<O: IsA<TreeStore>> TreeStoreExtManual for O {
    fn insert_with_values(
        &self,
        parent: Option<&TreeIter>,
        position: Option<u32>,
        columns: &[u32],
        values: &[&dyn ToValue],
    ) -> TreeIter {
        unsafe {
            assert!(position.unwrap_or(0) <= i32::max_value() as u32);
            assert_eq!(columns.len(), values.len());
            let n_columns = ffi::gtk_tree_model_get_n_columns(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
            ) as u32;
            assert!(columns.len() <= n_columns as usize);
            for (&column, value) in columns.iter().zip(values.iter()) {
                let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                    self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
                    column as c_int,
                ));
                assert!(Value::type_transformable(value.to_value_type(), type_));
            }
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_with_valuesv(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
                position.map_or(-1, |n| n as c_int),
                mut_override(columns.as_ptr() as *const c_int),
                values.to_glib_none().0,
                columns.len() as c_int,
            );
            iter
        }
    }

    fn reorder(&self, parent: &TreeIter, new_order: &[u32]) {
        unsafe {
            let count = ffi::gtk_tree_model_iter_n_children(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
                mut_override(parent.to_glib_none().0),
            );
            let safe_count = count as usize == new_order.len();
            debug_assert!(
                safe_count,
                "Incorrect `new_order` slice length. Expected `{}`, found `{}`.",
                count,
                new_order.len()
            );
            let safe_values = new_order.iter().max().map_or(true, |&max| {
                let max = max as i32;
                max >= 0 && max < count
            });
            debug_assert!(
                safe_values,
                "Some `new_order` slice values are out of range. Maximum safe value: \
                 `{}`. The slice contents: `{:?}`",
                count - 1,
                new_order
            );
            if safe_count && safe_values {
                ffi::gtk_tree_store_reorder(
                    self.as_ref().to_glib_none().0,
                    mut_override(parent.to_glib_none().0),
                    mut_override(new_order.as_ptr() as *const c_int),
                );
            }
        }
    }

    fn set(&self, iter: &TreeIter, columns: &[u32], values: &[&dyn ToValue]) {
        unsafe {
            assert_eq!(columns.len(), values.len());
            let n_columns = ffi::gtk_tree_model_get_n_columns(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
            ) as u32;
            assert!(columns.len() <= n_columns as usize);
            for (&column, value) in columns.iter().zip(values.iter()) {
                assert!(column < n_columns);
                let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                    self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
                    column as c_int,
                ));
                assert!(Value::type_transformable(value.to_value_type(), type_));
            }
            ffi::gtk_tree_store_set_valuesv(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(columns.as_ptr() as *const c_int),
                values.to_glib_none().0,
                columns.len() as c_int,
            );
        }
    }

    fn set_column_types(&self, types: &[glib::Type]) {
        unsafe {
            let types_ptr: Vec<glib::ffi::GType> = types.iter().map(|t| t.to_glib()).collect();
            ffi::gtk_tree_store_set_column_types(
                self.as_ref().to_glib_none().0,
                types.len() as i32,
                mut_override(types_ptr.as_ptr()),
            )
        }
    }

    fn set_value(&self, iter: &TreeIter, column: u32, value: &dyn ToValue) {
        unsafe {
            let columns = ffi::gtk_tree_model_get_n_columns(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
            );
            assert!(column < columns as u32);
            let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
                column as c_int,
            ));
            assert!(Value::type_transformable(value.to_value_type(), type_));
            ffi::gtk_tree_store_set_value(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                column as c_int,
                mut_override(value.to_value().to_glib_none().0),
            );
        }
    }
}
