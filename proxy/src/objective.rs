trait Operation {
    fn touch(&mut self) -> ();
}

#[derive(Debug)]
struct BigBlob {
    payload: String,
}

impl BigBlob {
    fn new() -> Self {
        println!("BigBlob created");
        BigBlob { payload: "Me be the BigBlob!".to_string() }
    }
}


impl Operation for BigBlob {
    fn touch(&mut self) -> () {
        println!("BigBlob {:?} touched", self);
    }
}


struct BigBlobProxy {
    target: Option<Box<Operation>>,
}

impl BigBlobProxy {
    fn new() -> Self {
        println!("BigBlobProxy created");
        BigBlobProxy { target: None }
    }
}


impl Operation for BigBlobProxy {
    fn touch(&mut self) -> () {
        match self.target {
            Some(ref mut t) => {
                println!("Performing proxied touch");
                t.touch();
            }
            None => {
                let mut bb = BigBlob::new();
                bb.touch();
                self.target = Some(Box::new(bb));
            }
        }
    }
}

//* http://pl.wikipedia.org/wiki/Pe%C5%82nomocnik_%28wzorzec_projektowy%29
//* Rodzaje i zastosowanie
//*
//* Istnieją cztery rodzaje tego wzorca, które jednocześnie definiują sytuacje, w
//* których może zostać użyty
//*
//* wirtualny – przechowuje obiekty, których utworzenie jest kosztowne; tworzy je
//* na żądanie
//*
//* ochraniający – kontroluje dostęp do obiektu sprawdzając, czy obiekt
//* wywołujący ma odpowiednie prawa do obiektu wywoływanego
//*
//* zdalny – czasami nazywany ambasadorem; reprezentuje obiekty znajdujące się w
//* innej przestrzeni adresowej
//*
//* sprytne odwołanie – czasami nazywany sprytnym wskaźnikiem; pozwala na
//* wykonanie dodatkowych akcji podczas dostępu do obiektu, takich jak: zliczanie
//* referencji do obiektu czy ładowanie obiektu do pamięci
pub fn run() {
    println!("-------------------- {} --------------------", file!());
    //virtual proxy
    let mut r = BigBlobProxy::new();
    r.touch();
    r.touch();
    r.touch();
}