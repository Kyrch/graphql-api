pub mod anime_document;
pub mod studio_document;

pub trait HasId {
    fn id(&self) -> &str;
}
