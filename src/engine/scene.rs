use super::{
    event,
    model::{bullet, character, stage},
    render::{self, Drawable},
};

pub struct Scene<'a> {
    character: character::Character,
    stage: Box<dyn stage::Stage>,
    bullets: Vec<bullet::Bullet>,
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
            bullets: Vec::new(),
            render,
        }
    }

    pub fn update(&mut self, _event: event::Event) {
        let mut command = SceneCommand::default();

        // 既存要素の更新
        self.character.update(_event, &self.stage, &mut command);
        self.bullets
            .iter_mut()
            .for_each(|bullet| bullet.update(&self.stage));
        self.bullets.retain(|bullet| !bullet.is_outside);

        // 新規要素の追加
        self.execute_command(command);
    }

    pub fn render(&mut self) {
        self.render.render(self.stage.get_render_info());
        self.render.render(self.character.get_render_info());
        self.bullets.iter().for_each(|bullet| {
            self.render.render(bullet.get_render_info());
        });
        self.render.apply();
    }

    fn execute_command(&mut self, command: SceneCommand) {
        command.append_bullet.into_iter().for_each(|bullet| {
            self.bullets.push(bullet);
        });
    }
}

#[derive(Default)]
pub struct SceneCommand {
    append_bullet: Vec<bullet::Bullet>,
}

impl SceneCommand {
    pub fn append_bullet(&mut self, bullet: bullet::Bullet) {
        self.append_bullet.push(bullet);
    }
}
