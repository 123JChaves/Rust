//Define uma trait "Animal" com um método 'falar'
trait Animal {
    fn falar(&self);
}

//Define uma struct "Cachorro" que implementa a trait "Animal"

struct Cachorro;

impl Animal for Cachorro {
    fn falar(&self) {
        println!("Au, au!");
    }
}

struct Gato;

//Define uma struct "Gato" que implementa a trait "Animal"
impl Animal for Gato {
    fn falar(&self) {
        println!("Miau!");
    }
}

fn main() {
    let cachorro = Cachorro;
    let gato = Gato;

    cachorro.falar(); //Imprime "Au, au!"
    gato.falar(); //Imprime "Miau!"
}