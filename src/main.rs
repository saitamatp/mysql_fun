use std::env;
mod prm_reader;
use prm_reader::find_value;
use prm_reader::read;
mod csv_read;
use csv_read::csv_read;
mod clear_tmp;
use clear_tmp::clear_tmp;
mod load_to_base;
use load_to_base::load_base;
mod period_end;
use period_end::period_end;
mod credit_load;
use credit_load::credit_load;

/*
1-> CSV to temp
2->temp to load
3->Clear temporary tables
4->Credit Load
5->Run period End SP 
*/
fn main() {
    
    let prm1:i32;
    let args: Vec<String> = env::args().collect();
         
    if args.len()==1{
        prm1=100;
    }else {
     prm1 = args[1].trim().parse().expect("Please enter a valid number");
    }

    let values=read();
    let host_name=find_value(&values,"host".to_string());
    let ports=find_value(&values,"port".to_string());
    let user_name=find_value(&values,"username".to_string());
    let passwords=find_value(&values,"password".to_string());

    let url;
    url=format!("mysql://{}:{}@{}:{}",user_name,passwords,host_name,ports);
    println!("The connection url is mysql://{}:XXXX@{}:{}",user_name,host_name,ports);
    let path1:String=find_value(&values,"src_file1".to_string());
    let path2:String=find_value(&values,"src_file2".to_string());

    if prm1==1
    {
    println!("{}",csv_read(url,path1));
    }else if prm1==2
    {
        println!("{}",load_base(url));
    }
    else if prm1==3
    {
        println!("{}",clear_tmp(url));
    }
    else if prm1==4
    {
        println!("{}",credit_load(url,path2));
    }else if prm1==5
    {
        println!("{}",period_end(url));
    }
    /*Check if the program is correctly trigeering rust code */
    else if prm1==100{
        println!("Hello World from Rust :)")
    }
    else 
    {
        println!("Not able to find the given argument")
    }
}
