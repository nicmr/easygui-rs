/// Marker trait for functions in lib.rs
pub trait ConrodIds{

}
/// Contains text to be displayed in a corresponding Widget
pub trait TextContainer{
    fn title(&self) -> &str;
}