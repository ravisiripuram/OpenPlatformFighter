use common::fighter::{Fighter, AnimationArray};
use common::animation::{AnimationState, Animation};
use common::frame::{Frame, FrameData, FrameType};

pub fn new<'a>() -> Fighter<'a> {
    let mut aa: AnimationArray = Default::default();
    aa += Animation::new(
        AnimationState::Idle,
        vec![FrameData(
            &[
                &[Frame::Hurt(&[[0.0, -6.0, 6.0],]),],
                &[Frame::Hurt(&[[0.0, -6.1, 6.1],]),],
                &[Frame::Hurt(&[[0.0, -6.3, 6.3],]),],
                &[Frame::Hurt(&[[0.0, -6.5, 6.5],]),],
                &[Frame::Hurt(&[[0.0, -7.0, 7.0],]),],
                &[Frame::Hurt(&[[0.0, -7.3, 7.3],]),],
                &[Frame::Hurt(&[[0.0, -7.7, 7.7],]),],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]),],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]),],
                &[Frame::Hurt(&[[0.0, -7.0, 7.0],]),],
                &[Frame::Hurt(&[[0.0, -6.7, 6.7],]),],
                &[Frame::Hurt(&[[0.0, -6.5, 6.5],]),],
                &[Frame::Hurt(&[[0.0, -6.0, 6.0],]),],
                &[Frame::Hurt(&[[0.0, -6.0, 6.0],]),]
            ]
        )],
        vec![FrameType::single(14,0)],
    );
    aa += Animation::new(
        AnimationState::Walk,
        vec![
            FrameData(&[
                &[Frame::Hurt(&[[0.0, -10.0, 10.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -10.0, 10.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -10.5, 10.5]                   ]),],
                &[Frame::Hurt(&[[0.0, -11.0, 11.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -12.0, 12.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -13.5, 13.5]                   ]),],
            ]),
            FrameData(&[
                &[Frame::Hurt(&[[0.0, -14.5, 14.5],[0.0, -27.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -15.0, 15.0],[0.0, -37.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -15.0, 15.0],[0.0, -47.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -12.0, 14.7]                   ]),],
                &[Frame::Hurt(&[[0.0, -12.0, 14.5]                   ]),],
                &[Frame::Hurt(&[[0.0, -12.0, 14.3]                   ]),],
            ]),
            FrameData(&[
                &[Frame::Hurt(&[[0.0, -12.0, 14.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -12.0, 13.7]                   ]),],
                &[Frame::Hurt(&[[0.0, -12.0, 13.5]                   ]),],
                &[Frame::Hurt(&[[0.0, -12.0, 13.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -10.0, 12.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -10.0, 11.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -10.0, 10.0]                   ]),],
            ]),
        ],
        vec![
            FrameType::single(6,0),
            FrameType::repeat(6,0),
            FrameType::single(7,0),
        ]
    );
    aa += Animation::new(
        AnimationState::Jump,
        vec![FrameData(
            &[
                &[Frame::Hurt(&[[0.0, -13.5, 13.5],[0.0, -20.5, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -14.5, 14.5],[0.0, -22.5, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -15.0, 15.0],[0.0, -24.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -15.0, 15.0],[0.0, -25.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -12.0, 14.7],[0.0, -23.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -12.0, 14.5],[0.0, -24.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -12.0, 14.3],[0.0, -23.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -12.0, 14.0],[0.0, -22.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -12.0, 13.7],[0.0, -21.0, 12.0]]),],
                &[Frame::Hurt(&[[0.0, -12.0, 13.5]                   ]),],
                &[Frame::Hurt(&[[0.0, -12.0, 13.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -10.0, 12.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -10.0, 11.0]                   ]),],
                &[Frame::Hurt(&[[0.0, -10.0, 11.0]                   ]),],
            ])],
        vec![FrameType::single(14,0)],
    );
    aa += Animation::new(
        AnimationState::Jab,
        vec![FrameData(
            &[
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]), Frame::Hit(&[[9.0, -8.0, 3.5],])],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]), Frame::Hit(&[[9.0, -8.0, 3.5],])],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]), Frame::Hit(&[[9.0, -8.0, 3.5],])],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]), Frame::Hit(&[[9.0, -8.0, 3.5],])],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]),],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]),],
            ]
        )],
        vec![FrameType::single(6,0)],
    );
    Fighter {
        aa: aa,
        astate: AnimationState::Idle,
        weight:  10.0,
        walkspeed:  100.0,
        init_fallspeed: 20.0,
        max_fallspeed:  50.0,
        jumpheight: 1000.0,
        jumpspeed: 14000.0,
    }
}
