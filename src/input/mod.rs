use nom_locate::LocatedSpan;
use uuid::Uuid;

pub type Span<'a> = LocatedSpan<&'a str, InputSource>;

#[derive(Copy, Clone, Debug)]
pub struct InputSource {
    pub id: Uuid,
}
