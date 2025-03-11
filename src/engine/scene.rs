use super::{
    event,
    model::{character, stage},
    render::{self, Drawable},
};

pub struct Scene<'a> {
    character: character::Character,
    stage: Box<dyn stage::Stage>,
    render: &'a mut render::Render,
}

impl Scene<'_> {
    pub fn new(
        character: character::Character,
        stage: Box<dyn stage::Stage>,
        render: &mut render::Render,
    ) -> Scene {
        Scene {
            character,
            stage,
            render,
        }
    }

    pub fn update(&mut self, _event: event::Event) {
        self.character = self.character.update(_event, self);
    }

    pub fn render(&mut self) {
        self.render.render(self.stage.get_render_info());
        self.render.render(self.character.get_render_info());
        self.render.apply();
    }

    pub fn get_stage_info(&self) -> &Box<dyn stage::Stage> {
        &self.stage
    }
}
