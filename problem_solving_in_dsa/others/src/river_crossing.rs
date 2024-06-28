use std::collections::HashSet;

trait Item: Clone + std::fmt::Debug + Sized {}

trait Movement {
    fn movement(&self, arrangement: &mut Arrangement, to: Place, from: Place, carry_item: Box<dyn Item>);
}

#[derive(Debug, Clone)]
struct Farmer;
impl Item for Farmer {}
impl Movement for Farmer {
    fn movement(&self, arrangement: &mut Arrangement, to: Place, from: Place, carry_item: Box<dyn Item>) {
        // Move the carry_item from the 'from' place to the 'to' place.
        match (from, to) {
            (Place::LeftShore, Place::Boat) => {
                arrangement.left_shore.remove(carry_item.as_ref());
                arrangement.boat.insert(carry_item);
            },
            (Place::Boat, Place::RightShore) => {
                arrangement.boat.remove(carry_item.as_ref());
                arrangement.right_shore.insert(carry_item);
            },
            // Add other cases as needed
            _ => (),
        }
    }
}

#[derive(Debug, Clone)]
struct Fox;
impl Item for Fox {}

#[derive(Debug, Clone)]
struct Duck;
impl Item for Duck {}

#[derive(Debug, Clone)]
struct BagOfGrain;
impl Item for BagOfGrain {}

#[derive(Debug, Clone)]
enum Place {
    LeftShore,
    Boat,
    RightShore,
}

#[derive(Debug, Clone, Default)]
struct Arrangement {
    left_shore: HashSet<Box<dyn Item>>,
    boat: HashSet<Box<dyn Item>>,
    right_shore: HashSet<Box<dyn Item>>,
}

fn main() {
    // Example usage
    let farmer = Box::new(Farmer) as Box<dyn Item>;
    let fox = Box::new(Fox) as Box<dyn Item>;
    let duck = Box::new(Duck) as Box<dyn Item>;
    let grain = Box::new(BagOfGrain) as Box<dyn Item>;

    let mut arrangement = Arrangement {
        left_shore: vec![farmer.clone(), fox.clone(), duck.clone(), grain.clone()].into_iter().collect(),
        boat: HashSet::new(),
        right_shore: HashSet::new(),
    };

    let farmer_movement = Farmer;
    farmer_movement.movement(&mut arrangement, Place::Boat, Place::LeftShore, farmer.clone());

    println!("{:?}", arrangement);
}
