use std::vec::Vec;

#[derive(Debug)]
struct Mat<T> {
    data : Vec<T>,
    row  : usize,
    col  : usize,
    nch  : usize,
    tlen : usize,
    rclen: usize,
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

    pub fn new(r:usize, c: usize, ch: usize) -> Mat<T> {
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
        let mut b : Mat<i32> = Mat::new(3, 3, 3);
        let mut c = b.get_data();
        for i in 0..c.len(){
            c[i] = (i+1) as i32;
        }
        println!("{:?}", c);

    }
}
