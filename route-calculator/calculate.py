from typing import List


def tokenize(exp: str) -> List[str]:
    def is_part_of_num(s: str) -> bool:
        return s.isdigit() or s == "."

    tokens = []
    last_pred = None
    for c in exp:
        pred = is_part_of_num(c)
        if last_pred is None or pred != last_pred:
            tokens.append("")
        last_pred = pred
        tokens[-1] += c

    return tokens


def calculate(exp: str) -> str:
    OPERATIONS = {
            "+": lambda a, b: a + b,
            "-": lambda a, b: a - b,
            "*": lambda a, b: a * b,
            "$": lambda a, b: a / b
    }
    OPERATIONS_BY_PRIORITY = ["$", "*", "-", "+"]

    tokens = tokenize(exp)

    for i, t in enumerate(tokens):
        if t in OPERATIONS.keys():
            continue
        else:
            try:
                tokens[i] = float(t)
            except ValueError:
                return "400: Bad request"

    def calc(single_exp: List[str]) -> float:
        left = single_exp[0]
        op = OPERATIONS[single_exp[1]]
        right = single_exp[2]
        return op(left, right)

    while len(tokens) > 1:
        present_ops = filter(lambda o: o in tokens, OPERATIONS_BY_PRIORITY)
        op = next(present_ops)
        op_idx = tokens.index(op)
        single_exp_slice = slice(op_idx - 1, op_idx + 2)
        res = calc(tokens[single_exp_slice])
        del tokens[single_exp_slice]
        tokens.insert(single_exp_slice.start, res)

    return f"{tokens[0]:g}"
