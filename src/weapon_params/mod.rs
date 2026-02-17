use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Deserialize, Serialize)]
pub struct WeaponParams {
    pub blade_length: BladeLength,
    pub blade_width: BladeWidth,
    pub blade_curvature: f64,
    pub blade_direction: Direction,
    pub blade_count: u8,
    pub has_guard: bool,

    // Could not implement: would be better applied in texture which is outside of the scope of the project
    // pub handle_material: WeaponMaterial,
    // pub guard_material: WeaponMaterial,
    // pub pommel_material: WeaponMaterial,
    // pub blade_material: WeaponMaterial,
    ///
    
    pub guard_coverage: GuardCoverage,
    pub guard_plate_shape: GuardPlateShape,
    pub guard_plate_curvature: f64,
    
    /// Could not implement: ran out of time + would have texture implications
    // pub age: f32,
    // pub ornamental_level: f32,
    ///

    pub blade_thickness: f64,
    
    /// Could not implement: ran out of time
    // pub period: TimePeriod,
    ///

    pub handle_length: HandleLength,
    pub blade_type: BladeType
}

impl Default for WeaponParams {
    fn default() -> Self {
        WeaponParams {
            blade_length: BladeLength::Medium,
            blade_width: BladeWidth::Standard,
            blade_curvature: 0.0,
            blade_direction: Direction::Central,
            blade_count: 2,
            has_guard: true,
            guard_coverage: GuardCoverage::Plate,
            guard_plate_shape: GuardPlateShape::Flat,
            guard_plate_curvature: 1.0,
            // handle_material: WeaponMaterial::Wood,
            // guard_material: WeaponMaterial::Steel,
            // pommel_material: WeaponMaterial::Bone,
            // blade_material: WeaponMaterial::Steel,
            // age: 1.0,
            // ornamental_level: 0.5,
            blade_thickness: 20.0,
            // period: TimePeriod::Medieval,
            handle_length: HandleLength::OneHanded,
            blade_type: BladeType::Sharp,
        }
    }
}

#[derive(strum::IntoStaticStr, Deserialize, Serialize)]
pub enum BladeType {
    Sharp,
    Dull,
    Serated,
    Spikey,
    SpikeyAndSerated,
}

#[derive(strum::IntoStaticStr, Deserialize, Serialize)]
pub enum HandleLength {
    Dagger,
    OneHanded,
    TwoHanded,
    ForearmLength,
    Polearm
}

#[derive(strum::IntoStaticStr, Deserialize, Serialize)]
pub enum BladeLength {
    Short,
    Medium,
    Long,
    Great,
}

#[derive(strum::IntoStaticStr, Deserialize, Serialize)]
pub enum GuardCoverage {
    Open,
    Bar,
    SemiEnclosed,
    Plate,
    Shell,
    Complex,
    Enclosed,
}

#[derive(strum::IntoStaticStr, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum GuardPlateShape {
    Flat,
    Horseshoe,
    Dome,
    Bowl,
    BowlCapped,
    Upturned,
}

#[derive(Debug, PartialEq, Clone, Copy, IntoStaticStr, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Central,
    Right,
}

#[derive(strum::IntoStaticStr, Deserialize, Serialize)]
pub enum BladeWidth {
    Narrow,
    Standard,
    Wide,
}

#[derive(strum::IntoStaticStr, Deserialize, Serialize)]
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

#[derive(strum::IntoStaticStr, Deserialize, Serialize)]
pub enum TimePeriod {
    Neanderthal,
    Classical,
    Medieval,
    Crusador,
    Colonial,
    Industrial,
    SpaceAge,
    Contempary,
    SciFi,
}
