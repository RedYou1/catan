use crate::ressource_manager::RessourceManager;

pub trait TPlayer {
    fn ressources(&self) -> &RessourceManager;
    fn ressources_mut(&mut self) -> &mut RessourceManager;
}
