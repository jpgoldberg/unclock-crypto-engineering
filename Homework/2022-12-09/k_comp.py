import datetime

sec_per_year = 60 * 60 * 24 * 365.4


def k_comp(ell: int, t: float) -> float:
    return (t * (2 ** -(ell/2)))


for ell, t in [(8, 6.53e-6), (16, 102e-6), (24, 1.66e-3), (32, 26.0e-3), (48, 13.1)]:
    k = k_comp(ell, t)
    print(f"k({ell}, {t} = {k:.2e}")


def t_comp(ell: int, k: float) -> float:
    t = k * (2.0 ** (ell/2))
    return t/sec_per_year


k = 4e-7

print(t_comp(256, k))
