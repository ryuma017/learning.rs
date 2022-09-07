from numpy import array
from create_gif import gol_gif

status = array([
    [
        [0, 0, 0, 0, 0, 0],
        [0, 1, 1, 0, 0, 0],
        [0, 1, 1, 0, 0, 0],
        [0, 0, 0, 1, 1, 0],
        [0, 0, 0, 1, 1, 0],
        [0, 0, 0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0, 0, 0],
        [0, 1, 1, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 1, 1, 0],
        [0, 0, 0, 0, 0, 0],
    ],
])

x50_status = []
for state in status:
    x50_state = state.repeat(50, axis=0).repeat(50, axis=1).flatten().tolist()
    x50_status.append(x50_state)

# print(x50_status)
gol_gif(6 * 50, 6 * 50, x50_status, 'testx50.gif')
