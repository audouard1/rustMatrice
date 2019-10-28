use std::time::{Duration, Instant};
use std::thread::sleep;


fn main() {
    let mut tab : Vec<f64> = Vec::new();
    let mut res : Vec<f64> = Vec::new();
    for _i in 0..100{
        res.push(1.0);
    }
    init_zero(&mut tab, 100_i32);
    a_bord1(&mut tab, 100_i32);
    let now = Instant::now();
    gauss_supp(&mut tab, &mut res,100);
    gauss_inff(&mut tab, &mut res,100);
    gauss_res(&mut tab, &mut res,100);
    println!("{}", now.elapsed().as_secs_f32());
    //display_tab_res(&mut res, 1000_i32);
}

fn gauss_supp(tab : &mut Vec<f64>, res : &mut Vec<f64>, d : i32){
    let mut coeff;
    for i in 0..d {
        for n in i+1..d {
            coeff = tab[(n*d + i) as usize]/tab[(i*d + i)  as usize];
            for j in i+i..d{
                tab[(n*d+j)  as usize] = tab[(n*d+j)  as usize] - coeff*tab[(i*d+j)  as usize];
            }
            res[n  as usize] = res[n  as usize] - coeff*res[i  as usize];
        }
    }
}

fn init_zero(tab : &mut Vec<f64>, d : i32){
    for _i in 0..d*d {
        tab.push(0.0);
    }
}

fn a_bord1(tab : &mut Vec<f64>, d : i32){
    for i in 0..d {
        for j in 0..d {
            if i+1 == j+1 {
                tab[(i*d+j) as usize] = 1.0;
            }
            else{
                if i == 0 {
                    tab[(i*d+j) as usize] = 2_f64.powf((-j).into());
                    tab[(j*d) as usize] = 2_f64.powf((-j).into());
                }               
            }
        }
    }
}

fn gauss_inff(tab : &mut Vec<f64>, res : &mut Vec<f64>, d : i32){
    let mut coeff;
    for i in (1..d).rev(){
        for n in (0..i).rev(){
            coeff = tab[(n*d + i) as usize]/tab[(i*d + i) as usize];
            tab[(n*d+i) as usize] = tab[(n*d+i) as usize] - coeff*tab[(i*d+i) as usize];
            res[n as usize] = res[n as usize] - coeff*res[i as usize];
        }
    }
}

fn gauss_res(tab : &mut Vec<f64>, res : &mut Vec<f64>, d : i32){
    for i in 0..d{
        res[i as usize]= res[i as usize]/tab[(i*d+i) as usize];
    }
}

fn display_tab(tab : &mut Vec<f64>, d : i32){
    for i in 0..d {
        for j in 0..d {
            print!("{} ",tab[(i*d+j) as usize]);
        }
        print!("\n");
    }
}

fn display_tab_res(tab : &mut Vec<f64>, d : i32){
    for i in 0..d {
        print!("{} \n",tab[i as usize]);
    }
}