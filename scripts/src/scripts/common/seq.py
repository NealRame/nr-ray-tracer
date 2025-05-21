def seq(start: float, stop: float, step: float):
    v = start
    while v <= stop:
        yield v
        v += step
