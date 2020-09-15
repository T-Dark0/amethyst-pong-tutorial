use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)] //derive default system instantiation
pub struct PaddleSystem;

//Lifetime of the components it operates on
impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>, //reads from and writes to the transform component
        ReadStorage<'s, Paddle>,     //Reads the paddle component
        Read<'s, InputHandler<StringBindings>>, //Reads the input handling resource
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        //Iterate over all entities with both these components
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let side_name = match paddle.side {
                        Side::Left => "left",
                        Side::Right => "right",
                    };
                    println!("Side {:?} moving: {}", side_name, mv_amount);
                }
            }
        }
    }
}
