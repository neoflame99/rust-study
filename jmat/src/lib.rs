use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Mat<T> {
    pub data : Vec<T>,
    pub row  : usize,
    pub col  : usize,
    pub nch  : usize,
    pub tlen : usize,
    pub rclen: usize,
}

impl<T> Mat<T> {
    #[inline] pub fn get_row(&self) -> usize { self.row }
    #[inline] pub fn get_col(&self) -> usize { self.col }
    #[inline] pub fn get_chn(&self) -> usize { self.nch }
    #[inline] pub fn get_len(&self) -> usize { self.tlen}
    #[inline] pub fn get_rclen(&self) -> usize {self.rclen}
    #[inline] pub fn get_data(&mut self) -> &mut Vec<T> {&mut(self.data)}

    pub fn set_size(&mut self, r: usize, c: usize, ch: usize) {
        self.row   = r;
        self.col   = c;
        self.nch   = ch;
        self.rclen = r*c;
        self.tlen  = self.rclen *ch;
    }
    pub fn new(v: Vec<T>, r:usize, c: usize, ch: usize) -> Mat<T> {
        let rclen = r * c;
        let size = rclen * ch;
        let mut v: Vec<T> = Vec::with_capacity(size);
        unsafe { v.set_len(size); }
        Mat {
            data : v,
            row  : r,
            col  : c,
            nch  : ch,
            tlen : size,
            rclen: rclen,
        }
    }
}
impl Mat<f64> {
    pub fn new_f64(r:usize, c: usize, ch: usize) -> Mat<f64> {
        let rclen = r * c;
        let size = rclen * ch;
        let mut v = vec![0.0f64; size];
        Mat {
            data : v,
            row  : r,
            col  : c,
            nch  : ch,
            tlen : size,
            rclen: rclen,
        }
    }
}
impl Mat<f32> {
    pub fn new_f32(r:usize, c: usize, ch: usize) -> Mat<f32> {
        let rclen = r * c;
        let size = rclen * ch;
        let mut v = vec![0.0f32; size];
        Mat {
            data : v,
            row  : r,
            col  : c,
            nch  : ch,
            tlen : size,
            rclen: rclen,
        }
    }
}
impl Mat<i64> {
    pub fn new_i64(r:usize, c: usize, ch: usize) -> Mat<i64> {
        let rclen = r * c;
        let size = rclen * ch;
        let mut v = vec![0i64; size];
        Mat {
            data : v,
            row  : r,
            col  : c,
            nch  : ch,
            tlen : size,
            rclen: rclen,
        }
    }
}
impl Mat<i32> {
    pub fn new_i32(r:usize, c: usize, ch: usize) -> Mat<i32> {
        let rclen = r * c;
        let size = rclen * ch;
        let mut v = vec![0i32; size];
        Mat {
            data : v,
            row  : r,
            col  : c,
            nch  : ch,
            tlen : size,
            rclen: rclen,
        }
    }
}
/*
trait<T> construct {
    fn new(&self) -> &jmat<T>;
    fn new(&self, r: u32, c:u32, ch:u32) -> &jmat<T>;
}
impl<T> construct<T> for jmat<T>{
    fn new(&mut self) -> &jmat<T>{
        self.setSize(0,0,0);
        self.data = vec![0;0];
        &self

    }
    fn new(&self, r: u32, c:u32, ch:u32) -> &jmat<T>{
        self.setSize(r,c,ch);
        self.data = vec![0;self.tlen];
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut b : Mat<i32> = Mat::new_i32(3, 3, 3);
        let mut t = vec![0i32; 10];
        let mut b : Mat<i32> ;
        b.data = t;
        let mut c = b.get_data();
        for i in 0..c.len(){
            c[i] = (i+1) as i32;
        }
        println!("{:?}", c);

    }
}
