use macroquad::prelude::*;

trait Renderable {
    fn render(&self) {
    }
}

fn random_vec2() -> Vec2 {
    Vec2::new(
        rand::gen_range(0., screen_width()),
        rand::gen_range(0., screen_width()))
}

struct World {
    creatures: Vec<Creature>,
    food: Vec<Food>,
}
impl Renderable for World {
    fn render(&self) {
        for c in &self.creatures {
            c.render()
        }
        for f in &self.food { 
            f.render()
        }
    }
}


impl World {
    fn new() -> Self {
        Self {
            creatures : (0..35).map(|_| Creature::random()).collect(),
            food : (0..100).map(|_| Food::random()).collect(),
        }
    }
}

struct Creature {
    pos: Vec2,
    consumed: f32,

}

impl Creature {
    fn random () -> Self {
     Self { pos: random_vec2(),
     consumed: 0.,
     }   

   } 
}

struct Food {
    pos: Vec2,
}

impl Food {
    fn random() -> Self {
     Self { pos: random_vec2()  }   
    }
}

impl Renderable for Creature {
    fn render(&self) {
        draw_circle(self.pos.x,self.pos.y, 15.0, WHITE);
    }
}
impl Renderable for Food {
    fn render(&self) {
        draw_circle(self.pos.x,self.pos.y, 15.0, GREEN);
    }
}



#[macroquad::main("InputKeys")]
async fn main() {
    let world = World::new();
    loop {
    clear_background(RED);
  
    world.render(); 

    next_frame().await;
    }
}
