use std::sync::mpsc;
use std::thread;


fn somme (tab: Vec<u32>) -> u32 {
    let mut s : u32 = 0;
    for a in 0..tab.len() {
        s += tab[a];
    }
    s
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let  t1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let t2 = vec![5, 8, 4];

    let tx1 = tx.clone();

    let handle =  thread::spawn(move || {        
            tx.send(somme(t1)).unwrap();           
    });     
      
    thread::spawn(move || {        
        tx1.send(somme(t2)).unwrap();
    });
    
    let mut i = 1;
    let mut sommetab = 0;
   for received in rx {   
      sommetab += received;
      println!("la somme du tableau{}: {}", i, received);
      i+=1;
   }    

    println!("la somme des 2 tableaux est: {}", sommetab);
    
    handle.join().unwrap();
}