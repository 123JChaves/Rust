//Define uma trait "Pessoa" com um método apresentar
trait Pessoa {
    fn apresentar(&self);
}

//Define uma struct "Estudante" que implementa a trait "Pessoa"
struct Estudante {
    nome: String,
    idade: u32,
    curso: String,
}

impl Pessoa for Estudante {
    fn apresentar(&self) {
        println!("Olá, meu nome é {}. Tenho {} anos e estudo {}.", self.nome, self.idade, self.curso);
    }
}

//Define uma struct "Professor" que implementa a trait "Pessoa"
struct Professor {
    nome: String,
    idade: u32,
    disciplina: String,
}

impl Pessoa for Professor {
    fn apresentar(&self) {
        println!("Olá, meu nome é {}. Tenho {} anos e sou professor(@) de {}.", self.nome, self.idade, self.disciplina);
    }

}

fn main() {
    let estudante = Estudante {
        nome: "João".to_string(),
        idade: 18,
        curso: "TADS".to_string(),   
    };

    let professor = Professor {
        nome: "Flávia".to_string(),
        idade: 25,
        disciplina: "Programação".to_string(),
    };

    estudante.apresentar();
    professor.apresentar();
}