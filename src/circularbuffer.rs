use std::mem;

struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    size: usize,
    head: usize,
    tail: usize,
    count: usize,
}

impl<T: std::fmt::Debug + Clone + PartialEq> CircularBuffer<T> {
    // Création d'un nouveau buffer circulaire
    fn new(size: usize) -> Self {
        assert!(size > 0, "La taille du buffer doit être positive.");
        Self {
            buffer: vec![None; size],
            size,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    // Ajout d'un élément au buffer
    fn push(&mut self, item: T) {
        if self.is_full() {
            // Si le buffer est plein, déplacer le "tail" pour écraser le plus ancien
            self.tail = (self.tail + 1) % self.size;
        } else {
            self.count += 1;
        }

        self.buffer[self.head] = Some(item); // Ajouter l'élément à "head"
        self.head = (self.head + 1) % self.size; // Avancer "head"
    }

    // Retrait de l'élément le plus ancien
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None // Rien à retirer si le buffer est vide
        } else {
            let item = self.buffer[self.tail].take(); // Retirer l'élément à "tail"
            self.tail = (self.tail + 1) % self.size; // Avancer "tail"
            self.count -= 1;
            item
        }
    }

    // Vérifie si le buffer est plein
    fn is_full(&self) -> bool {
        self.count == self.size
    }

    // Vérifie si le buffer est vide
    fn is_empty(&self) -> bool {
        self.count == 0
    }

    // Retourne la taille actuelle du buffer
    fn len(&self) -> usize {
        self.count
    }

    // Retourne la capacité totale du buffer
    fn capacity(&self) -> usize {
        self.size
    }

    // Réduit la capacité du buffer pour qu'elle corresponde à sa taille utilisée
    fn shrink_to_fit(&mut self) {
        if self.count < self.size {
            let mut new_buffer = Vec::with_capacity(self.count);
            for i in 0..self.count {
                new_buffer.push(self.buffer[(self.tail + i) % self.size].take());
            }
            self.buffer = new_buffer;
            self.size = self.count;
            self.head = self.count;
            self.tail = 0;
        }
    }

    // Affiche tous les éléments du buffer
    fn display(&self) {
        print!("Buffer: ");
        for i in 0..self.size {
            if let Some(val) = &self.buffer[i] {
                print!("{:?} ", val);
            } else {
                print!("_ "); // Indique une case vide
            }
        }
        println!();
    }

    // Redimensionne le buffer circulaire en conservant les éléments dans l'ordre
    fn resize(&mut self, new_size: usize) -> Result<(), String> {
        if new_size == 0 {
            return Err("La taille du buffer doit être supérieure à 0.".to_string());
        }
        
        let mut new_buffer = vec![None; new_size];
        for i in 0..self.count {
            new_buffer[i] = self.buffer[(self.tail + i) % self.size].take();
        }
        self.buffer = new_buffer;
        self.size = new_size;
        self.head = self.count;
        self.tail = 0;
        Ok(())
    }

    // Retourne une référence au prochain élément à être retiré sans le supprimer
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.buffer[self.tail].as_ref()
        }
    }

    // Vide complètement le buffer
    fn clear(&mut self) {
        self.buffer = vec![None; self.size];
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }

    // Vérifie si un élément est présent dans le buffer
    fn contains(&self, item: &T) -> bool {
        self.buffer.iter().any(|val| val.as_ref() == Some(item))
    }

    // Permet de traverser le buffer
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter().filter_map(|x| x.as_ref())
    }
}

fn main() {
    // Création d'un buffer circulaire de taille 5
    let mut buffer = CircularBuffer::new(5);

    // Ajout d'éléments au buffer
    buffer.push(10);
    buffer.push(20);
    buffer.push(30);
    buffer.display(); // Affiche : Buffer: 10 20 30 _ _

    buffer.push(40);
    buffer.push(50);
    buffer.display(); // Affiche : Buffer: 10 20 30 40 50

    buffer.push(60); // Écrase le plus ancien élément (10)
    buffer.display(); // Affiche : Buffer: 60 20 30 40 50

    // Retrait d'éléments
    let popped = buffer.pop();
    println!("Popped: {:?}", popped); // Affiche : Popped: Some(20)
    buffer.display(); // Affiche : Buffer: 60 _ 30 40 50

    buffer.push(70);
    buffer.display(); // Affiche : Buffer: 60 70 30 40 50

    // Utilisation de peek
    if let Some(peeked) = buffer.peek() {
        println!("Peeked: {:?}", peeked); // Affiche : Peeked: 30
    }

    // Vérification de contains
    println!("Contains 30: {}", buffer.contains(&30)); // Affiche : Contains 30: true
    println!("Contains 100: {}", buffer.contains(&100)); // Affiche : Contains 100: false

    // Affichage de la taille et capacité
    println!("Taille du buffer: {}", buffer.len()); // Affiche : Taille du buffer: 5
    println!("Capacité du buffer: {}", buffer.capacity()); // Affiche : Capacité du buffer: 5

    // Vider le buffer
    buffer.clear();
    buffer.display(); // Affiche : Buffer: _ _ _ _ _

    // Redimensionnement du buffer
    match buffer.resize(7) {
        Ok(()) => {
            println!("Redimensionnement réussi à 7...");
            buffer.push(80);
            buffer.push(90);
            buffer.display(); // Affiche : Buffer: 80 90 _ _ _ _ _
        }
        Err(err) => println!("Erreur de redimensionnement: {}", err),
    }
    
    // Traverser le buffer
    for val in buffer.iter() {
        println!("Iterated: {:?}", val);
    }

    // Réduire la capacité à la taille utilisée
    buffer.shrink_to_fit();
    println!("Capacité après shrink_to_fit: {}", buffer.capacity());
}

