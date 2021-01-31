use structopt::StructOpt;
mod hashdb;
mod vecdb;
use hashdb::HashDB;
use vecdb::VecDB;


fn main() {
    let args = Cli::from_args();
    return match args {
        Cli::HashAdd { k, v } => hashadd(k, v),
        Cli::HashGet { k } => hashget(k),
        Cli::HashDump => hashdump(),
        Cli::HashWipe => hashwipe(),
        Cli::ListAdd {v} => vecadd(v),
        Cli::ListGet { k } => vecget(k),
        Cli::ListDump => vecdump(),
        Cli::ListWipe => vecwipe()
    }
}


#[derive(StructOpt, Debug)]
enum Cli {
    #[structopt(name = "add", about = "Add a key value pair", help = "USAGE: add <KEY> <VALUE")]
    HashAdd {
        k: String,
        v: String
    },
    #[structopt(name = "get", about = "Get a value from given key", help = "USAGE: get <KEY>")]
    HashGet {
        k: String,
    },
    #[structopt(name = "dump", about = "Dump all from the database", help = "USAGE: dump")]
    HashDump,
    #[structopt(name = "wipe", about = "Wipe all from the database", help = "USAGE: wipe")]
    HashWipe,
    #[structopt(name = "listadd", about = "Add to list", help = "USAGE: listadd")]
    ListAdd {
        v: String
    },
    #[structopt(name = "listget", about = "Get a value from given key", help = "USAGE: listget <KEY>")]
    ListGet {
        k: String,
    },
    #[structopt(name = "listdump", about = "Dump all from the database", help = "USAGE: listdump")]
    ListDump,
    #[structopt(name = "listwipe", about = "Wipe all from the database", help = "USAGE: listwipe")]
    ListWipe
}

fn hashadd(k: String, v: String) {
    println!("{}->{}", k, v);
    let mut database = HashDB::new().expect("Reading db failed");
    database.insert(k, v);
    database.write();

}

fn hashget(k: String) {
    let database = HashDB::new().expect("Reading db failed");
    database.get(k);
}

fn hashdump() {
    let database = HashDB::new().expect("Reading db failed");
    println!("START\n");
    database.dump();
    println!("END");
}

fn hashwipe() {
    let database = HashDB::new().expect("Reading db failed");
    database.wipe();
    println!("WIPED DB");
}

fn vecadd(v: String) {
    println!("adding {}", v);
    let mut database = VecDB::new().expect("Reading db failed");
    database.insert(v);
    database.write();

}

fn vecget(k: String) {
    let database = VecDB::new().expect("Reading db failed");
    database.get(k);
}

fn vecdump() {
    let database = VecDB::new().expect("Reading db failed");
    println!("START\n");
    database.dump();
    println!("END");
}

fn vecwipe() {
    let database = VecDB::new().expect("Reading db failed");
    database.wipe();
    println!("WIPED DB");
}