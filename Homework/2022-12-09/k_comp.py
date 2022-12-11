import datetime


def k_comp(ell: int, t: float) -> float:
    return (t * (2 ** -(ell/2)))


for ell, t in [(8, 6.7e-6), (16, 102e-6), (24, 1.73e-3), (32, 27.9e-3), (48, 14.7)]:
    k = k_comp(ell, t)
    print(f"k({ell}, {t} = {k:.2e}")


def t_comp(ell: int, k: float) -> datetime.timedelta:
    t = k * (2 ** (ell/2))
    return datetime.timedelta(seconds=t)
