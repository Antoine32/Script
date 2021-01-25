use crate::quicksort;
use std::ops::{Index, IndexMut};

#[allow(unused_imports)]
use crate::{eprint, eprintln};

pub struct VecFree<T: Clone> {
    vec_val: Vec<T>,
    free_val: Vec<usize>,
}

impl<T: Clone> VecFree<T> {
    pub fn new() -> Self {
        VecFree {
            vec_val: Vec::new(),
            free_val: Vec::new(),
        }
    }

    pub fn add(&mut self, value: T) -> usize {
        if self.free_val.len() > 0 {
            let pos = self.free_val.remove(0);
            self.vec_val[pos] = value;
            pos
        } else {
            self.vec_val.push(value);
            self.vec_val.len() - 1
        }
    }

    pub fn remove(&mut self, pos: usize) {
        if pos + 1 == self.vec_val.len() {
            self.vec_val.remove(pos);
            for i in (0..(self.free_val.len())).rev() {
                if self.free_val[i] + 1 == self.vec_val.len() {
                    self.vec_val.remove(self.free_val.remove(i));
                } else {
                    break;
                }
            }
        } else {
            self.free_val.push(pos);
            quicksort(&mut self.free_val);
        }
    }

    pub fn retrieve_all(&self) -> Vec<T> {
        let mut v = self.vec_val.clone();
        let mut count = 0;

        for i in 0..(self.vec_val.len()) {
            if count >= self.free_val.len() {
                break;
            }

            if i == self.free_val[count] {
                v.remove(i - count);
                count += 1;
            }
        }

        v
    }
}

impl<T: Clone> Index<usize> for VecFree<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec_val[index]
    }
}

impl<T: Clone> IndexMut<usize> for VecFree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec_val[index]
    }
}

impl<T: Clone> Clone for VecFree<T> {
    fn clone(&self) -> Self {
        VecFree {
            vec_val: self.vec_val.clone(),
            free_val: self.free_val.clone(),
        }
    }
}
