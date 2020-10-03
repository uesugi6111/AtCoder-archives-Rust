#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize,s:Chars);

    let mut count = 0;
        for i in (2..n+1).step_by(2){

            let mut dna = vec![0;4];
            let mut left =0;
            for j in 0..i{
                dna[convert_dna(&s[j])]+=1;
            }
            loop {

                if dna[0]==dna[1] &&  dna[2]==dna[3]{
                    count+=1;
                } 

if !(left+i<n){
    break;
}


                    dna[convert_dna(&s[left+i])]+=1;
               
                
                    dna[convert_dna(&s[left])]-=1;
                


                left+=1;
            }

        }
    

    println!("{}", count );
}

fn convert_dna(c:&char)->usize{
    if *c=='A'{0}else if *c=='T'{1}else if *c=='C'{2}else {3}

}
