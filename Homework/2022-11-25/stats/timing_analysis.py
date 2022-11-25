from typing import Optional
import cbor2
import scipy.stats

class CritData:
    """Criterion measuerment data"""

    def __init__(self, criterion_data: dict) -> None:

        # Should run checks on the soundness of input (at some point)
       self.data: dict = criterion_data

class Comparison:
    def __init__(self, file1: str, file2: str, alternative="two-tailed"):
        valid_alternatives = ["two-tailed", "less", "greater"]
        if alternative not in valid_alternatives:
            raise ValueError(f'alternative must be one of {valid_alternatives}')
        
        self.alt = alternative # will pass to t-test

        with open(file1, "rb") as f:
            f1_data = cbor2.decoder.load(f)

        self.ds1 = CritData(f1_data)

        with open(file2, "rb") as f:
            f2_data = cbor2.decoder.load(f)

        self.ds2 = CritData(f2_data)

        # Check whether these can be treated as related samples
        self.related: bool
        if self.ds1.data["iterations"] == self.ds2.data["iterations"]:
            self.related = True
        else:
            self.related = False

        self.t, self.p = self.ttest()


    def ttest(self) -> tuple[float, float]:
        t: float
        p: float
        if self.related:
            (t, p) = scipy.stats.ttest_rel(self.ds1.data["values"],
                                self.ds2.data["values"],
                                alternative=self.alt)
        else:
            (t, p) = scipy.stats.ttest_ind(self.ds1.data["avg_values"],
                                self.ds2.data["avg_values"],
                                alternative=self.alt)
        
        return t, p

EARLY_PATH = './32early.cbor'
LATE_PATH = './32late.cbor'
def main() -> None:

    comp32 = Comparison(EARLY_PATH, LATE_PATH, alternative="less")

    print(f'With 32 character alphabet: t = {comp32.t:.3f}; p = {comp32.p:.3f}')

    comp_mod = Comparison("mod_add_early.cbor", "mod_add_late.cbor", alternative="less")

    print(f'Mod 26 addition: t = {comp_mod.t:.3f}; p = {comp_mod.p:.3f}')

    comp26 = Comparison("abc26-early.cbor", "abc26-late.cbor", alternative="less")

    print(f'ABC26 encryption: t = {comp26.t:.3f}; p = {comp26.p:.3f}')



if __name__ == "__main__":
    main()

