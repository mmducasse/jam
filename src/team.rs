use bevy::ecs::component::Component;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum TeamId {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Team(pub TeamId);
