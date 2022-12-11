def k_comp(ell: int, t: float) -> float:
    return (t * (2 ** -(ell/2)))


for ell, t in [(8, 6.5e-6), (16, 102e-6), (24, 1.7e-3), (32, 23e-3), (48, 12.4)]:
    k = k_comp(ell, t)
    print(f"k({ell}, {t} = {k:.2e}")
