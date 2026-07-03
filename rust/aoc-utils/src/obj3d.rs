use std::cmp::Ordering;

#[derive(Debug,Default,Copy,Clone,Hash,Eq,PartialEq)]
pub struct Rectangle2d {
    w: usize,
    h: usize
}

#[derive(Debug,Default,Copy,Clone,Hash,Eq,PartialEq)]
pub struct Rectangle3d {
    l: usize,
    h: usize,
    w: usize
}

impl Rectangle2d {
    
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h }
    }

    pub fn set_width(&mut self, w: usize) {
        self.w = w;
    }

    pub fn set_height(&mut self, h: usize) {
        self.h = h;
    }

    pub fn area(&self) -> u64 {
        (self.h * self.w) as u64
    }
}

impl Rectangle3d {

    pub fn new(l: usize, w: usize, h: usize) -> Self {
        Self { w, h, l }
    }

    pub fn set_width(&mut self, w: usize) {
        self.w = w;
    }

    pub fn set_height(&mut self, h: usize) {
        self.h = h;
    }

    pub fn set_length(&mut self, l: usize) {
        self.l = l;
    }

    pub fn cmp(&self, b: &Rectangle3d) -> Ordering {
        self.h.cmp(&b.h)
            .then(self.l.cmp(&b.l))
            .then(self.w.cmp(&b.w))
    }

    fn get_smallest_sides(&self) -> (usize,usize) {
        let mut v = vec![self.w, self.h, self.l];
        v.sort();
        (v[0], v[1])
    }

    pub fn smallest_perimeter(&self) -> u64 {
        let v= self.get_smallest_sides();
        (v.0*2 + v.1*2) as u64
    }

    pub fn area(&self) -> u64 {
        let wh = self.w * self.h;
        let wl = self.w * self.l;
        let hl = self.h * self.l;
        (2 * (wh + wl + hl)) as u64
    }

    pub fn area_smallest_sides(&self) -> u64 {
        let v = self.get_smallest_sides();
        (v.0 * v.1) as u64
    }

    pub fn volume(&self) -> u64 {
        (self.h * self.w * self.l) as u64
    }
}