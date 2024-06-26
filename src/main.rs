/*
 * Escribe una función que reciba dos palabras (String) y retorne
 * verdadero o falso (Bool) según sean o no anagramas.
 * - Un Anagrama consiste en formar una palabra reordenando TODAS
 *   las letras de otra palabra inicial.
 * - NO hace falta comprobar que ambas palabras existan.
 * - Dos palabras exactamente iguales no son anagrama.
 */

mod easy {
    pub mod c1_fizz_buzz;
    pub mod c5_polygon_area;
    pub mod c7_reversing_strings;
}

mod medium {
    pub mod c2_anagram;
    pub mod c4_prime_number;
}

mod hard {
    pub mod c3_fibonacci;
    pub mod c6_aspect_ratio;
}

fn main() {
    // #1 EL FAMOSO "FIZZ BUZZ"
    // easy::c1_fizz_buzz::fizz_buzz();

    // #2 ¿ES UN ANAGRAMA?
    // println!("{}", medium::c2_anagram::is_anagram("hello", "world"));
    // println!("{}", medium::c2_anagram::is_anagram("cinema", "iceman"));

    // #3 ¿ES UN NÚMERO PRIMO?
    // medium::c4_prime_number::prime_number();

    // #4 ÁREA DE UN POLÍGONO
    // let triangle = easy::c5_polygon_area::Polygon::Triangle {
    //     base: 5.0,
    //     height: 10.0,
    // };
    // println!("{}", easy::c5_polygon_area::polygon_area(&triangle));

    // #5 LA SUCESIÓN DE FIBONACCI
    // hard::c3_fibonacci::fibonacci();

    // #6 ASPECT RATIO DE UNA IMAGEN
    hard::c6_aspect_ratio::aspect_ratio();

    // #7 INVIRTIENDO CADENAS
    // println!("{}", easy::c7_reversing_strings::reverse_string("Hola mundo"));
}
