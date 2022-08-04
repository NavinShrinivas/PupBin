pub fn help_print(){
    println!(r"   ___            ___  _    
  / _ \__ _____  / _ )(_)__ 
 / ___/ // / _ \/ _  / / _ \
/_/   \_,_/ .__/____/_/_//_/
         /_/               ");
    println!("Usage :\t $ executable --option1 value --option2 value --option3 ...");
    println!("value is not must for all options");
    println!("Options :");
    println!("--paste ./file/path.txt : To paste content of file to share");
    println!("--get key : to get the content present in paste using key");
    println!("--help : to get to this page");
    println!("--verbose : to get more output and diagonstics");
    println!("All output can be redirected!");
}
