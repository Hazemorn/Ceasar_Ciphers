use std::io;
use std::char;
use std::time:: {Instant};

fn main() {
    loop{
    let mut input_text = String::new();
    let mut key_str= String::new();

    //TEST ONE
    //let test_fir = String::from("Hello and bye");
    //let test_sec= "Bye".to_string();
    /////
/* 
    //first try
    //let eng_let: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x', 'y','z']; 
    
    //let mut eng_letters: HashMap<usize,char> = HashMap::new(); //let mut eng_letters =['a';26];

    //let mut i=0;
    //while i<eng_let.len() {//&& j<eng_letters.len(){
    //    eng_letters.insert(i, eng_let[i]);
    //    i+=1;
    //}
    //println!("{:?}", eng_letters);
*/
    println!("Input the text:");
    match io::stdin().read_line(&mut input_text){
        Ok(_)=>{println!("The text accepted")},
        Err(_e)=>{eprintln!("No input text.Please try again")}
    }
    
    println!("Input the key:");
    /*match io::stdin().read_line(&mut key){
        Ok(_)=>println!("The key accepted"),
        Err(_e)=> eprintln!("No input. The key set is default")    
    }
    */
    io::stdin().read_line(&mut key_str)
        .expect("Failed to read line");
    let key:u32 = key_str.trim().parse().expect("Input not an integer");

    //TEST 
    //let char_vec_fir: Vec<char>= test_fir.chars().collect();
   // let char_vec_sec:Vec<char> = test_sec.chars().collect();
    //////
    convert(key ,input_text);
       
    }
}


fn convert ( key:u32 , input_text: String){
    let start = Instant::now();
    let char_vec:Vec<char>= input_text.chars().collect();
    println!("Result: ");
    for n in char_vec{
        if n != ' '{
            let c = n as u32;
            let y = c + key;
            let k = unsafe {char::from_u32_unchecked(y)};
            print!("{k}");
        }else{
            print!("{n}");
        }
    }
    let duration = start.elapsed();
    println!("\n\nTime elapsed in Caesar Cipher converter is: {:?}", duration); 

}
/*A a		

B b		
C c	
D d		
E e		
F f
G g		
H h		
I i		
J j		
K k		
L l		
M m		
N n		
O o		
P p		
Q q		
R r	
S s		
T t		
U u		
V v		
W w		
X x		
Y y		
Z z	*/

 