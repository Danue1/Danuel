#[derive(Serialize, Deserialize)]
pub enum ItemKind {
    Weapon(WeaponKind),
    Armor(ArmorKind),
    Etc(EtcKind),
}

#[derive(Serialize, Deserialize)]
pub enum WeaponKind {
    //
}

#[derive(Serialize, Deserialize)]
pub enum ArmorKind {
    //
}

#[derive(Serialize, Deserialize)]
pub enum EtcKind {
    //
}
