pub struct WeaponParams {
    pub blade_length: BladeLength,
    pub blade_width: BladeWidth,
    pub blade_type: BladeType,
    pub blade_count: u8,
    pub has_guard: bool,
    pub handle_material: WeaponMaterial,
    pub blade_material: WeaponMaterial,
    pub age: f32,
    pub ornamental_level: f32,
}

pub enum BladeLength {
    Short,
    Medium,
    Long,
    Great,
}

pub enum BladeWidth {
    Narrow,
    Standard,
    Wide,
}

pub enum BladeType {

}

pub enum WeaponMaterial {
    Wood,
    Leather,
    Iron,
    Steel,
    Mithril,
    Titanium,
    Synthetic,
    Plant,
    Cork,
    Stone,
    Bone,
}