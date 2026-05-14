pub mod data;
pub use data::Gltf;

fn main() {
    let gltf = Gltf {
        ..Default::default()
    };
    println!("{:?}", gltf);
}
