use std::io;
pub fn readCom()->(f64,f64){

    let mut x = String::new();
    let mut y = String::new();

    println!("Real Part : ");
    io::stdin().read_line(&mut x).expect("invalid input");
    println!("Imaginary Part : ");
    io::stdin().read_line(&mut y).expect("invalid input");

    let x :f64 = x.trim().parse().expect("invalid input");
    let y :f64 = y.trim().parse().expect("invalid input");
    (x,y)
}

pub use crate::Operations;

pub mod Operations{   
     pub fn ComAdd(c1:&(f64,f64), c2:&(f64,f64))->(f64,f64){
        (c1.0 + c2.0, c1.1 + c2.1)
    }

    pub fn ComSubstract(c1:&(f64,f64), c2:&(f64,f64))->(f64,f64){
        (c1.0 - c2.0, c1.1 - c2.1)
    }

    pub fn ComDistance(c1:&(f64,f64), c2:&(f64,f64))->f64{
        let x= c1.0 - c2.0;
        let y= c1.1 - c2.1;
        let d= x*x + y*y;
        d.sqrt()
    }

    pub fn ComMult(c1:&(f64,f64), c2:&(f64,f64))-> (f64, f64){

        let x = (c1.0*c2.0)-(c1.1*c2.1);
        let y = (c1.1*c2.0)+(c1.0*c2.1);
        (x,y)
    }

    pub fn ComDiv(c1:&(f64,f64), c2:&(f64,f64))-> (f64, f64){
        let x_num = (c1.0*c2.0)+(c1.1*c2.1);
        let y_num = (c1.1*c2.0)-(c1.0*c2.1);
        let den = c2.0*c2.0 + c2.1*c2.1;
        (x_num/den,y_num/den)
    }

    pub fn ComAdd_Inv(c1: &(f64,f64))->(f64,f64){
        (-c1.0,-c1.1)
    }

    pub fn ComMult_Inv(c1: &(f64,f64)) ->(f64,f64){

        let x_num = c1.0;
        let y_num = -c1.1;
        let den = c1.0*c1.0 + c1.1*c1.1;
        (x_num/den,y_num/den)

    }

    pub fn ComMod(c1: &(f64,f64))->f64{

        let x = c1.0*c1.0;
        let y = c1.1*c1.1;
        let m = x+y;
        m.sqrt()
    }

    pub fn CompCompare(c1: &(f64,f64),c2 :&(f64,f64)){

        let d1 = ComMod(c1) ;
        let d2 = ComMod(c2);
        if d1>d2      {   println!("{:?} is greater than {:?}.",c1,c2);   }
        else if d2>d1 {  println!("{:?} is greater than {:?}.",c2,c1);    }
        else         {  println!("{:?} is equal to {:?}",c1,c2);     }
    }

}
