trait Drawable {}

struct Mesh {
    // Vertex info, etc
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {}
    }
}

struct Instance<'a> {
    mesh: &'a Mesh,
}

impl<'a> Instance<'a> {
    pub fn new(mesh: &'a Mesh) -> Instance<'a> {
        Instance { mesh }
    }
}

impl<'a> Drawable for Instance<'a> {}

struct Scene<'a> {
    entries: Vec<Box<dyn Drawable + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene { entries: vec![] }
    }
}

fn render_scene(scene: Scene) {}

fn main() {
    let mesh = Mesh::new();

    // Add vertices, etc

    let mut scene = Scene::new();
    let b = Box::new(Instance::new(&mesh));
    scene.entries.push(b);

    render_scene(scene)
}
