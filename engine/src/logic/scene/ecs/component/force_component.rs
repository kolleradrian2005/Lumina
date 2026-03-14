use crate::math::vec3::Vec3;

use super::component::Component;

#[derive(Component)]
pub struct ForceComponent {
    pub mass: f32,
    forces: Vec<AppliedForce>,
}

pub struct AppliedForce {
    pub id: String,
    pub effect: ForceEffect,
    pub mode: ForceMode,
}

pub enum ForceEffect {
    Linear(Vec3),
    Drag(f32),
}

pub enum ForceMode {
    Impulse,
    Continuous,
}

impl ForceComponent {
    pub fn new(mass: f32) -> Self {
        ForceComponent {
            mass,
            forces: Vec::new(),
        }
    }
    pub fn apply_force(&mut self, force: AppliedForce) {
        self.forces.push(force);
    }
    pub fn remove_applied_force(&mut self, id: &str) {
        self.forces.retain(|f| f.id != id);
    }
    pub fn get_linear_force_vecs(&self) -> impl Iterator<Item = Vec3> + '_ {
        self.forces.iter().filter_map(|f| match f.effect {
            ForceEffect::Linear(vec) => Some(vec.clone()),
            _ => None,
        })
    }
    pub fn get_drag_force_factors(&self) -> impl Iterator<Item = f32> + '_ {
        self.forces.iter().filter_map(|f| match f.effect {
            ForceEffect::Drag(drag) => Some(drag),
            _ => None,
        })
    }
    pub fn get_linear_forces(&self) -> impl Iterator<Item = Vec3> + '_ {
        self.forces.iter().filter_map(|f| match f.effect {
            ForceEffect::Linear(vec) => Some(vec.clone()),
            _ => None,
        })
    }
    pub fn clear_impulses(&mut self) {
        self.forces
            .retain(|force| !matches!(force.mode, ForceMode::Impulse));
    }
}
