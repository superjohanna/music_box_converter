// These macros let you configure the ui from the settings.rs file.

#[macro_export]
/// This needs to be called with all items you want to add
/// See settings.rs
macro_rules! config_macro_list_items {
    ($enclosing:ty, $($item:ident),+ $(,)?) => {
        pub fn get_items() -> Vec<(String, String, String, ValueType)> {
            paste::paste! {
                let items = vec![
                    $(
                        $enclosing::[<get_ $item _name>](),
                    )+
                ];
            }
            return items;
        }
    };
}

#[macro_export]
/// This needs to be called with all items you want to add to a group
/// See settings.rs
macro_rules! config_macro_add_item {
    ($self:ident, $group:literal, $($item:ident, $item_string:literal, $value_type_enum:expr, $value_type:ident),+ $(,)?) => {
        $(
            paste::paste! {
                fn [<get_ $item _name>] () -> (String, String, String, ValueType) {
                    return ($group.to_string(), stringify!($item).to_string(), $item_string.to_string(), $value_type_enum);
                }
            }
        )+
    };
}
