use super::simplicial_complex::SimplicialComplex;

pub struct Filtration<T> {
    data: Vec<SimplicialComplex<T>>
}

impl<T> Filtration<T> {
    pub fn new(data: Vec<SimplicialComplex<T>>) -> Self {
        Filtration { data }
    }
}
