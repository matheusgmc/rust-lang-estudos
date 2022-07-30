fn sum(n1:i8,n2:i8) -> i8{
    return n1+n2;
}

fn mult(n1:i8,n2:i8) -> i8{
    return n1 * n2; 
}

fn main() {
    let n1:i8 = 10;
    let n2:i8 = 3;

    let result_sum = sum(n1, n2);
    let result_mult = mult(n1,n2);

    println!("soma: {} multiplicação: {:?}",result_sum,result_mult);

    let short_sum= |n1:i8,n2:i8| -> i8 { n1+n2};
    let short_mult= |n1:i8 ,n2:i8| -> i8 {n1*n2};

    let short_result_sum = short_sum(n1,n2);
    let short_result_mult = short_mult(n1,n2);

    println!("soma curta: {} multiplicação curta: {}",short_result_sum,short_result_mult);

}
