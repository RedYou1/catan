use macroquad::prelude::*;

use crate::{
    center::{Center, CenterH, CenterV},
    drawable::Drawable,
    fix_rect::FixRect,
    h_stack::HStack,
    range::Range,
    v_stack::VStack,
    z_stack::ZStack,
};

#[test]
fn test_widgets() {
    let screen = ZStack::new(vec![
        Box::new(FixRect::new(100.0, 100.0, DARKGRAY)),
        Box::new(VStack::new(vec![Box::new(FixRect::new(5.0, 5.0, RED))])),
    ]);

    assert_eq!(
        screen.width(),
        Range {
            min: 100.0,
            max: Some(100.0)
        }
    );
    assert_eq!(
        screen.height(),
        Range {
            min: 100.0,
            max: Some(100.0)
        }
    );

    let screen = ZStack::new(vec![
        Box::new(FixRect::new(100.0, 100.0, DARKGRAY)),
        Box::new(VStack::new(vec![
            Box::new(FixRect::new(5.0, 5.0, RED)),
            Box::new(FixRect::new(100.0, 100.0, GREEN)),
        ])),
    ]);

    assert_eq!(
        screen.width(),
        Range {
            min: 100.0,
            max: Some(100.0)
        }
    );
    assert_eq!(
        screen.height(),
        Range {
            min: 105.0,
            max: Some(105.0)
        }
    );

    let mut screen = Center::new(ZStack::new(vec![Box::new(VStack::new(vec![
        Box::new(HStack::new(vec![
            Box::new(TestRect::new("Center1.1", 1.0, 1.0, 1.0, 1.0)),
            Box::new(TestRect::new("Center2.1", 2.0, 1.0, 1.0, 1.0)),
        ])),
        Box::new(HStack::new(vec![
            Box::new(TestRect::new("Center1.2", 1.0, 2.0, 1.0, 1.0)),
            Box::new(TestRect::new("Center2.2", 2.0, 2.0, 1.0, 1.0)),
        ])),
    ]))]));
    assert_eq!(
        screen.width(),
        Range {
            min: 2.0,
            max: None
        }
    );
    assert_eq!(
        screen.height(),
        Range {
            min: 2.0,
            max: None
        }
    );
    screen.draw(0.0, 0.0, 4.0, 4.0);

    let mut screen = Center::new(ZStack::new(vec![
        Box::new(TestRect::new("Back", 50.0, 47.5, 100.0, 100.0)),
        Box::new(VStack::new(vec![
            Box::new(CenterH::new(TestRect::new("Little", 97.5, 47.5, 5.0, 5.0))),
            Box::new(TestRect::new("Big", 50.0, 52.5, 100.0, 100.0)),
        ])),
    ]));

    assert_eq!(
        screen.width(),
        Range {
            min: 100.0,
            max: None
        }
    );
    assert_eq!(
        screen.height(),
        Range {
            min: 105.0,
            max: None
        }
    );
    screen.draw(0.0, 0.0, 200.0, 200.0);

    let mut screen = Center::new(ZStack::new(vec![
        Box::new(TestRect::new("Back", 0.0, 0.0, 200.0, 200.0)),
        Box::new(Center::new(VStack::new(vec![
            Box::new(CenterH::new(TestRect::new("Title", 75.0, 0.0, 50.0, 10.0))),
            Box::new(CenterH::new(HStack::new(vec![
                Box::new(CenterV::new(TestRect::new("T1", 50.0, 45.0, 20.0, 10.0))),
                Box::new(CenterV::new(TestRect::new("T2", 70.0, 45.0, 20.0, 10.0))),
                Box::new(CenterV::new(TestRect::new("T3", 90.0, 45.0, 20.0, 10.0))),
                Box::new(CenterV::new(TestRect::new("T4", 110.0, 45.0, 20.0, 10.0))),
                Box::new(CenterV::new(TestRect::new("T5", 130.0, 45.0, 20.0, 10.0))),
            ]))),
            Box::new(CenterH::new(HStack::new(vec![
                Box::new(CenterV::new(VStack::new(vec![
                    Box::new(CenterH::new(TestRect::new("E1.1", 0.0, 125.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E1.2", 0.0, 135.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E1.3", 0.0, 145.0, 40.0, 10.0))),
                ]))),
                Box::new(CenterV::new(VStack::new(vec![
                    Box::new(CenterH::new(TestRect::new("E2.1", 40.0, 125.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E2.2", 40.0, 135.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E2.3", 40.0, 145.0, 40.0, 10.0))),
                ]))),
                Box::new(CenterV::new(VStack::new(vec![
                    Box::new(CenterH::new(TestRect::new("E3.1", 80.0, 125.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E3.2", 80.0, 135.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E3.3", 80.0, 145.0, 40.0, 10.0))),
                ]))),
                Box::new(CenterV::new(VStack::new(vec![
                    Box::new(CenterH::new(TestRect::new("E4.1", 120.0, 125.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E4.2", 120.0, 135.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E4.3", 120.0, 145.0, 40.0, 10.0))),
                ]))),
                Box::new(CenterV::new(VStack::new(vec![
                    Box::new(CenterH::new(TestRect::new("E5.1", 160.0, 125.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E5.2", 160.0, 135.0, 40.0, 10.0))),
                    Box::new(CenterH::new(TestRect::new("E5.3", 160.0, 145.0, 40.0, 10.0))),
                ]))),
            ]))),
            Box::new(CenterH::new(TestRect::new("submit", 75.0, 190.0, 50.0, 10.0))),
        ]))),
    ]));

    assert_eq!(
        screen.width(),
        Range {
            min: 200.0,
            max: None
        }
    );
    assert_eq!(
        screen.height(),
        Range {
            min: 200.0,
            max: None
        }
    );
    screen.draw(0.0, 0.0, 200.0, 200.0);
}

struct TestRect<'a> {
    message: &'a str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl<'a> TestRect<'a> {
    pub fn new(message: &'a str, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            message,
            x,
            y,
            width,
            height,
        }
    }
}

impl<'a> Drawable for TestRect<'a> {
    fn width(&self) -> Range {
        Range {
            min: self.width,
            max: Some(self.width),
        }
    }

    fn height(&self) -> Range {
        Range {
            min: self.height,
            max: Some(self.height),
        }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        assert_eq!(self.x, x, "x:{}", self.message);
        assert_eq!(self.y, y, "y:{}", self.message);
        assert_eq!(self.width, width, "w:{}", self.message);
        assert_eq!(self.height, height, "h:{}", self.message);
    }
}
