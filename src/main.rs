use chrono::{Local};

const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];


fn main() {
    print!("\x1b[2J");
    print!("\x1b[?25l");

    loop{
        println!("");
        //obtendo el objeto hora
        let t = Local::now();
    
        //convierto el formato a string
        let time = t.format("%H:%M:%S").to_string();
        
        //saco un vector, en usize.
        let valor = format(time);

        //muestro el valor. paso el vector con valor.
        /*
        let mut valor:usize = 1;
        let prueba: Vec<usize> = [valor,valor *5,valor+3].to_vec(); 
        display(prueba);
        */
        display(valor);
        //demora.
        std::thread::sleep(std::time::Duration::from_millis(999));
        
        //limpia pantalla
        println!("\x1b[7A");

    };

}


fn display(hora:Vec<usize>){
    let horario = hora;

    for row in &DIGITS {
        for i in 0..=horario.len()-1{
            print!("{}",row[horario[i]]);
        };
            println!("");
    };
}



pub fn format(time:String) -> Vec<usize> {
    
    let mut vec = Vec::new();

    for i in time.chars(){
        let cifra = match i {
            '0'..='9' => i as usize - '0' as usize,
            ':' => 10,
            _ => 10, //condicion para que match funcione sino no anda.
        };
        vec.push(cifra);
    };
    return vec;
}