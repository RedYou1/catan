use crate::ressource_manager::RessourceManager;

pub trait TPlayer {
    fn road_remaining(&self) -> u8;
    fn road_remaining_mut(&mut self) -> &mut u8;

    fn house_remaining(&self) -> u8;
    fn house_remaining_mut(&mut self) -> &mut u8;

    fn big_house_remaining(&self) -> u8;
    fn big_house_remaining_mut(&mut self) -> &mut u8;

    fn longuest_road(&self) -> u8;
    fn longuest_road_mut(&mut self) -> &mut u8;

    fn army(&self) -> u8;
    fn army_mut(&mut self) -> &mut u8;

    fn ressources(&self) -> &RessourceManager;
    fn ressources_mut(&mut self) -> &mut RessourceManager;
}
