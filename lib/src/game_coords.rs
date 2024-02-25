pub const fn building_around_tile(x: u8, y: u8) -> [(u8, u8); 6] {
    let x = x * 2 + (y % 2);
    [
        (x, y),
        (x + 1, y),
        (x + 2, y),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 2, y + 1),
    ]
}
pub fn building_near_building(x: u8, y: u8) -> Vec<(u8, u8)> {
    let mut buildings: Vec<(u8, u8)> = hroad_near_building(x, y)
        .iter()
        .flat_map(|(x, y)| building_near_hroad(*x, *y))
        .collect();
    if let Some((x, y)) = vroad_near_building(x, y) {
        buildings.extend(building_near_vroad(x, y));
    }
    buildings
}
pub fn hroad_near_vroad(x: u8, y: u8) -> Vec<(u8, u8)> {
    let buildings = building_near_vroad(x, y);
    let mut hroads_1 = hroad_near_building(buildings[0].0, buildings[0].1);
    hroads_1.extend(hroad_near_building(buildings[1].0, buildings[1].1));
    hroads_1
}
pub fn vroad_near_hroad(x: u8, y: u8) -> Vec<(u8, u8)> {
    let buildings = building_near_hroad(x, y);
    let mut res = Vec::with_capacity(2);
    if let Some(vroad) = vroad_near_building(buildings[0].0, buildings[0].1) {
        res.push(vroad);
    }
    if let Some(vroad) = vroad_near_building(buildings[1].0, buildings[1].1) {
        res.push(vroad);
    }
    res
}
pub fn hroad_near_hroad(x: u8, y: u8) -> Vec<(u8, u8)> {
    if x == 0 {
        vec![(1, y)]
    } else if x == 9 {
        vec![(8, y)]
    } else {
        vec![(x - 1, y), (x + 1, y)]
    }
}
pub const fn building_near_vroad(x: u8, y: u8) -> [(u8, u8); 2] {
    let off = y % 2;
    [(x * 2 + off, y), (x * 2 + off, y + 1)]
}
pub const fn vroad_near_building(x: u8, y: u8) -> Option<(u8, u8)> {
    let off = if x % 2 == y % 2 { 0 } else { 1 };
    if y == 0 && off == 1 {
        return None;
    }
    if y == 5 && off == 0 {
        return None;
    }
    Some((x / 2, y - off))
}
pub const fn building_near_hroad(x: u8, y: u8) -> [(u8, u8); 2] {
    [(x, y), (x + 1, y)]
}
pub fn hroad_near_building(x: u8, y: u8) -> Vec<(u8, u8)> {
    if x == 0 {
        vec![(0, y)]
    } else if x == 10 {
        vec![(9, y)]
    } else {
        vec![(x - 1, y), (x, y)]
    }
}