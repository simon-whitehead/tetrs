macro_rules! RcCell {
    ( $val:expr ) => (
        Rc::new(Cell::new($val))
    )
}

macro_rules! menu {
    ( ($config:expr, $asset_factory:expr), $( $text:expr => $result:expr ),* ) => {
        {
            let mut temp_menu = Menu::new($config, $asset_factory);

            $(
                temp_menu.add_item($text, $result);
            )*

            temp_menu
        }
    }
}