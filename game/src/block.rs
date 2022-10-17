#[derive(Copy, Clone, Debug, Default)]
pub enum Side {
    #[default]
    Front = 0,
    Back,
    Top,
    Bottom,
    Left,
    Right,
}

impl From<Side> for u8 {
    fn from(s: Side) -> Self {
        s as u8
    }
}
impl From<Side> for usize {
    fn from(s: Side) -> Self {
        s as usize
    }
}

#[derive(Clone, Debug, Default)]
pub struct BlockType {
    name: String,
    texture_index: [u8; 6],
}

impl BlockType {
    pub fn new(name: &str) -> Self {
        let texture_index: [u8; 6] = [0; 6];
        let name = name.to_string();
        Self {
            name,
            texture_index,
        }
    }
    pub fn with_texture(mut self, tex: u8) -> Self {
        self.texture_index = [tex; 6];
        self
    }
    pub fn with_texture_side(mut self, tex: u8, side: Side) -> Self {
        let i: usize = side.into();
        self.texture_index[i] = tex;
        self
    }

    pub fn _with_texture_front(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Front)
    }
    pub fn _with_texture_back(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Back)
    }
    pub fn _with_texture_left(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Left)
    }
    pub fn _with_texture_right(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Right)
    }
    pub fn with_texture_top(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Top)
    }
    pub fn with_texture_bottom(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Bottom)
    }

    pub fn tex_front(&self) -> u8 {
        self.texture_index[Side::Front as usize]
    }
    pub fn tex_back(&self) -> u8 {
        self.texture_index[Side::Back as usize]
    }
    pub fn tex_left(&self) -> u8 {
        self.texture_index[Side::Left as usize]
    }
    pub fn tex_right(&self) -> u8 {
        self.texture_index[Side::Right as usize]
    }
    pub fn tex_top(&self) -> u8 {
        self.texture_index[Side::Top as usize]
    }
    pub fn tex_bottom(&self) -> u8 {
        self.texture_index[Side::Bottom as usize]
    }

    pub fn load_all() -> Vec<BlockType> {
        vec![
            BlockType::new("Air"),
            BlockType::new("Dirt").with_texture(1),
            BlockType::new("Grass")
                .with_texture(16)
                .with_texture_top(0)
                .with_texture_bottom(1),
            BlockType::new("Stone").with_texture(2),
            BlockType::new("Coal").with_texture(3),
            BlockType::new("Spruce log").with_texture(4),
            BlockType::new("Spruce leaves").with_texture(5),
            BlockType::new("Dry grass")
                .with_texture(22)
                .with_texture_top(6)
                .with_texture_bottom(1),
            BlockType::new("Roots").with_texture(7),
            BlockType::new("Obsidian").with_texture(8),
            BlockType::new("Oak log").with_texture(9),
            BlockType::new("Oak leaves").with_texture(10),
            BlockType::new("Iron ore (hematite)").with_texture(11),
            BlockType::new("Marble block").with_texture(12),
            BlockType::new("Marble pillar")
                .with_texture(13)
                .with_texture_top(12)
                .with_texture_bottom(12),
            BlockType::new("Marble blocks").with_texture(14),
            BlockType::new("Acacia leaves").with_texture(24),
            BlockType::new("Boards").with_texture(17),
            BlockType::new("Crystals").with_texture(18),
            BlockType::new("Sakura leaves").with_texture(19),
            BlockType::new("Birch log").with_texture(20),
            BlockType::new("Flower bush").with_texture(21),
            BlockType::new("Date bush").with_texture(23),
        ]
    }
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = &self.name;
        write!(f, "<BlockType name={} />", name)
    }
}
