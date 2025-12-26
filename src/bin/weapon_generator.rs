use fidget::{
    Context,
    context::Tree,
    jit::JitShape,
    mesh::{Octree, Settings},
};
use nalgebra::Matrix4;
use semantic_weapons::model_generator::create_basic_weapon;

fn main() {
    let sdf = create_basic_weapon(Tree::x(), Tree::y(), Tree::z());
    let world_to_model = Matrix4::new_translation(&nalgebra::Vector3::new(0.0, 17.5, 0.0))
        * Matrix4::new_scaling(25.0);
    let shape = JitShape::from(sdf);
    let settings = Settings {
        depth: 10,
        world_to_model: world_to_model,
        ..Default::default()
    };
    let o = Octree::build(&shape, &settings).unwrap();
    let mesh = o.walk_dual();
    let mut f = std::fs::File::create("out.stl").unwrap();
    mesh.write_stl(&mut f).unwrap();
}
