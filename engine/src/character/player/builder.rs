use crate::{Build, ClassId, ItemCount, ItemId, Player, PlayerError, Stat};

#[derive(Default)]
pub struct PlayerBuilder {
    name: Option<String>,
    class: Option<ClassId>,
    level: Option<u8>,
    health: Option<u32>,
    mana: Option<u32>,
    stamina: Option<u32>,
    stat: Option<Stat>,
    item_list: Option<Vec<(ItemId, ItemCount)>>,
}

impl Build for PlayerBuilder {
    type Target = Player;
    type Error = PlayerError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(Player {
            name: self.name.ok_or(PlayerError::UnspecifiedName)?,
            class: self.class.ok_or(PlayerError::UnspecifiedClass)?,
            level: self.level.ok_or(PlayerError::UnspecifiedLevel)?,
            health: self.health.ok_or(PlayerError::UnspecifiedHealth)?,
            mana: self.mana.ok_or(PlayerError::UnspecifiedMana)?,
            stamina: self.stamina.ok_or(PlayerError::UnspecifiedStamina)?,
            stat: self.stat.ok_or(PlayerError::UnspecifiedStat)?,
            item_list: self.item_list.ok_or(PlayerError::UnspecifiedItemList)?,
        })
    }
}

impl PlayerBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn class(mut self, class: ClassId) -> Self {
        self.class = Some(class);
        self
    }

    pub fn level(mut self, level: u8) -> Self {
        self.level = Some(level);
        self
    }

    pub fn health(mut self, health: u32) -> Self {
        self.health = Some(health);
        self
    }

    pub fn mana(mut self, mana: u32) -> Self {
        self.mana = Some(mana);
        self
    }

    pub fn stamina(mut self, stamina: u32) -> Self {
        self.stamina = Some(stamina);
        self
    }

    pub fn stat(mut self, stat: Stat) -> Self {
        self.stat = Some(stat);
        self
    }

    pub fn item_list(mut self, item_list: Vec<(ItemId, ItemCount)>) -> Self {
        self.item_list = Some(item_list);
        self
    }
}
