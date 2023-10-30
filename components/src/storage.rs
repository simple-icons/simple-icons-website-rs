#[allow(non_snake_case)]
pub mod LocalStorage {
    pub enum Keys {
        DownloadType,
        OrderMode,
        SearchValue,
        ColorScheme,
        Layout,
        Language,
    }

    impl Keys {
        pub fn as_str(&self) -> &'static str {
            match self {
                Keys::DownloadType => "download-type",
                Keys::OrderMode => "order-mode",
                Keys::SearchValue => "search-value",
                Keys::ColorScheme => "color-scheme",
                Keys::Layout => "layout",
                Keys::Language => "language",
            }
        }
    }
}

macro_rules! _base_impl_get_from_localstorage {
    ($localstorage_key:ident, $some_return_expr:expr, $value:ident) => {
        match window()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(LocalStorage::Keys::$localstorage_key.as_str())
        {
            Ok(Some($value)) => $some_return_expr,
            _ => None,
        }
    };
}

macro_rules! conversion_get_from_localstorage {
    ($localstorage_key:ident, $from_str_dyn:ident) => {
        $crate::storage::_base_impl_get_from_localstorage!(
            $localstorage_key,
            $from_str_dyn::from_str(value.as_str()).ok(),
            value
        )
    };
}

macro_rules! transparent_get_from_localstorage {
    ($localstorage_key:ident) => {
        $crate::storage::_base_impl_get_from_localstorage!(
            $localstorage_key,
            Some(value),
            value
        )
    };
}

pub(crate) use _base_impl_get_from_localstorage;
pub(crate) use conversion_get_from_localstorage;
pub(crate) use transparent_get_from_localstorage;

macro_rules! set_on_localstorage {
    ($localstorage_key:ident, $value:expr) => {
        window()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(LocalStorage::Keys::$localstorage_key.as_str(), $value)
            .unwrap()
    };
}

pub(crate) use set_on_localstorage;
