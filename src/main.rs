fn main() {
    exercicio_ownership();
}

fn grita(string: String) {
    println!("{}", string.to_uppercase());
}

fn grita_ref(string: &String) {
    println!("{}", string.to_uppercase());
}

fn grita_mas_grita_baixo(string: &mut String) {
    println!("original {}", string);
    string.push('.');
    string.push('.');
    string.push('.');
    println!("mutada {}", string.to_uppercase());
    println!("original {}", string);
}

fn exercicio_ownership() {
    let mut t = "tchau".to_string();
    //grita_ref(&t);
    //grita_ref(&t);

    grita_mas_grita_baixo(&mut t);
    grita_ref(&t);
}

fn exercicio_loop() {
    let mut contagem = 1;

    loop {
        canta(contagem);
        contagem += 1;
        if contagem == 10 {
            canta(contagem);
            break;
        }
    }

    contagem = 1;
    while contagem <= 10 {
        canta(contagem);
        contagem += 1;
    }

    for contagem in 1..11 {
        canta(contagem);
    }

    let vetor = vec![1,2,3,4,5,6,7,8,9,10];
    for n in vetor {
        canta(n);
    }

    let mut vetor2 = Vec::new();
    vetor2.push(1);
    vetor2.push(2);
    vetor2.push(3);
    vetor2.push(4);
    vetor2.push(5);
    vetor2.push(6);
    vetor2.push(7);
    vetor2.push(8);
    vetor2.push(9);
    vetor2.push(10);
    for n in vetor2 {
        canta(n);
    }
}

fn canta(n: i32) {
    print!("{}", n);
    if !(n % 3 == 0) {
        if !(n % 10 == 0) {
            print!(", ");
        }
    } else {
        print!(" indiozinhos\n");
    }

    if n % 10 == 0 {
        print!(" no pequeno bote\n\n");
    }
}

fn exercicio_if() {
    println!("Hello, world!");

    let _um = 1;
    let _dois : i32 = 2;
    let _mentira : bool = false;

    // if
    let n = 10;
    if n % 2 == 0 {
        println!("par");
    } else {
        println!("ímpar");
    }

    let mensagem = if n % 2 == 0 {
        "par"
    } else {
        "ímpar"
    };
    println!("O número é {}", mensagem);

    let resultado = par(2);
    println!("O número é {}", resultado);

    let resultado2 = impar(2);
    println!("O valor de n é {} e ele é {}", n, resultado2);
}

fn par(n: i32) -> &'static str {
    if n % 2 == 0 {
        "par"
    } else {
        "ímpar"
    }
}

fn impar(n: i32) -> String {
    if n % 2 == 1 {
        String::from("par")
    } else {
        "ímpar".to_string()
    }
}
