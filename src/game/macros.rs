macro_rules! RcCell {
    ( $val:expr ) => (
        Rc::new(Cell::new($val))
    )
}
