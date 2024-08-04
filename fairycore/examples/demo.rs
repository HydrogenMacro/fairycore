use std::any::TypeId;

use fairycore::prelude::*;
fn main() {
	let mut app = App::new().background_color(RED).window_size(800, 600);
	let mut scene = Scene::new();
	scene.set_layers(vec!["bg", "player", "fg"]);
	app.root.add_child(scene);
	scene
		.query_trait(vec![
			trait_id::<dyn Entity3D>(),
			exclude_trait_id::<dyn EventListener>(),
		])
		.for_each(|entity| {
			scene.remove(entity);
		});

	app.run();
}

#[derive(EventTarget)]
#[inherit(Entity3D)]
pub struct Player {
	transform: Transform,
}
impl Node for Player {
	fn enter_tree() {}
}
impl Lifecycle for Player {
	fn start(&self) {}
	fn update(&self) {
		self.emit(MoveEvent)
	}
	fn cleanup(&self) {}
}
impl EventListener<MoveEvent> for Player {
	fn on_receive<T>(&mut self, event: &T) {}
}
pub trait Inheritable {
	type Types;
	const FIELD_NAMES: &'static [&'static str];
}
