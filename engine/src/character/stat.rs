use crate::Build;

#[derive(Debug)]
pub enum StatError {
    UnspecifiedStrong,
    UnspecifiedInt,
    UnspecifiedDexterity,
    UnspecifiedAgility,
    UnspecifiedLuck,
    UnspecifiedSpeed,
    UnspecifiedCritical,
}

#[derive(Serialize, Deserialize)]
pub struct Stat {
    strong: u32,    // 힘
    int: u32,       // 지능
    dexterity: u32, // 손재주
    agility: u32,   // 민첩
    luck: u32,      // 운
    speed: u32,     // 속도
    critical: u32,  // 치명
}

#[derive(Default)]
pub struct StatBuilder {
    strong: Option<u32>,
    int: Option<u32>,
    dexterity: Option<u32>,
    agility: Option<u32>,
    luck: Option<u32>,
    speed: Option<u32>,
    critical: Option<u32>,
}

impl Build for StatBuilder {
    type Target = Stat;
    type Error = StatError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(Stat {
            strong: self.strong.ok_or(StatError::UnspecifiedStrong)?,
            int: self.int.ok_or(StatError::UnspecifiedInt)?,
            dexterity: self.dexterity.ok_or(StatError::UnspecifiedDexterity)?,
            agility: self.agility.ok_or(StatError::UnspecifiedAgility)?,
            luck: self.luck.ok_or(StatError::UnspecifiedLuck)?,
            speed: self.speed.ok_or(StatError::UnspecifiedSpeed)?,
            critical: self.critical.ok_or(StatError::UnspecifiedCritical)?,
        })
    }
}

impl StatBuilder {
    pub fn strong(mut self, strong: u32) -> Self {
        self.strong = Some(strong);
        self
    }

    pub fn int(mut self, int: u32) -> Self {
        self.int = Some(int);
        self
    }

    pub fn dexterity(mut self, dexterity: u32) -> Self {
        self.dexterity = Some(dexterity);
        self
    }

    pub fn agility(mut self, agility: u32) -> Self {
        self.agility = Some(agility);
        self
    }

    pub fn luck(mut self, luck: u32) -> Self {
        self.luck = Some(luck);
        self
    }

    pub fn speed(mut self, speed: u32) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn critical(mut self, critical: u32) -> Self {
        self.critical = Some(critical);
        self
    }
}
