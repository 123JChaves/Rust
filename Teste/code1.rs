//Define uma struct base chamada "Pessoa"
struct Pessoa {
    nome: String,
    idade: u32,
}

impl Pessoa {
    fn new(nome: String, idade: u32) -> Pessoa {
        Pessoa {nome, idade}
    }

    fn apresentar(&self) {
        println!("Olá, meu nome é {} e eu tenho {} anos.", self.nome, self.idade);
    }
}

//Define uma struct "Estudante" que herda de "Pessoa"
struct Estudante {
    pessoa: Pessoa,
    curso: String,
}

impl Estudante {
    fn new(nome: String, idade: u32, curso: String) -> Estudante {
        Estudante {
            pessoa: Pessoa::new(nome, idade),
            curso,
        }
    }

    fn apresentar(&self) {
        self.pessoa.apresentar();
        println!("Eu, {}, estou estudando {}.", self.pessoa.nome, self.curso);
    }
}

fn main() {
    let estudante = Estudante::new("Joao".to_string(), 20, "Ciências da Computação".to_string());
    estudante.apresentar();
}