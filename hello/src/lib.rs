
// aqui esta la lib con tus modulos y funciones pubs
// para que el binario entienda la libreria, debes agregarlo en el Cargo.toml de la carpeta del binario "my_app/Cargo.toml"

pub fn mg_add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = mg_add(2, 2);
        assert_eq!(result, 4);
    }
}
