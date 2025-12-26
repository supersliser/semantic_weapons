use fidget::{Context, context::Tree, jit::JitShape, mesh::{Octree, Settings}};
use semantic_weapons::model_generator::create_basic_weapon;
fn main() {

    let mut ctx = Context::new();
    let sdf = create_basic_weapon();

    let shape = JitShape::from(sdf);
    let settings = Settings {
        depth: 4,
        ..Default::default()
    };
    let o = Octree::build(&shape, &settings).unwrap();
    let mesh = o.walk_dual();
    let mut f = std::fs::File::create("out.stl").unwrap();
    mesh.write_stl(&mut f).unwrap();
}