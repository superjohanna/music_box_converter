// These macros let you configure the ui from the settings.rs file.

#[macro_export]
/// This needs to be called with all items you want to add
/// See settings.rs for usage
macro_rules! config_macro_list_items {
    ($enclosing:ty, $($item:ident),+ $(,)?) => {
        pub fn get_items() -> Vec<(String, String, String, String, ValueType, String)> {
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
    ($group:literal, $group_help:expr; $($item:ident, $item_string:literal, $value_type_enum:expr, $help:expr);+ $(;)?) => {
        $(
            paste::paste! {
                fn [<get_ $item _name>] () -> (String, String, String, String, ValueType, String) {
                    return ($group.to_string(), $group_help.to_string(), stringify!($item).to_string(), $item_string.to_string(), $value_type_enum, $help.to_string());
                }
            }
        )+
    };
}
