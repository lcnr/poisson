use ::PoissonDisk;

use test::{test_with_seeds, test_with_seeds_prefill};

use rand::{SeedableRng, XorShiftRng};

use na::Vec3 as naVec3;
pub type Vec3 = naVec3<f64>;

#[test]
fn test_3d_max_radius_normal() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds::<Vec3>(radius, 1600, false);
}

#[test]
fn test_3d_2nd_max_radius_normal() {
    let radius = 2f64.sqrt() / 2f64 / 2f64;
    test_with_seeds::<Vec3>(radius, 800, false);
}

#[test]
fn test_3d_4th_of_max_radius_normal() {
    let radius = 2f64.sqrt() / 2f64 / 4f64;
    test_with_seeds::<Vec3>(radius, 400, false);
}

#[test]
fn test_3d_8th_of_max_radius_normal() {
    let radius = 2f64.sqrt() / 2f64 / 8f64;
    test_with_seeds::<Vec3>(radius, 200, false);
}

#[test]
fn test_3d_16th_of_max_radius_normal() {
    let radius = 2f64.sqrt() / 2f64 / 16f64;
    test_with_seeds::<Vec3>(radius, 100, false);
}

#[test]
fn test_3d_max_radius_perioditic() {
    let radius = 0.499999999;
    test_with_seeds::<Vec3>(radius, 400, true);
}

#[test]
fn test_3d_2nd_max_radius_perioditic() {
    let radius = 2f64.sqrt() / 2f64 / 2f64;
    test_with_seeds::<Vec3>(radius, 200, true);
}

#[test]
fn test_3d_4th_of_max_radius_perioditic() {
    let radius = 2f64.sqrt() / 2f64 / 4f64;
    test_with_seeds::<Vec3>(radius, 100, true);
}

#[test]
fn test_3d_8th_of_max_radius_perioditic() {
    let radius = 2f64.sqrt() / 2f64 / 8f64;
    test_with_seeds::<Vec3>(radius, 50, true);
}

#[test]
fn test_3d_16th_of_max_radius_perioditic() {
    let radius = 2f64.sqrt() / 2f64 / 16f64;
    test_with_seeds::<Vec3>(radius, 25, true);
}

#[test]
fn test_3d_2th_of_max_radius_prefilled_with_max_normal() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds_prefill::<Vec3, _>(radius / 2f64, 800, false, &mut |ref mut v, i| {
            let rand = XorShiftRng::from_seed([i * 2 + 1, i * 1 + 1, i + 1, 2]);
            let mut poisson = PoissonDisk::new(rand, radius);
            poisson.create(v);
        });
}

#[test]
fn test_3d_4th_of_max_radius_prefilled_with_2rd_of_max_normal() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds_prefill::<Vec3, _>(radius / 4f64, 400, false, &mut |ref mut v, i| {
            let rand = XorShiftRng::from_seed([i * 2 + 1, i * 1 + 1, i + 1, 2]);
            let mut poisson = PoissonDisk::new(rand, radius / 2f64);
            poisson.create(v);
        });
}

#[test]
fn test_3d_8th_of_max_radius_prefilled_with_4th_of_max_normal() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds_prefill::<Vec3, _>(radius / 8f64, 200, false, &mut |ref mut v, i| {
            let rand = XorShiftRng::from_seed([i * 2 + 1, i * 1 + 1, i + 1, 2]);
            let mut poisson = PoissonDisk::new(rand, radius / 4f64);
            poisson.create(v);
        });
}

#[test]
fn test_3d_16th_of_max_radius_prefilled_with_8th_of_max_normal() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds_prefill::<Vec3, _>(radius / 16f64, 100, false, &mut |ref mut v, i| {
            let rand = XorShiftRng::from_seed([i * 2 + 1, i * 1 + 1, i + 1, 2]);
            let mut poisson = PoissonDisk::new(rand, radius / 8f64);
            poisson.create(v);
        });
}

#[test]
fn test_3d_2th_of_max_radius_prefilled_with_max_radius_perioditic() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds_prefill::<Vec3, _>(radius / 2f64, 200, true, &mut |ref mut v, i| {
            let rand = XorShiftRng::from_seed([i * 2 + 1, i * 1 + 1, i + 1, 2]);
            let mut poisson = PoissonDisk::perioditic(rand, 0.499999999);
            poisson.create(v);
        });
}

#[test]
fn test_3d_4th_of_max_radius_prefilled_with_2rd_of_max_radius_perioditic() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds_prefill::<Vec3, _>(radius / 4f64, 100, true, &mut |ref mut v, i| {
            let rand = XorShiftRng::from_seed([i * 2 + 1, i * 1 + 1, i + 1, 2]);
            let mut poisson = PoissonDisk::perioditic(rand, radius / 2f64);
            poisson.create(v);
        });
}

#[test]
fn test_3d_8th_of_max_radius_prefilled_with_4th_of_max_radius_perioditic() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds_prefill::<Vec3, _>(radius / 8f64, 50, true, &mut |ref mut v, i| {
            let rand = XorShiftRng::from_seed([i * 2 + 1, i * 1 + 1, i + 1, 2]);
            let mut poisson = PoissonDisk::perioditic(rand, radius / 4f64);
            poisson.create(v);
        });
}

#[test]
fn test_3d_16th_of_max_radius_prefilled_with_8th_of_max_radius_perioditic() {
    let radius = 2f64.sqrt() / 2f64;
    test_with_seeds_prefill::<Vec3, _>(radius / 16f64, 25, true, &mut |ref mut v, i| {
            let rand = XorShiftRng::from_seed([i * 2 + 1, i * 1 + 1, i + 1, 2]);
            let mut poisson = PoissonDisk::perioditic(rand, radius / 8f64);
            poisson.create(v);
        });
}