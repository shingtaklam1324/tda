use tda::simplex::Simplex;

#[test]
fn boundary_coeff_2_0() {
    let s: Simplex<i32> = vec![0, 1].into();
    let t: Simplex<i32> = vec![0].into();
    assert_eq!(s.boundary_coeff::<i32>(&t), -1);
}

#[test]
fn boundary_coeff_2_1() {
    let s: Simplex<i32> = vec![0, 1].into();
    let t: Simplex<i32> = vec![1].into();
    assert_eq!(s.boundary_coeff::<i32>(&t), 1);
}

#[test]
fn boundary_coeff_3_0() {
    let s: Simplex<i32> = vec![0, 1, 2].into();
    let t: Simplex<i32> = vec![0, 1].into();
    assert_eq!(s.boundary_coeff::<i32>(&t), 1);
}

#[test]
fn boundary_coeff_3_1() {
    let s: Simplex<i32> = vec![0, 1, 2].into();
    let t: Simplex<i32> = vec![0, 2].into();
    assert_eq!(s.boundary_coeff::<i32>(&t), -1);
}

#[test]
fn boundary_coeff_3_2() {
    let s: Simplex<i32> = vec![0, 1, 2].into();
    let t: Simplex<i32> = vec![1, 2].into();
    assert_eq!(s.boundary_coeff::<i32>(&t), 1);
}
