N = int(input())
S = []

for _ in range(N):
    S.append(input())

S_list = list(map(list, S))
S_list = [list(map(int, l)) for l in S_list]
S_list = tuple(S_list)

mins = []
for obj in range(10):
    ans = -1
    pushed = [0 for _ in range(N)]
    finished = False
    while True:
        for s in zip(*S_list):
            ans += 1
            indices = [i for i, x in enumerate(s) if x == obj]
            for ind in indices:
                if pushed[ind] == 0:
                    pushed[ind] = 1
                    break
            if 0 not in pushed:
                finished = True
                break
        if finished:
            break
    mins.append(ans)

print(min(mins))
