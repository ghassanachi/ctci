#[allow(dead_code)]
use std::collections::VecDeque;
use std::ops::Deref;
use std::time::Instant;

#[derive(Debug)]
pub enum Pet {
    Cat(PetInfo),
    Dog(PetInfo),
}

impl Deref for Pet {
    type Target = PetInfo;
    fn deref(&self) -> &Self::Target {
        match self {
            Pet::Cat(info) => info,
            Pet::Dog(info) => info,
        }
    }
}

#[derive(Debug)]
pub struct PetInfo {
    name: String,
    timestamp: Instant,
}

#[derive(Debug)]
pub struct PetShop {
    cats: VecDeque<Pet>,
    dogs: VecDeque<Pet>,
}

impl Pet {
    pub fn new_cat(name: &str) -> Self {
        Self::Cat(PetInfo {
            name: name.to_string(),
            timestamp: Instant::now(),
        })
    }

    pub fn new_dog(name: &str) -> Self {
        Self::Dog(PetInfo {
            name: name.to_string(),
            timestamp: Instant::now(),
        })
    }
}

impl PetShop {
    pub fn new() -> Self {
        Self {
            cats: VecDeque::new(),
            dogs: VecDeque::new(),
        }
    }

    pub fn from_iter<I: IntoIterator<Item = Pet>>(iter: I) -> Self {
        let mut out = Self::new();
        for pet in iter {
            out.enqueue(pet);
        }
        out
    }

    pub fn enqueue(&mut self, pet: Pet) {
        match pet {
            Pet::Cat(_) => self.cats.push_back(pet),
            Pet::Dog(_) => self.dogs.push_back(pet),
        }
    }

    pub fn dequeue_any(&mut self) -> Option<Pet> {
        match (self.cats.front(), self.dogs.front()) {
            (None, Some(_)) => self.dogs.pop_front(),
            (Some(_), None) => self.cats.pop_front(),
            (Some(Pet::Cat(cat)), Some(Pet::Dog(dog))) => {
                if cat.timestamp > dog.timestamp {
                    return self.dogs.pop_front();
                }
                self.cats.pop_front()
            }
            _ => None,
        }
    }

    pub fn dequeue_dog(&mut self) -> Option<Pet> {
        self.dogs.pop_front()
    }

    pub fn dequeue_cat(&mut self) -> Option<Pet> {
        self.cats.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pet_shop_1() {
        let iter = vec![
            Pet::new_cat("cat1"),
            Pet::new_dog("dog1"),
            Pet::new_cat("cat2"),
            Pet::new_cat("cat3"),
            Pet::new_dog("dog2"),
            Pet::new_dog("dog3"),
        ];
        let mut pet_shop = PetShop::from_iter(iter);
        assert_eq!(pet_shop.dequeue_any().unwrap().name, "cat1".to_string());
        assert_eq!(pet_shop.dequeue_dog().unwrap().name, "dog1".to_string());
        assert_eq!(pet_shop.dequeue_dog().unwrap().name, "dog2".to_string());
        assert_eq!(pet_shop.dequeue_cat().unwrap().name, "cat2".to_string());
        assert_eq!(pet_shop.dequeue_any().unwrap().name, "cat3".to_string());
        assert!(pet_shop.dequeue_cat().is_none());
        assert_eq!(pet_shop.dequeue_any().unwrap().name, "dog3".to_string());

        assert!(pet_shop.dequeue_dog().is_none());
        assert!(pet_shop.dequeue_any().is_none());
    }
}
