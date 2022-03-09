#![allow(non_snake_case)]
use rand::Rng;

pub struct Sudoku {

    vec : Vec<Vec<usize>>,
    N : i32,
    SRN : i32,
    K : i32,
}

impl Sudoku {
    // how to use
    // let sudoku = Sudoku::new(9, 12);
    fn new(N : i32, K : i32) -> Self {
        let srn : i32 = f64::sqrt(N.into()) as i32;
        Sudoku {
            vec : vec![vec![0; N.try_into().unwrap()] ; N.try_into().unwrap()],
            N,
            SRN :srn,
            K,
        }
    }
    fn fillValues(&mut self){
        self.fillDiagonal();
        self.fillRemaining(0,self.SRN);
        // print sudoku here.
        println!("++++++++ SUDOKU before removing K digits +++++++++");
        self.printSudoku2();
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++");
        self.removeKDigits();
    }

    fn fillDiagonal(&mut self){
        for i in (0..self.N).step_by(self.SRN.try_into().unwrap()) {
            self.fillBox(i , i);
        }
    }
    fn unUsedInBox(&self,rowStart:usize , colStart:usize,num:i32) -> bool{
        for i in (0..self.SRN).step_by(1){
            for j in (0..self.SRN).step_by(1){
                if self.vec[rowStart+i as usize][colStart+j as usize] == num as usize {
                    return false;
                }

            }

        }
        true
    }
    fn fillBox(&mut self,row:i32,col:i32) {
        for i in (0..self.SRN).step_by(1) {
            for j in (0..self.SRN).step_by(1) {
                let mut num = self.randomGenerator(self.N);
                while !self.unUsedInBox(row as usize, col as usize, num.try_into().unwrap()) {
                        num = self.randomGenerator(self.N);
                }
                    self.vec[(row + i) as usize][(col + j) as usize] = num;
            }
        }
    }
    fn randomGenerator(&self, num:i32) -> usize {
        let mut rng = rand::thread_rng();
        let random : f64 = rng.gen_range(0.0..1.0);
        let r_num = (random * num as f64 ) + 1 as f64;
        r_num.floor() as usize
         //round::floor(rand::rng()*num+1);

    }
    fn CheckIfSafe(&self,i:i32,j:i32,num:i32) -> bool{
        return self.unUsedInRow(i,num)&&
            self.unUsedInCol(j,num as u32)&&
            self.unUsedInBox((i-i%self.SRN).try_into().unwrap(),(j-j%self.SRN).try_into().unwrap(),num.try_into().unwrap());
    }
    fn unUsedInRow(&self, i:i32,num:i32) -> bool{
        for j in (0..self.N).step_by(1) {
            if self.vec[i as usize][j as usize] == num.try_into().unwrap(){
                return false;
            }
        }
        true
    }
    fn unUsedInCol(&self,j:i32,num:u32) -> bool {
        for i in (0..self.N).step_by(1) {
            if self.vec[i as usize][j as usize] == num as usize{
                return false;
            }
        }
        true
    }
    fn fillRemaining(&mut self,mut i:i32,mut j:i32) -> bool {
        if j>=self.N && i<self.N-1 {
            i = i+1;
            j = 0;
        }
        if i>= self.N && j>=self.N {
            return true;
        }
        if i<self.SRN {
            if j < self.SRN {
                j = self.SRN;
            }
        }
        else if i < self.N - self.SRN {
            if j == i / self.SRN * self.SRN {
                j = j + self.SRN;
                }
        }
        else {
            if j == self.N - self.SRN {
                i = i + 1;
                j = 0;
                if i >= self.N {
                        return true;
                    }
            }
        }
        for num in (1..=self.N).step_by(1) {
            if self.CheckIfSafe(i, j, num) {
                self.vec[i as usize][j as usize] = num as usize;
                if self.fillRemaining(i, j + 1) {
                    return true;
                }
                self.vec[i as usize][j as usize] = 0;
            }
        }
        return false;

    }
    fn removeKDigits(&mut self){
        let mut count = self.K;
        while count != 0 {
               let cellId = self.randomGenerator(self.N*self.N)-1;
                let i = cellId/self.N as usize;
                let mut j = cellId%9;

                if j != 0 {
                    j = j - 1;
                }
                    if self.vec[i as usize][j as usize] != 0 {
                         count = count -1;
                        self.vec[i as usize][j as usize] = 0;
                    }
        }
    }
    fn printSudoku2(&self) {
        for i in (0..self.N).step_by(1) {
            for j in (0..self.N).step_by(1) {
                print!("{:?} ", self.vec[i as usize][j as usize]);
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let N = 9;
    let K = 20;
    let mut Sudoku = Sudoku::new(N, K);
    Sudoku.fillValues();
    Sudoku.printSudoku2();
}
