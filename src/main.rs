use rdkit_rs::ROMol;

fn main() {
    let mol = ROMol::from_smiles("CCO");
    let inchi = mol.to_inchi_key();
    println!("Hello inchi: {inchi}!")
}