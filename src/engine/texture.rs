#[derive(Debug)]
pub struct Texture
{
    id: u32
}

impl Texture {
    pub fn new(id: u32) -> Self {
        Texture { id }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
