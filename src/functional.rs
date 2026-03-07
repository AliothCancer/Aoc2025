// 1. Tratto core: espone l'esecuzione e la Fluent Interface
pub trait Callable<Args> {
    type Output;
    
    // Il corpo della funzione come metodo del trait
    fn call(&self, args: Args) -> Self::Output;

    // Concatena questa funzione con un'altra. 
    // L'output di 'self' deve coincidere con l'input di 'next'
    fn chain<G, Out>(self, next: G) -> Chain<Self, G>
    where
        Self: Sized,
        G: Callable<(Self::Output,), Output = Out>,
    {
        Chain { f: self, g: next }
    }

    // Esegue il currying del primo argomento (Partial Application).
    // Trasforma una funzione (T1, T2) in una funzione (T2,)
    fn curry<T1, T2>(self, arg: T1) -> Curry<Self, T1>
    where
        Self: Sized + Callable<(T1, T2)>,
        T1: Clone,
    {
        Curry { f: self, arg }
    }
}

// 2. Struct base per wrappare le funzioni o le closure primitive
pub struct FuncObj<F> {
    pub body: F,
}

impl<F> FuncObj<F> {
    pub fn new(body: F) -> Self {
        Self { body }
    }
}

// 3. Strutture intermedie che rappresentano le operazioni componibili
pub struct Chain<F, G> {
    f: F,
    g: G,
}

pub struct Curry<F, T> {
    f: F,
    arg: T,
}

// 4. Implementazioni di Callable per il wrapper base (1 e 2 argomenti come tuple)
impl<F, T1, R> Callable<(T1,)> for FuncObj<F>
where
    F: Fn(T1) -> R,
{
    type Output = R;
    fn call(&self, args: (T1,)) -> Self::Output {
        (self.body)(args.0)
    }
}

impl<F, T1, T2, R> Callable<(T1, T2)> for FuncObj<F>
where
    F: Fn(T1, T2) -> R,
{
    type Output = R;
    fn call(&self, args: (T1, T2)) -> Self::Output {
        (self.body)(args.0, args.1)
    }
}

// 5. Implementazione del Chaining
impl<F, G, Args, Intermediate, Out> Callable<Args> for Chain<F, G>
where
    // 'F' prende Args e ritorna Intermediate
    F: Callable<Args, Output = Intermediate>,
    // 'G' prende Intermediate (in tupla) e ritorna Out
    G: Callable<(Intermediate,), Output = Out>,
{
    type Output = Out;
    fn call(&self, args: Args) -> Self::Output {
        let intermediate = self.f.call(args);
        self.g.call((intermediate,))
    }
}

// 6. Implementazione del Currying
impl<F, T1, T2, Out> Callable<(T2,)> for Curry<F, T1>
where
    // L'oggetto originale richiedeva (T1, T2)
    F: Callable<(T1, T2), Output = Out>,
    T1: Clone,
{
    type Output = Out;
    fn call(&self, args: (T2,)) -> Self::Output {
        // Combiniamo l'argomento salvato (T1) con quello nuovo fornito (T2)
        self.f.call((self.arg.clone(), args.0))
    }
}

// ==========================================
// ESEMPIO DI UTILIZZO (COMPTIME CHECKED)
// ==========================================
fn test_func() {
    // Definizione di funzioni base incapsulate
    let somma = FuncObj::new(|a: i32, b: i32| a + b);
    let moltiplica = FuncObj::new(|x: i32| x * 10);
    let formatta = FuncObj::new(|x: i32| format!("Risultato computato a compile-time: {}", x));

    // Creazione della pipeline tramite Fluent Interface
    // I tipi sono interamente risolti dal compilatore
    let pipeline = somma
        .curry(5)            // Trasforma (i32, i32)->i32 in (i32,)->i32 fissando a=5
        .chain(moltiplica)   // Assorbe il risultato: f(x) * 10
        .chain(formatta);    // Assorbe il risultato e genera una String

    
    // Esecuzione passando l'unico argomento rimanente (il parametro 'b' di somma)
    // Pipeline: (5 + 3) * 10 = 80 => formatterà il numero 80.
    let risultato = pipeline.call((3,));

    println!("{}", risultato); 
}