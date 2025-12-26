pub struct WeaponParams {
    pub blade_length: BladeLength,
    pub blade_width: BladeWidth,
    pub blade_curvature: BladeCurvature,
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
    Rapier,
    Narrow,
    Standard,
    Wide,
}

pub enum BladeCurvature {
    Straight,
    Curved,
    RightAngled,
    Circular,
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