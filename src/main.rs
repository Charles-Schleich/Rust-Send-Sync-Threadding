
use std::thread;
use std::sync::Arc;
mod foo_example;
use foo_example::*;

pub trait Hittable: Send + Sync {
    fn hit(&self, tmin:f64, tmax:f64 ) -> Option<f64>;

    fn displ(&self) -> String;
}

#[derive(Debug)]
pub struct Sphere  {
    pub center: f64,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self,tmin:f64, tmax:f64 ) -> Option<f64>{
        if self.center > 10.0 {
            return Some(100.0);
        }
        return None;
    }
    fn displ(&self) -> String {
        format!("r:{}   c:{}", self.radius, self.center)
    }
}

pub struct HittableList{
    pub objects: Vec<Box<Hittable + Send + 'static + Sync>>
}

impl HittableList{

    pub fn new() -> HittableList {
        HittableList {
            objects:Vec::new()
        }
    }
                                         
    pub fn add(&mut self, sharedptr:Box<Hittable + Send+ 'static + Sync> ){
        self.objects.push(sharedptr);
    }
}

impl Hittable for HittableList {
    fn hit(&self, tmin:f64, tmax:f64, ) -> Option<f64> {
        if tmin<tmax {  return Some(24.0); }
        None
    }

    fn displ(&self ) -> String { 
        "Hittable List ".to_owned() 
    }
}


fn process_image_chunk (thread:i32,world: Arc<HittableList>){
    for obj in &world.objects {
        let x =obj.displ();
        // println!("thread: {}",thread);
        println!("thread: {}, {:?}",thread,x);
    }
    println!("Hello");
}



fn main() {
    // foo_example::main();
    println!("Start complex Example!\n-----------------------");

    let mut world: HittableList = HittableList::new() ; // as Box<dyn Hittable + Send>;

    world.add(Box::new(Sphere{center: 1.0, radius: 10.5}));
    world.add(Box::new(Sphere{center: 2.0, radius: 20.5  }));
    world.add(Box::new(Sphere{center: 3.0, radius: 30.5  }));

    let world_arc = Arc::new(world);

    let mut joinhandles = Vec::new();

    for thread in 0..10 {

        let w_arc_cl = world_arc.clone();
        let handler = thread::spawn(move || process_image_chunk(thread, w_arc_cl));
    
        joinhandles.push(handler);
    }


    
    for jh in joinhandles {
        jh.join().unwrap();
    }
}
