Rust - Amazing Maze
Table des matières
1. Objectif
2. L'application
   2.1. Définition du labyrinthe
   2.2. Exploration
   2.3. Variante
3. Passage à Rust
   3.1. Définition et création du labyrinthe
   3.2. Ajout de la méthode d'exploration
4. Version intermédiaire
5. Version concurrente
   1 Objectif
   L'exploration d'un labyrinthe simple va nous permettre de voir comment combiner types algébriques et pointeurs intelligents avant, dans un second temps, de gérer la concurrence.

2 L'application
On considère des labyrinthes qui sont des DAG binaires. En voici un exemple, dont l'entrée (la racine du DAG) est étiquetée 0. Dans un premier temps, on ne considère pas de sortie et il va s'agir d'explorer un tel labyrinthe de gauche à droite, en profondeur d'abord, l'idée étant de ne pas revisiter les parties déjà explorées.

    0
/ \
1   6
/ \ / \
2   3   7
/ \ / \
4   5   8
Un labyrinthe est une branche ou une feuille. Une branche est constituée d'un nœud étiqueté par une chaîne de caractères et de deux labyrinthes, à gauche et à droite, d'une étiquette (une chaîne de caractères) et d'un niveau d'exploration : exploré ou pas. Une feuille est constituée d'un simple nœud.

2.1 Définition du labyrinthe
Voici une définition du labyrinthe en Scala (on utilise l'organisation hybride, qui va faciliter le passage à Rust) :

enum Exploration :
case Explored, UnExplored

enum Maze :
case Branch(label: String, left: Maze, right: Maze, var status: Exploration = UnExplored)
case Leaf(label: String)

Construction de l'exemple :

val leaf2 = Leaf("2")
val leaf4 = Leaf("4")
val leaf5 = Leaf("5")
val leaf8 = Leaf("8")
val branch3 = Branch("3", leaf4, leaf5)
val branch1 = Branch("1", leaf2, branch3)
val branch7 = Branch("7", leaf5, leaf8)
val branch6 = Branch("6", branch3, branch7)
val branch0 = Branch("0", branch1, branch6)
2.2 Exploration
À la base, lorsqu'on rencontre une branche déjà explorée, ou une feuille, on ne fait rien, sinon on dit la branche explorée (à ce stade, seule sa racine l'est mais si on se retrouve ultérieurement au même point toute la branche aura été explorée) et on explore son labyrinthe gauche et son labyrinthe droit.

À cette exploration, on ajoute un enregistrement des étiquettes rencontrées, qui fournit une trace du parcours effectué.

Si on définit la méthode d'exploration comme retournant la trace de l'exploration, on obtient :

enum Maze :
...
def explore: List[String] = this match
case branch@Branch(label, left, right, status) =>
status match
case UnExplored => branch.status = Explored; label :: left.explore ::: right.explore
case Explored => List(label)
case Leaf(label) => List(label)
attention.png Il peut être tentant de simplement écrire :

case Branch(label, left, right, status) =>
status match
case UnExplored => status = Explored; ...
Mais cette formulation ne marche pas. Il faut faire la distinction entre status, variable d'instance mutable d'un objet de type Maze.Branch et status, variable immutable introduite dans le pattern Branch(label, left, right, status). L'utilisation du pattern @ et de la notation pointée branch.status permet de faire cette distinction.

L'exploration du labyrinthe explore(branch0) rend la liste :

List(0, 1, 2, 3, 4, 5, 6, 3, 7, 5, 8)
Après l'exploration du nœud 6, l'exploration du labyrinthe gauche conduit à une nouvelle visite du nœud 3, visite qui n'est pas poursuivie, le nœud 3 ayant déjà été exploré. Cette visite est toutefois enregistrée dans la trace, qui enregistre tous les noeuds visités, pas seulement ceux qui n'ont pas encore été explorés.

attention.png Assurez-vous que vous comprenez bien cette trace.

2.3 Variante
Une variante consiste à utiliser une structure de données impérative, soit une instance de ListBuffer pour enregistrer la trace. La signature de la méthode d'exploration devient :

def explore(trace: ListBuffer[String]): Unit
Implémentez cette version et assurez-vous que vous obtenez bien la bonne trace d'exploration.

3 Passage à Rust
Il s'agit maintenant de traduire la version précédente en Rust.

3.1 Définition et création du labyrinthe
Dans un premier temps, il s'agit de définir la structure d'un labyrinthe et d'implémenter l'exemple. Utilisez pour cela la note sur les types algébriques et les pointeurs intelligents. Le fait que des branches soient partagées requiert l'utilisation de pointeur Rc.

Il faut aussi gérer la mutabilité des étiquettes ce qui fait appel au pointeur intelligent RefCell et au principe de mutabilité intérieure. En effet, le labyrinthe est globalement immutable, à l'exception du niveau d'exploration. Pour ne pas avoir à gérer des structures mutables partagées, il faut encapsuler le niveau d'exploration derrière un pointeur intelligent de type RefCell. Le compilateur ne va pas s'intéresser aux mutations affectant les données encapsulées.

