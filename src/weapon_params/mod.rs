pub struct WeaponParams {
    pub blade_length: BladeLength,
    pub blade_width: BladeWidth,
    pub blade_curvature: u16,
    pub blade_direction: Direction,
    pub blade_count: u8,
    pub has_guard: bool,
    pub handle_material: WeaponMaterial,
    pub guard_material: WeaponMaterial,
    pub guard_coverage: GuardCoverage,
    pub pommel_material: WeaponMaterial,
    pub blade_material: WeaponMaterial,
    pub age: f32,
    pub ornamental_level: f32,
    pub sharpness: u8,
    pub period: TimePeriod
}

pub enum BladeLength {
    Short,
    Medium,
    Long,
    Great,
}

pub enum GuardCoverage {
    Open,
    Bar,
    SemiEnclosed,
    Plate,
    Shell,
    Complex,
    Enclosed,
}

pub enum Direction {
    Left,
    Central,
    Right
}

pub enum BladeWidth {
    Narrow,
    Standard,
    Wide,
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

pub enum TimePeriod {
    Neanderthal,
    Classical
}