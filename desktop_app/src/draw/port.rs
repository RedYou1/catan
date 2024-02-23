use crate::draw::building::coords;
use catan_lib::{port::Port, ressource::Ressource};
use macroquad::prelude::*;
use macroquadstate::{fix_circle::FixCircle, line::Line, offset::Offset, z_stack::ZStack, zstack};

#[allow(clippy::similar_names)]
pub fn port(id: u8, port: Port) -> ZStack<3> {
    let coords1 = coords(port.pos1().x(), port.pos1().y());
    let coords2 = coords(port.pos2().x(), port.pos2().y());
    let diffx = coords2.0 - coords1.0;
    let diffy = coords2.1 - coords1.1;
    let moveangle = std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_4;
    let angtocoords3 = diffy.atan2(diffx)
        + if id == 3 || id == 5 || id == 7 || id == 8 {
            moveangle
        } else {
            -moveangle
        };
    let len = (diffx * diffx + diffy * diffy).sqrt();
    let coords3 = (
        coords2.0 + angtocoords3.cos() * len / 1.5,
        coords2.1 + angtocoords3.sin() * len / 1.5,
    );
    zstack![
        Line::new(coords1.0, coords1.1, coords3.0, coords3.1, 5.0, BLACK),
        Line::new(coords2.0, coords2.1, coords3.0, coords3.1, 5.0, BLACK),
        Offset::new(
            coords3.0,
            coords3.1,
            FixCircle::new(
                10.0,
                match port.ressource() {
                    Some(Ressource::Tree) => GREEN,
                    Some(Ressource::Wheet) => GOLD,
                    Some(Ressource::Brick) => RED,
                    Some(Ressource::Sheep) => WHITE,
                    Some(Ressource::Stone) => GRAY,
                    None => YELLOW,
                }
            )
        )
    ]
}
