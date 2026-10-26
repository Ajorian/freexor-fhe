
  pub  fn log2_erfc(x: f64) -> f64 //erfc is error function of Gaussian dist., working in logs avoids underflow for tiny failure probabilities.
{
    let mut t = x;
    for k in (1..=80).rev() {
        t = x + (k as f64 / 2.0) / t;}
    let ln = -x * x - 0.5 * std::f64::consts::PI.ln() - t.ln();
    ln / std::f64::consts::LN_2
}

pub fn z_for_alpha(alpha: f64) -> f64  // Smallest l with P_fail = erfc(l/sqrt2) <= 2^-alpha (two-sided Gaussian tail).
{
    let (mut lo, mut hi) = (0.8_f64, 40.0_f64);
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        let log2_p = log2_erfc(mid / std::f64::consts::SQRT_2);
        if log2_p > -alpha { lo = mid; } else { hi = mid; }   // p too big -> need larger l
    }
    hi
}