Voici un petit exemple d'utilisation de RefCell :

use std::cell::RefCell;

fn main() {
let r = RefCell::new(0);
if *r.borrow() == 0 { r.replace(1); }
println!("{:?}", r);
}
On peut avoir l'impression qu'on risque de casser les règles de propriété, ce n'est pas le cas, mais les règles ne sont plus seulement garanties par le compilateur, il y a aussi des vérifications faites à l'exécution, qui peut mal se passer. Ainsi, thread 'main' panicked at 'already borrowed: BorrowMutError' indique une détection à l'exécution d'un double emprunt mutable, ce qui est interdit par les règles de propriété.

3.2 Ajout de la méthode d'exploration
La trace va être enregistrée dans un vecteur mutable qu'on va vouloir partager pour pouvoir l'afficher, ce qui donne un paramètre de type &mut Vec<String>. On manipule ce vecteur comme une pile avec la méthode push.

Tester le statut d'exploration par pattern matching est problématique. Il faut revenir à l'utilisation de conditionnelles (voir le pattern d'utilisation de RefCell ci-dessus).

hint.png Dériver le trait PartialEq permet d'utiliser == sur un type de données pour lequel l'égalité n'est pas définie a priori.

4 Version intermédiaire
En préparation d'une version concurrente, on va traiter l'exploration en gérant une pile de branches à explorer. Il y a maintenant trois niveaux d'exploration :

exploré : il n'y a rien à faire ;
partiellement exploré (la branche gauche a déjà été explorée) : la branche est considérée comme explorée et sa branche droite est explorée ;
non exploré : la branche est empilée, passe à partiellement explorée et sa branche gauche est explorée.
La pile peut facilement être implémentée à l'aide d'un vecteur et des méthodes push et pop et passée comme nouveau paramètre de la fonction d'exploration. Cette fonction d'exploration s'arrête maintenant dès qu'une feuille est atteinte.

L'exploration du labyrinthe consiste à créer une pile de travail work qui ne contient que l'entrée, puis, de manière répétitive, à dépiler une branche et l'explorer jusqu'à ce que la pile soit vide.

let mut work = vec![Rc::clone(&maze)];
let mut trace = vec![];
while work.len() != 0 {
let node = work.pop().expect("unexpected");
node.explore(Rc::clone(&node), &mut work, &mut trace);
println!("trace so far: {:?}", trace);
}
Notes :

le premier paramètre de explore, de type Rc<Maze> permet de disposer d'une version du labyrinthe encapsulée dans un pointeur intelligent Rc, au cas où il faudrait l'empiler ;
l'affichage dans la boucle visualise les chemins successivement ajoutés à la trace.
5 Version concurrente
L'idée est de maintenant exploiter la pile des branches à explorer de manière concurrente et donc d'explorer le labyrinthe de manière concurrente. Pour cela, il faut se partager le labyrinthe et la pile entre différents fils d'exécution. Partager des données se gère à l'aide d'une exclusion mutuelle, au travers de l'utilisation d'un nouveau pointeur intelligent Mutex. Comme RefCell, ce pointeur fait appel au principe de mutabilité intérieure.

Voici le schéma de base de l'utilisation de ce pointeur (cf https://doc.rust-lang.org/book/ch16-03-shared-state.html) :

use std::sync::Mutex;
use std::thread;

fn main() {
let counter = Arc::new(Mutex::new(0)); // creation of a mutex to share a counter
let mut handles = vec![]; // a vector of threads

    for _ in 0..10 {
        // creation of a counter reference to be owned by a thread
        let counter = Arc::clone(&counter);
        // creation of the thread, with a closure defining its activity
        let handle = thread::spawn(move || {         
            let mut num = counter.lock().unwrap(); // take the lock and access data

            *num += 1;                             // update data (and release the lock)
        });
        handles.push(handle); // the thread is recorded in the list of threads
    }

    for handle in handles {
        handle.join().unwrap(); // waiting for all the spawned threads to terminate
    }
    // checking the result in the main thread
    println!("Result: {}", *counter.lock().unwrap());
}
Pour partager le labyrinthe, une première chose à faire est de rendre le comptage de référence robuste vis-à-vis de la concurrence et donc de remplacer les pointer Rc par des pointeurs Arc.

Il faut aussi partager et protéger la pile, ce qui conduit à l'encapsuler dans un Mutex puis un Arc. Par contre, on peut supposer que chaque fil d'exécution possède sa propre trace.

Il est possible de favoriser la concurrence en utilisant la fonction thread::yield_now(). Cette fonction autorise l'ordonnanceur à choisir un autre fil d'exécution.

while work. ... .len() != 0 {
...
println!("worker {} explored nodes: {:?}", i, v);
// thread::sleep(Duration::new(0, 1000));
thread::yield_now();
}
Auteur: Jacques Noyé

Email: jacques.noye@imt-atlantique.fr

Created: 2022-05-11 Wed 13:34

Validate