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




// The key problem here is that Box<dyn T> is equivalent to Box<dyn T + 'static>. But that's just the default lifetime - you can override it by writing Box<dyn T + 'a>.

// The only thing you need to change is to give a lifetime to the Scene and pass that down to the contained Boxes.

// struct Scene<'a> {
//     entries: Vec<Box<dyn Drawable + 'a>>,
// }

// impl<'a> Scene<'a> {
//     pub fn new() -> Scene<'a> {
//         Scene { entries: vec![] }
//     }
// }
// A full example is available on the playground