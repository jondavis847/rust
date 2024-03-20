mod butcher;

use std::ops::{Add, Mul};
use nalgebra::{SVector,Dim};

pub fn runge_kutta<Ty, Tp>(
    f: impl Fn(Ty, Ty, f64, Tp) -> Ty, // Function to solve
    y0: Ty,                            // Initial value
    t0: f64,                           // Initial time
    p: Tp,                             //ode function parameters
    mut dt: f64,                       // Initial step size
    mut t_end: f64,                    // End time
    order: usize,                      // Order of the Runge-Kutta
    tableau: butcher::Tableau,         // butcher tableau of RK coefficients
) -> (Vec<f64>, Vec<Ty>)
where
    Ty: Copy + Default + Mul<f64, Output = Ty>,
{
    let mut y = y0;
    let mut t = t0;
    let mut result = (vec![], vec![]);

    result.0.push(t);
    result.1.push(y);

    while t < t_end {
        if t + dt > t_end {
            dt = t_end - t;
        }

        let mut k = vec![Ty::default(); order];

        let dy = Ty::default();

        k[0] = f(dy,y,t,p) * dt ;

        for i in 1..order {
            let mut sum = Ty::default();
            for j in 0..i {
                sum = sum
                    + k[j]
                        * (T::from(1.0).unwrap()
                            / (T::from(2.0).unwrap() + T::from(j as f64).unwrap()));
            }
            k[i] = f(t + T::from(i as f64).unwrap() * dt, y + sum) * dt;
        }

        let mut sum = T::default();
        for i in 0..order {
            sum = sum
                + k[i]
                    * (T::from(1.0).unwrap()
                        / (T::from(2.0).unwrap() + T::from(i as f64).unwrap()));
        }

        y = y + sum;
        t = t + dt;

        result.push((t, y));
    }

    result
}
