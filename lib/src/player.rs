use crate::ressource_manager::RessourceManager;

pub trait TPlayer {
    fn road_remaining(&self) -> usize;
    fn road_remaining_mut(&mut self) -> &mut usize;

    fn house_remaining(&self) -> usize;
    fn house_remaining_mut(&mut self) -> &mut usize;

    fn big_house_remaining(&self) -> usize;
    fn big_house_remaining_mut(&mut self) -> &mut usize;

    fn longuest_road(&self) -> usize;
    fn longuest_road_mut(&mut self) -> &mut usize;

    fn army(&self) -> usize;
    fn army_mut(&mut self) -> &mut usize;

    fn ressources(&self) -> &RessourceManager;
    fn ressources_mut(&mut self) -> &mut RessourceManager;
}
